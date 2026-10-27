use crate::error::{ChipdocError, Result};
use crate::model::{BBox, DocMetadata, SanityReport};
use crate::pdf_loader::{parse_page_range_expr, render_single_figure_crop};
use memchr::memmem;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Summary of a discovered Agent Documentation Bundle on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BundleStatus {
    pub bundle_dir: PathBuf,
    pub metadata: DocMetadata,
    pub sanity: SanityReport,
}

/// Workspace status combining converted bundles and unconverted PDF files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceStatus {
    pub bundles: Vec<BundleStatus>,
    pub unconverted_pdfs: Vec<PathBuf>,
}

/// Normalized hardware query supporting C hex (`0x400`), datasheet hex (`400h`),
/// and peripheral-prefixed identifiers (`GPIO.FSUB_0`, `GPIO->FSUB_0`, `GPIO_FSUB_0`).
#[derive(Debug, Clone)]
pub struct NormalizedHwQuery {
    pub raw: String,
    pub peripheral_hint: Option<String>,
    pub symbol: String,
    pub hex_value: Option<u64>,
    pub explicit_sep: bool,
}

impl NormalizedHwQuery {
    pub fn parse(raw_query: &str) -> Self {
        let trimmed = raw_query.trim();
        let hex_value = parse_hw_hex(trimmed);

        // Check explicit separators first: `GPIO.FSUB_0`, `GPIO->FSUB_0`, `GPIO::FSUB_0`
        if let Some((p, s)) = trimmed
            .split_once("->")
            .or_else(|| trimmed.split_once("::"))
            .or_else(|| trimmed.split_once('.'))
        {
            if !p.is_empty() && !s.is_empty() {
                return Self {
                    raw: trimmed.to_string(),
                    peripheral_hint: Some(p.trim().to_ascii_uppercase()),
                    symbol: s.trim().to_string(),
                    hex_value,
                    explicit_sep: true,
                };
            }
        }

        // Also check `PERIPH_REG` (e.g. `GPIO_FSUB_0` or `UART0_CTL0`)
        if let Some((prefix, rest)) = trimmed.split_once('_') {
            if prefix.len() >= 2
                && rest.len() >= 2
                && prefix.chars().all(|c| c.is_ascii_alphanumeric())
            {
                return Self {
                    raw: trimmed.to_string(),
                    peripheral_hint: Some(prefix.to_ascii_uppercase()),
                    symbol: rest.to_string(),
                    hex_value,
                    explicit_sep: false,
                };
            }
        }

        Self {
            raw: trimmed.to_string(),
            peripheral_hint: None,
            symbol: trimmed.to_string(),
            hex_value,
            explicit_sep: false,
        }
    }
}

/// Splits a hardware identifier token into `(canonical_stem, optional_instance)`.
/// Examples:
/// - `SYSCTL_C1105_C1106` -> `("SYSCTL", None)`
/// - `GPIOx` / `TIMx` / `CCRx` -> `("GPIO", None)` / `("TIM", None)` / `("CCR", None)`
/// - `GPIOA`..`GPIOK` -> `("GPIO", Some("A"))`..`("GPIO", Some("K"))`
/// - `TIM14` -> `("TIM", Some("14"))`, `USART1` -> `("USART", Some("1"))`, `CCR1` -> `("CCR", Some("1"))`
/// - `TIMA0` / `TIMG14` -> `("TIM", Some("A0"))` / `("TIM", Some("G14"))`
pub fn split_hw_stem_instance(raw_token: &str) -> (String, Option<String>) {
    let trimmed = raw_token.trim();
    if trimmed.is_empty() {
        return (String::new(), None);
    }

    // Strip peripheral variant suffixes after `_` (e.g., `SYSCTL_C1105_C1106` -> `SYSCTL`)
    let base = if let Some((prefix, rest)) = trimmed.split_once('_') {
        let is_variant_suffix = prefix.len() >= 2
            && !rest.is_empty()
            && rest.split('_').all(|seg| {
                seg.len() >= 3
                    && seg.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
                    && seg.chars().filter(|c| c.is_ascii_digit()).count() >= 2
            });
        if is_variant_suffix {
            prefix
        } else {
            trimmed
        }
    } else {
        trimmed
    };

    // Check template placeholder ending with lowercase 'x', 'n', 'y'
    // or uppercase 'X' when length >= 4 and not ending in MUX, IDX, BOX, MAX, AUX, TX, RX
    if base.len() >= 3 {
        let last_ch = base.chars().last().unwrap_or('\0');
        let upper = base.to_ascii_uppercase();
        let is_lower_placeholder = matches!(last_ch, 'x' | 'n' | 'y')
            && base[..base.len() - 1]
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        let is_upper_x_placeholder = last_ch == 'X'
            && upper.len() >= 4
            && !["MUX", "IDX", "BOX", "MAX", "AUX", "TX", "RX"]
                .iter()
                .any(|s| upper.ends_with(s));

        if is_lower_placeholder || is_upper_x_placeholder {
            let stem = upper[..upper.len() - 1].trim_end_matches('_').to_string();
            if stem.len() >= 2 {
                return (stem, None);
            }
        }
    }

    let upper = base.to_ascii_uppercase();

    // Check trailing digits after >= 2 letters (e.g. TIM14, USART1, CCR1, EXTICR1, TIMA0, TIMG14)
    if !upper.contains('_') {
        let digit_start = upper
            .find(|c: char| c.is_ascii_digit())
            .unwrap_or(upper.len());
        if digit_start >= 2 && digit_start < upper.len() {
            let alpha_part = &upper[..digit_start];
            let rest_part = &upper[digit_start..];
            if alpha_part.chars().all(|c| c.is_ascii_alphabetic())
                && rest_part.chars().all(|c| c.is_ascii_digit())
            {
                if (alpha_part.starts_with("TIM") && alpha_part.len() == 4)
                    || (alpha_part.starts_with("GPIO") && alpha_part.len() == 5)
                {
                    let stem_len = if alpha_part.starts_with("TIM") { 3 } else { 4 };
                    return (
                        alpha_part[..stem_len].to_string(),
                        Some(upper[stem_len..].to_string()),
                    );
                }
                return (alpha_part.to_string(), Some(rest_part.to_string()));
            }
        }
    }

    // Port-letter instance of length >= 5 like GPIOA..GPIOK
    if upper.len() == 5 && upper.starts_with("GPIO") && matches!(upper.as_bytes()[4], b'A'..=b'K') {
        return ("GPIO".to_string(), Some(upper[4..].to_string()));
    }

    (upper, None)
}

/// Checks whether a document hardware token and a query hardware token are compatible.
/// Returns `Some(4)` for exact match, `Some(3)` for template/instance/short-suffix match, or `None`.
pub fn hw_tokens_compatible(doc_tok: &str, query_tok: &str) -> Option<u8> {
    let d_trim = doc_tok.trim();
    let q_trim = query_tok.trim();
    if d_trim.is_empty() || q_trim.is_empty() {
        return None;
    }
    if d_trim.eq_ignore_ascii_case(q_trim) {
        return Some(4);
    }

    let (d_stem, d_inst) = split_hw_stem_instance(d_trim);
    let (q_stem, q_inst) = split_hw_stem_instance(q_trim);
    if !d_stem.is_empty() && d_stem == q_stem {
        let d_has_placeholder = d_trim.ends_with(['x', 'n', 'y', 'X']);
        let q_has_placeholder = q_trim.ends_with(['x', 'n', 'y', 'X']);
        match (d_inst, q_inst) {
            (Some(di), Some(qi)) if di == qi => return Some(4),
            (None, Some(_)) if d_stem.len() >= 3 || d_has_placeholder => return Some(3),
            (Some(_), None) if d_stem.len() >= 3 || q_has_placeholder => return Some(3),
            (None, None) => return Some(3),
            _ => return None,
        }
    }

    fn split_periph_reg(s: &str) -> Option<(&str, &str)> {
        s.split_once('.').or_else(|| s.split_once('_'))
    }

    if let (Some((dp, dr)), Some((qp, qr))) = (split_periph_reg(d_trim), split_periph_reg(q_trim)) {
        if let (Some(_), Some(_)) = (hw_tokens_compatible(dp, qp), hw_tokens_compatible(dr, qr)) {
            return Some(3);
        }
    }

    if !q_trim.contains('_') && !q_trim.contains('.') {
        if let Some((_, doc_short_reg)) = split_periph_reg(d_trim) {
            if hw_tokens_compatible(doc_short_reg, q_trim).is_some() {
                return Some(3);
            }
        }
    }

    None
}

/// Checks whether `haystack` contains `needle` at hardware identifier boundaries,
/// ensuring a trailing digit or short token does not match a longer number (e.g. `PA1` never matches `PA10`).
pub fn contains_hw_token_bounded(haystack: &str, needle: &str) -> bool {
    let h = haystack.to_ascii_uppercase();
    let n = needle.trim().to_ascii_uppercase();
    if n.is_empty() || n.len() > h.len() {
        return false;
    }
    for (start, _) in h.match_indices(&n) {
        let prev_ok = h[..start]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_ascii_alphanumeric());
        let next_ch = h[start + n.len()..].chars().next();
        let next_ok = if n.ends_with(|c: char| c.is_ascii_digit()) || n.len() <= 4 {
            next_ch.is_none_or(|c| !c.is_ascii_alphanumeric())
        } else {
            next_ch.is_none_or(|c| !c.is_ascii_digit())
        };
        if prev_ok && next_ok {
            return true;
        }
    }
    false
}

/// Parses either C-style hex (`0x41C00004`, `0x400`) or vendor-style hex (`41C00004h`, `400h`)
/// into a canonical `u64` integer for exact numerical address/offset comparison.
pub fn parse_hw_hex(token: &str) -> Option<u64> {
    let clean = token.trim().replace('_', "");
    if let Some(hex) = clean
        .strip_prefix("0x")
        .or_else(|| clean.strip_prefix("0X"))
    {
        if !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return u64::from_str_radix(hex, 16).ok();
        }
    }
    if let Some(hex) = clean.strip_suffix('h').or_else(|| clean.strip_suffix('H')) {
        if !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return u64::from_str_radix(hex, 16).ok();
        }
    }
    None
}

/// Resolves `target` into one or more converted bundle directories.
/// If `target` is itself a bundle (contains `meta.json`), returns `[target]`.
/// Otherwise searches `target` (and `target/output` if `target` is `.`) for subdirectories with `meta.json`.
pub fn resolve_bundles(target: &Path) -> Result<Vec<PathBuf>> {
    if target.join("meta.json").exists() && target.join("INDEX.md").exists() {
        return Ok(vec![target.to_path_buf()]);
    }
    let output_candidate = Path::new("output").join(target);
    if output_candidate.join("meta.json").exists() && output_candidate.join("INDEX.md").exists() {
        return Ok(vec![output_candidate]);
    }

    let mut bundles = Vec::new();
    let search_roots = if target.join("output").is_dir() {
        vec![target.to_path_buf(), target.join("output")]
    } else {
        vec![target.to_path_buf()]
    };

    for root in search_roots {
        if let Ok(entries) = fs::read_dir(&root) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir()
                    && p.join("meta.json").exists()
                    && p.join("INDEX.md").exists()
                    && !bundles.contains(&p)
                {
                    bundles.push(p);
                }
            }
        }
    }

    bundles.sort();
    if bundles.is_empty() {
        return Err(ChipdocError::BundleNotFound(target.to_path_buf()));
    }
    Ok(bundles)
}

/// Loads `BundleStatus` from a bundle directory's `meta.json`.
pub fn load_bundle_status(bundle_dir: &Path) -> Option<BundleStatus> {
    #[derive(Deserialize)]
    struct MetaFile {
        metadata: DocMetadata,
        sanity: SanityReport,
    }
    let meta_path = bundle_dir.join("meta.json");
    let content = fs::read_to_string(meta_path).ok()?;
    let parsed: MetaFile = serde_json::from_str(&content).ok()?;
    Some(BundleStatus {
        bundle_dir: bundle_dir.to_path_buf(),
        metadata: parsed.metadata,
        sanity: parsed.sanity,
    })
}

/// Scans a workspace root for converted bundles and unconverted `.pdf` files.
pub fn scan_workspace_status(workspace_root: &Path) -> WorkspaceStatus {
    let mut bundles = Vec::new();
    let mut converted_stems = std::collections::HashSet::new();

    if let Ok(dirs) = resolve_bundles(workspace_root) {
        for d in dirs {
            if let Some(st) = load_bundle_status(&d) {
                converted_stems.insert(st.metadata.doc_stem.clone());
                bundles.push(st);
            }
        }
    }

    let mut pdfs = Vec::new();
    collect_pdfs_bounded(workspace_root, 0, 3, &mut pdfs);
    pdfs.sort();

    let unconverted_pdfs = pdfs
        .into_iter()
        .filter(|p| {
            let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
            !converted_stems.contains(stem)
        })
        .collect();

    WorkspaceStatus {
        bundles,
        unconverted_pdfs,
    }
}

fn collect_pdfs_bounded(dir: &Path, depth: usize, max_depth: usize, out: &mut Vec<PathBuf>) {
    if depth > max_depth {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if p.is_dir() {
            if name_str.starts_with('.')
                || name_str == "target"
                || name_str == "node_modules"
                || name_str == "output"
                || name_str == "bin"
            {
                continue;
            }
            collect_pdfs_bounded(&p, depth + 1, max_depth, out);
        } else if p
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("pdf"))
        {
            out.push(p);
        }
    }
}

/// Queries registers and bitfields across one or more bundles with hardware hex/symbol normalization.
pub fn query_registers_formatted(
    bundles: &[PathBuf],
    raw_query: &str,
    limit: usize,
    compact: bool,
) -> Result<String> {
    let q = NormalizedHwQuery::parse(raw_query);
    let sym_upper = q.symbol.to_ascii_uppercase();
    let raw_upper = q.raw.to_ascii_uppercase();

    #[derive(Clone)]
    struct RegCard {
        bundle_name: String,
        peripheral: String,
        register: String,
        offset: String,
        reset: String,
        location: String,
        start_page: usize,
        end_page: usize,
        page: String,
        fields: Vec<(String, String, String, String, String)>,
        match_score: u8,
    }

    let per_bundle: Vec<Vec<RegCard>> = bundles
        .par_iter()
        .map(|bundle_dir| {
            let reg_tsv = bundle_dir.join("index").join("registers.tsv");
            let Ok(content) = fs::read_to_string(&reg_tsv) else {
                return Vec::new();
            };
            let bundle_name = bundle_dir
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("bundle")
                .to_string();

            let mut cards: Vec<RegCard> = Vec::new();

            for line in content.lines().skip(1) {
                let cols: Vec<&str> = line.split('\t').collect();
                if cols.len() < 11 {
                    continue;
                }
                let periph = cols[0];
                let reg = cols[1];
                let offset = cols[2];
                let reset = cols[3];
                let bits = cols[4];
                let field = cols[5];
                let access = cols[6];
                let f_reset = cols[7];
                let desc = cols[8];
                let loc = cols[9];
                let page_str = cols[10];
                let page_num = page_str.parse::<usize>().unwrap_or(0);

                let append_to_last = cards.last().is_some_and(|last| {
                    last.peripheral == periph
                        && last.register == reg
                        && last.offset == offset
                        && page_num >= last.start_page
                        && page_num <= last.end_page + 1
                });

                if append_to_last {
                    let card = cards.last_mut().expect("non-empty");
                    if page_num > card.end_page {
                        card.end_page = page_num;
                        if card.start_page != card.end_page {
                            card.page = format!("{}-{}", card.start_page, card.end_page);
                        }
                    }
                    let dup_field = card
                        .fields
                        .iter()
                        .any(|(b, f, _, _, _)| b == bits && f == field);
                    if !dup_field {
                        card.fields.push((
                            bits.to_string(),
                            field.to_string(),
                            access.to_string(),
                            f_reset.to_string(),
                            desc.to_string(),
                        ));
                    }
                } else {
                    cards.push(RegCard {
                        bundle_name: bundle_name.clone(),
                        peripheral: periph.to_string(),
                        register: reg.to_string(),
                        offset: offset.to_string(),
                        reset: reset.to_string(),
                        location: loc.to_string(),
                        start_page: page_num,
                        end_page: page_num,
                        page: page_str.to_string(),
                        fields: vec![(
                            bits.to_string(),
                            field.to_string(),
                            access.to_string(),
                            f_reset.to_string(),
                            desc.to_string(),
                        )],
                        match_score: 0,
                    });
                }
            }

            for card in &mut cards {
                let periph_u = card.peripheral.to_ascii_uppercase();
                let reg_u = card.register.to_ascii_uppercase();
                let short_reg = card
                    .register
                    .split_once('_')
                    .map(|(_, r)| r)
                    .unwrap_or(&card.register);
                let short_reg_u = short_reg.to_ascii_uppercase();
                let reg_prefix = card.register.split_once('_').map(|(p, _)| p).unwrap_or("");

                let combined_dot = format!("{periph_u}.{reg_u}");
                let combined_under = format!("{periph_u}_{reg_u}");
                let combined_short_dot = format!("{periph_u}.{short_reg_u}");
                let combined_short_under = format!("{periph_u}_{short_reg_u}");

                let hex_match = match (q.hex_value, parse_hw_hex(&card.offset)) {
                    (Some(qv), Some(ov)) => qv == ov,
                    _ => false,
                };

                let periph_filter_ok = q.peripheral_hint.as_ref().is_none_or(|ph| {
                    periph_u.contains(ph)
                        || reg_u.starts_with(ph)
                        || hw_tokens_compatible(&card.peripheral, ph).is_some()
                        || (!reg_prefix.is_empty()
                            && hw_tokens_compatible(reg_prefix, ph).is_some())
                });

                let exact_reg_match = reg_u == raw_upper
                    || (!q.explicit_sep && short_reg_u == raw_upper)
                    || (periph_filter_ok
                        && (combined_dot == raw_upper
                            || combined_under == raw_upper
                            || combined_short_dot == raw_upper
                            || combined_short_under == raw_upper
                            || (q
                                .peripheral_hint
                                .as_deref()
                                .is_some_and(|ph| periph_u == ph)
                                && (reg_u == sym_upper || short_reg_u == sym_upper))));

                let template_reg_match = hw_tokens_compatible(&card.register, &q.raw).is_some()
                    || hw_tokens_compatible(&combined_under, &q.raw).is_some()
                    || hw_tokens_compatible(&combined_short_under, &q.raw).is_some()
                    || (periph_filter_ok
                        && (hw_tokens_compatible(&card.register, &q.symbol).is_some()
                            || hw_tokens_compatible(short_reg, &q.symbol).is_some()));

                let exact_bitfield_match = card.fields.iter().any(|(_, f, _, _, _)| {
                    let fu = f.to_ascii_uppercase();
                    let fb = fu.split('[').next().unwrap_or(&fu);
                    (!q.explicit_sep
                        && (fu == raw_upper
                            || fb == raw_upper
                            || hw_tokens_compatible(fb, &q.raw).is_some()))
                        || (periph_filter_ok
                            && (fu == sym_upper
                                || fb == sym_upper
                                || hw_tokens_compatible(fb, &q.symbol).is_some()))
                });

                let periph_match = q.peripheral_hint.is_none()
                    && hw_tokens_compatible(&card.peripheral, &q.raw).is_some();

                let substring_match = (periph_filter_ok
                    && (reg_u.contains(&sym_upper)
                        || card
                            .fields
                            .iter()
                            .any(|(_, f, _, _, _)| f.to_ascii_uppercase().contains(&sym_upper))
                        || combined_dot.contains(&raw_upper)
                        || combined_under.contains(&raw_upper)))
                    || (!q.explicit_sep
                        && (reg_u.contains(&raw_upper)
                            || card.fields.iter().any(|(_, f, _, _, _)| {
                                f.to_ascii_uppercase().contains(&raw_upper)
                            })));

                card.match_score = if hex_match || exact_reg_match {
                    5
                } else if template_reg_match || exact_bitfield_match {
                    4
                } else if periph_match {
                    3
                } else if substring_match {
                    2
                } else {
                    0
                };
            }

            cards.into_iter().filter(|c| c.match_score > 0).collect()
        })
        .collect();

    let mut all_cards: Vec<RegCard> = per_bundle.into_iter().flatten().collect();
    all_cards.sort_by_key(|a| std::cmp::Reverse(a.match_score));

    let total_found = all_cards.len();
    if total_found == 0 {
        return Ok(format!(
            "No register or bitfield matches for '{raw_query}'. Try `chipdoc search \"{raw_query}\"` or `chipdoc table \"{raw_query}\"`.\n"
        ));
    }

    let mut out = String::new();
    let mut total_shown = 0usize;

    for card in all_cards.into_iter().take(limit) {
        total_shown += 1;
        let hex_norm = parse_hw_hex(&card.offset)
            .map(|v| format!(" (0x{v:X})"))
            .unwrap_or_default();
        out.push_str(&format!(
            "[{bundle}] {periph}.{reg} | Offset: {off}{hex_norm} | Reset: {rst} | p. {page} ({loc})\n",
            bundle = card.bundle_name,
            periph = card.peripheral,
            reg = card.register,
            off = card.offset,
            rst = card.reset,
            page = card.page,
            loc = card.location,
        ));
        for (bits, fname, acc, frst, desc) in &card.fields {
            let short_desc = if compact && desc.len() > 120 {
                format!("{}...", &desc[..117])
            } else {
                desc.clone()
            };
            out.push_str(&format!(
                "  [{bits:>5}] {fname:<18} ({acc}, reset={frst}): {short_desc}\n"
            ));
        }
        out.push('\n');
    }

    if total_found > total_shown {
        out.push_str(&format!(
            "... +{} more matching registers omitted (pass `--limit {}` to view all).\n",
            total_found - total_shown,
            total_found
        ));
    }

    Ok(out)
}

fn has_exact_number_boundary(text: &str, pat: &str) -> bool {
    if pat.is_empty() || pat.len() > text.len() {
        return false;
    }
    for (idx, _) in text.match_indices(pat) {
        let prev_ok = text[..idx]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_ascii_alphanumeric());
        let next_ch = text[idx + pat.len()..].chars().next();
        let next_ok = next_ch.is_none_or(|c| !c.is_ascii_digit() && c != '-');
        if prev_ok && next_ok {
            return true;
        }
    }
    false
}

fn score_caption_or_id_hit(id: &str, caption: &str, q_lower: &str) -> u8 {
    if q_lower.is_empty() {
        return 1;
    }
    let id_lower = id.to_ascii_lowercase();
    let cap_lower = caption.to_ascii_lowercase();

    let id_target = if let Some(rest) = q_lower.strip_prefix("table ") {
        Some(format!("table-{}", rest.trim()))
    } else if let Some(rest) = q_lower.strip_prefix("figure ") {
        Some(format!("fig-{}", rest.trim()))
    } else {
        q_lower
            .strip_prefix("fig.")
            .or_else(|| q_lower.strip_prefix("fig "))
            .map(|rest| format!("fig-{}", rest.trim()))
    };

    if id_lower == q_lower
        || cap_lower == q_lower
        || has_exact_number_boundary(&cap_lower, q_lower)
        || id_target
            .as_deref()
            .is_some_and(|t| id_lower == t || has_exact_number_boundary(&id_lower, t))
    {
        3
    } else if cap_lower.contains(q_lower)
        || id_lower.contains(q_lower)
        || id_target.as_deref().is_some_and(|t| id_lower.contains(t))
    {
        2
    } else {
        1
    }
}

/// Queries tables across one or more bundles, optionally rendering the full Markdown table directly.
pub fn query_tables_formatted(
    bundles: &[PathBuf],
    query: &str,
    show_markdown: bool,
    max_rows: usize,
    limit: usize,
) -> Result<String> {
    let q_lower = query.trim().to_ascii_lowercase();

    #[derive(Clone)]
    struct TableHit {
        bundle_dir: PathBuf,
        bundle_name: String,
        table_id: String,
        caption: String,
        columns: String,
        rows_count: String,
        location: String,
        pages: String,
        score: u8,
    }

    let hits_per_bundle: Vec<Vec<TableHit>> = bundles
        .par_iter()
        .map(|bundle_dir| {
            let tab_tsv = bundle_dir.join("index").join("tables.tsv");
            let Ok(content) = fs::read_to_string(&tab_tsv) else {
                return Vec::new();
            };
            let bundle_name = bundle_dir
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("bundle")
                .to_string();

            let mut hits = Vec::new();
            for line in content.lines().skip(1) {
                if !line.to_ascii_lowercase().contains(&q_lower) {
                    continue;
                }
                let cols: Vec<&str> = line.split('\t').collect();
                if cols.len() < 6 {
                    continue;
                }
                let score = score_caption_or_id_hit(cols[0], cols[1], &q_lower);
                hits.push(TableHit {
                    bundle_dir: bundle_dir.clone(),
                    bundle_name: bundle_name.clone(),
                    table_id: cols[0].to_string(),
                    caption: cols[1].to_string(),
                    columns: cols[2].to_string(),
                    rows_count: cols[3].to_string(),
                    location: cols[4].to_string(),
                    pages: cols[5].to_string(),
                    score,
                });
            }
            hits
        })
        .collect();

    let mut all_hits: Vec<TableHit> = hits_per_bundle.into_iter().flatten().collect();
    all_hits.sort_by_key(|a| std::cmp::Reverse(a.score));

    if all_hits.is_empty() {
        return Ok(format!(
            "No tables matched '{query}'. Try `chipdoc search \"{query}\"` to search inside table cells.\n"
        ));
    }

    let total = all_hits.len();
    let should_show_md = show_markdown || total <= 2;
    let mut out = String::new();

    for (idx, hit) in all_hits.into_iter().take(limit).enumerate() {
        out.push_str(&format!(
            "[{bundle}] {id} | {cap} | {rows} rows | pp. {pages} ({loc})\n  Columns: {cols}\n",
            bundle = hit.bundle_name,
            id = hit.table_id,
            cap = hit.caption,
            rows = hit.rows_count,
            pages = hit.pages,
            loc = hit.location,
            cols = hit.columns,
        ));

        if should_show_md && idx < 5 {
            if let Some(md_table) =
                extract_markdown_table_at_location(&hit.bundle_dir, &hit.location, max_rows)
            {
                out.push('\n');
                for l in md_table {
                    out.push_str("  ");
                    out.push_str(&l);
                    out.push('\n');
                }
            }
        }
        out.push('\n');
    }

    if total > limit {
        out.push_str(&format!(
            "... +{} more tables matched (refine query or pass `--limit`).\n",
            total - limit
        ));
    }

    Ok(out)
}

fn extract_markdown_table_at_location(
    bundle_dir: &Path,
    location: &str,
    max_rows: usize,
) -> Option<Vec<String>> {
    let (rel_file, line_str) = location.rsplit_once(':')?;
    let start_line: usize = line_str.parse().ok()?;
    let full_path = bundle_dir.join(rel_file);
    let content = fs::read_to_string(full_path).ok()?;

    let mut out = Vec::new();
    let mut data_rows = 0usize;
    let mut seen_pipe_row = false;
    for line in content.lines().skip(start_line.saturating_sub(1)) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if seen_pipe_row {
                break;
            }
            continue;
        }
        if seen_pipe_row && !trimmed.starts_with('|') {
            break;
        }
        out.push(line.to_string());
        if trimmed.starts_with('|') {
            seen_pipe_row = true;
            if !trimmed.contains("---") {
                data_rows += 1;
                if data_rows > max_rows + 1 {
                    out.push(format!(
                        "| ... (truncated at {max_rows} rows; read `{location}` for full table) |"
                    ));
                    break;
                }
            }
        }
    }
    Some(out)
}

fn contains_word_bounded(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() || needle.len() > haystack.len() {
        return false;
    }
    for (idx, _) in haystack.match_indices(needle) {
        let prev_ok = haystack[..idx]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_ascii_alphanumeric());
        let next_ok = haystack[idx + needle.len()..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_ascii_alphanumeric());
        if prev_ok && next_ok {
            return true;
        }
    }
    false
}

/// Queries section shards by ID, slug, or title keyword and prints either the section summary or content.
pub fn query_sections_formatted(
    bundles: &[PathBuf],
    query: &str,
    max_lines: usize,
) -> Result<String> {
    let q_lower = query.trim().to_ascii_lowercase();
    let mut out = String::new();
    let mut matches = Vec::new();

    for bundle_dir in bundles {
        let sec_tsv = bundle_dir.join("index").join("sections.tsv");
        let Ok(content) = fs::read_to_string(&sec_tsv) else {
            continue;
        };
        let bundle_name = bundle_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("bundle")
            .to_string();

        for line in content.lines().skip(1) {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 8 {
                continue;
            }
            let sec_id = cols[0];
            let title = cols[1];
            let file_path = cols[2];
            let lines_cnt = cols[3];
            let pages = cols[4];
            let tables = cols[5];
            let regs = cols[6];

            let title_lower = title.to_ascii_lowercase();
            let fpath_lower = file_path.to_ascii_lowercase();

            let score: u8 = if q_lower.is_empty() {
                1
            } else if title_lower.starts_with(&format!("{q_lower} "))
                || title_lower.starts_with(&format!("{q_lower}. "))
            {
                4
            } else if sec_id.eq_ignore_ascii_case(&q_lower) || title_lower == q_lower {
                3
            } else if contains_word_bounded(&title_lower, &q_lower)
                || contains_word_bounded(&fpath_lower, &q_lower)
            {
                2
            } else if title_lower.contains(&q_lower) || fpath_lower.contains(&q_lower) {
                1
            } else {
                0
            };

            if score > 0 {
                matches.push((
                    score,
                    bundle_dir.clone(),
                    bundle_name.clone(),
                    sec_id.to_string(),
                    title.to_string(),
                    file_path.to_string(),
                    lines_cnt.to_string(),
                    pages.to_string(),
                    tables.to_string(),
                    regs.to_string(),
                ));
            }
        }
    }

    matches.sort_by_key(|a| std::cmp::Reverse(a.0));

    // Dotted sub-section lookup (`10.3`, `3.7`, `6.4.1`) inside `sections/*.md`
    static RE_DOTTED_SEC: OnceLock<Regex> = OnceLock::new();
    let re_dotted =
        RE_DOTTED_SEC.get_or_init(|| Regex::new(r"^(\d+(?:\.\d+)+)").expect("valid regex"));

    if matches.is_empty() {
        if let Some(caps) = re_dotted.captures(&q_lower) {
            let dotted_num = caps.get(1).expect("group 1").as_str();
            for bundle_dir in bundles {
                let bundle_name = bundle_dir
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("bundle")
                    .to_string();
                let sec_tsv = bundle_dir.join("index").join("sections.tsv");
                let Ok(content) = fs::read_to_string(&sec_tsv) else {
                    continue;
                };
                for line in content.lines().skip(1) {
                    let cols: Vec<&str> = line.split('\t').collect();
                    if cols.len() < 8 {
                        continue;
                    }
                    let sec_id = cols[0];
                    let title = cols[1];
                    let file_path = cols[2];
                    let lines_cnt = cols[3];
                    let pages = cols[4];
                    let tables = cols[5];
                    let regs = cols[6];

                    let Ok(md) = fs::read_to_string(bundle_dir.join(file_path)) else {
                        continue;
                    };
                    let all_lines: Vec<&str> = md.lines().collect();
                    let found_idx = all_lines.iter().position(|l| {
                        let trimmed = l.trim_start();
                        if !trimmed.starts_with("##") {
                            return false;
                        }
                        let after_hashes = trimmed.trim_start_matches('#').trim_start();
                        let after_lower = after_hashes.to_ascii_lowercase();
                        if let Some(rest) = after_lower.strip_prefix(dotted_num) {
                            rest.chars()
                                .next()
                                .is_none_or(|c| !c.is_ascii_alphanumeric() && c != '.')
                        } else {
                            false
                        }
                    });

                    if let Some(start_idx) = found_idx {
                        out.push_str(&format!(
                            "[{bundle_name}] Section {sec_id}: {title} | pp. {pages} | {lines_cnt} lines, {tables} tables, {regs} regs | `{file_path}:{}`\n",
                            start_idx + 1
                        ));
                        if max_lines > 0 {
                            out.push_str("---\n");
                            for l in all_lines.iter().skip(start_idx).take(max_lines) {
                                out.push_str(l);
                                out.push('\n');
                            }
                            if all_lines.len() > start_idx + max_lines {
                                out.push_str(&format!(
                                    "\n... (+{} more lines in `{file_path}`; pass `--max-lines` or Read with offset={})\n",
                                    all_lines.len() - (start_idx + max_lines),
                                    start_idx + max_lines + 1
                                ));
                            }
                        }
                        return Ok(out);
                    }
                }
            }
        }

        return Ok(format!(
            "No sections matched '{query}'. Run `chipdoc section \"\"` to list all sections.\n"
        ));
    }

    for (idx, (score, bdir, bname, sid, title, fpath, lcnt, pages, tabs, regs)) in
        matches.iter().enumerate()
    {
        out.push_str(&format!(
            "[{bname}] Section {sid}: {title} | pp. {pages} | {lcnt} lines, {tabs} tables, {regs} regs | `{fpath}`\n"
        ));

        // If 1 or 2 sections matched (or top match is an exact chapter/ID hit) and max_lines > 0, print the section content preview
        if (matches.len() <= 2 || *score >= 3) && max_lines > 0 && idx == 0 {
            if let Ok(md) = fs::read_to_string(bdir.join(fpath)) {
                let total_lines = md.lines().count();
                out.push_str("---\n");
                for line in md.lines().take(max_lines) {
                    out.push_str(line);
                    out.push('\n');
                }
                if total_lines > max_lines {
                    out.push_str(&format!(
                        "\n... (+{} more lines in `{fpath}`; pass `--max-lines` or Read with offset={})\n",
                        total_lines - max_lines,
                        max_lines + 1
                    ));
                }
            }
        }
    }

    Ok(out)
}

/// Extracts Markdown content corresponding to specific 1-based PDF page number(s) from a bundle.
pub fn query_pages_formatted(
    bundle_dir: &Path,
    page_expr: &str,
    max_lines: usize,
) -> Result<String> {
    let target_pages = parse_page_range_expr(page_expr)?;
    let sec_tsv = bundle_dir.join("index").join("sections.tsv");
    let content = fs::read_to_string(&sec_tsv).map_err(|source| ChipdocError::Io {
        path: sec_tsv,
        source,
    })?;

    let mut matching_files = Vec::new();
    for line in content.lines().skip(1) {
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 5 {
            continue;
        }
        let file_path = cols[2];
        let pages_str = cols[4];
        if let Some((s, e)) = pages_str.split_once('-') {
            if let (Ok(sp), Ok(ep)) = (s.parse::<usize>(), e.parse::<usize>()) {
                if target_pages.iter().any(|&p| p >= sp && p <= ep) {
                    matching_files.push((file_path.to_string(), sp, ep));
                }
            }
        }
    }

    if matching_files.is_empty() {
        return Ok(format!(
            "No converted section covers page(s) '{page_expr}'.\n"
        ));
    }

    let re_page_marker = Regex::new(r"\(p\.\s*(\d+)\)").expect("valid regex");
    let mut out = String::new();
    let mut printed_lines = 0usize;

    for (rel_path, sp, ep) in matching_files {
        let full_path = bundle_dir.join(&rel_path);
        let Ok(md) = fs::read_to_string(&full_path) else {
            continue;
        };

        out.push_str(&format!("=== `{rel_path}` (shard pp. {sp}-{ep}) ===\n"));
        // Track current page as we walk headings/captions with `(p. N)`; if a shard only covers 1-2 pages, include all
        let mut current_page_in_target =
            (ep.saturating_sub(sp)) <= 1 && target_pages.iter().any(|&p| p >= sp && p <= ep);

        for (line_idx, line) in md.lines().enumerate() {
            if let Some(caps) = re_page_marker.captures(line) {
                if let Some(m) = caps.get(1) {
                    if let Ok(pnum) = m.as_str().parse::<usize>() {
                        current_page_in_target = target_pages.contains(&pnum);
                    }
                }
            }
            if current_page_in_target {
                out.push_str(&format!("{:>4}: {}\n", line_idx + 1, line));
                printed_lines += 1;
                if printed_lines >= max_lines {
                    out.push_str(&format!(
                        "... (truncated at {max_lines} lines; pass `--max-lines` or inspect `{rel_path}:{}`)\n",
                        line_idx + 2
                    ));
                    return Ok(out);
                }
            }
        }
        out.push('\n');
    }

    if printed_lines == 0 {
        out.push_str(
            "No explicit `(p. N)` markers found for that exact page inside the shard; use `chipdoc section` to view the full shard.\n",
        );
    }

    Ok(out)
}

/// Queries the Hardware Cross-Reference Graph (`index/xref.tsv`) across bundles.
pub fn query_xref_formatted(bundles: &[PathBuf], symbol: &str, limit: usize) -> Result<String> {
    type XrefRow = (String, String, String, String, String, String);

    let q = NormalizedHwQuery::parse(symbol);
    let sym_upper = q.symbol.to_ascii_uppercase();
    let raw_upper = q.raw.to_ascii_uppercase();

    let per_bundle: Vec<(String, Vec<XrefRow>)> = bundles
        .par_iter()
        .map(|bundle_dir| {
            let bundle_name = bundle_dir
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("bundle")
                .to_string();
            let xref_tsv = bundle_dir.join("index").join("xref.tsv");
            let mut scored_hits: Vec<(u8, XrefRow)> = Vec::new();

            if let Ok(content) = fs::read_to_string(&xref_tsv) {
                let mut seen_loc: std::collections::HashSet<(String, String)> =
                    std::collections::HashSet::new();
                // Pass 1: exact, template, or bounded token match on `symbol` or `peripheral`
                for line in content.lines().skip(1) {
                    let cols: Vec<&str> = line.split('\t').collect();
                    if cols.len() < 6 {
                        continue;
                    }
                    let xsym = cols[0];
                    let xsym_u = xsym.to_ascii_uppercase();
                    let xperiph = cols[2];
                    let xperiph_u = xperiph.to_ascii_uppercase();

                    let periph_hint_ok = q.peripheral_hint.as_ref().is_none_or(|ph| {
                        hw_tokens_compatible(xperiph, ph).is_some()
                            || contains_hw_token_bounded(&xsym_u, ph)
                    });

                    let score: u8 =
                        if xsym_u == raw_upper || (!q.explicit_sep && xsym_u == sym_upper) {
                            4
                        } else if hw_tokens_compatible(xsym, &q.raw).is_some()
                            || (q.peripheral_hint.is_none()
                                && (xperiph_u == raw_upper
                                    || hw_tokens_compatible(xperiph, &q.raw).is_some()))
                            || (periph_hint_ok && hw_tokens_compatible(xsym, &q.symbol).is_some())
                        {
                            3
                        } else if contains_hw_token_bounded(&xsym_u, &raw_upper)
                            || (periph_hint_ok && contains_hw_token_bounded(&xsym_u, &sym_upper))
                        {
                            2
                        } else {
                            0
                        };

                    if score > 0 {
                        let loc_key = (cols[1].to_string(), cols[4].to_string());
                        if seen_loc.insert(loc_key) {
                            scored_hits.push((
                                score,
                                (
                                    cols[0].to_string(),
                                    cols[1].to_string(),
                                    cols[2].to_string(),
                                    cols[3].to_string(),
                                    cols[4].to_string(),
                                    cols[5].to_string(),
                                ),
                            ));
                        }
                    }
                }
                // Pass 2: also include bounded summary matches whose location was not yet emitted
                for line in content.lines().skip(1) {
                    let cols: Vec<&str> = line.split('\t').collect();
                    if cols.len() < 6 {
                        continue;
                    }
                    let xsum = cols[3].to_ascii_uppercase();
                    if contains_hw_token_bounded(&xsum, &raw_upper) {
                        let loc_key = (cols[1].to_string(), cols[4].to_string());
                        if seen_loc.insert(loc_key) {
                            scored_hits.push((
                                1,
                                (
                                    cols[0].to_string(),
                                    cols[1].to_string(),
                                    cols[2].to_string(),
                                    cols[3].to_string(),
                                    cols[4].to_string(),
                                    cols[5].to_string(),
                                ),
                            ));
                        }
                    }
                }
            } else {
                // Fallback for v0.1 bundles that predate `index/xref.tsv`
                for (kind, file) in [
                    ("section", "sections.tsv"),
                    ("register", "registers.tsv"),
                    ("table", "tables.tsv"),
                    ("figure", "figures.tsv"),
                ] {
                    let p = bundle_dir.join("index").join(file);
                    if let Ok(c) = fs::read_to_string(p) {
                        for line in c.lines().skip(1) {
                            if contains_hw_token_bounded(&line.to_ascii_uppercase(), &sym_upper) {
                                scored_hits.push((
                                    1,
                                    (
                                        q.symbol.clone(),
                                        kind.to_string(),
                                        "-".to_string(),
                                        line.replace('\t', " | "),
                                        "-".to_string(),
                                        "-".to_string(),
                                    ),
                                ));
                            }
                        }
                    }
                }
            }

            scored_hits.sort_by_key(|a| std::cmp::Reverse(a.0));
            let hits: Vec<XrefRow> = scored_hits.into_iter().map(|(_, row)| row).collect();
            (bundle_name, hits)
        })
        .collect();

    let mut out = String::new();
    let mut total_hits = 0usize;

    for (bundle_name, hits) in per_bundle {
        if hits.is_empty() {
            continue;
        }
        total_hits += hits.len();
        out.push_str(&format!(
            "=== Cross-Reference Graph (`{bundle_name}`) for '{symbol}' ({} connections) ===\n",
            hits.len()
        ));
        for (sym, kind, periph, summary, loc, page) in hits.iter().take(limit) {
            out.push_str(&format!(
                "  [{kind:<9}] {sym:<16} ({periph}) -> {summary} | p. {page} ({loc})\n"
            ));
        }
        if hits.len() > limit {
            out.push_str(&format!(
                "  ... +{} more connections in `{bundle_name}` (pass `--limit` to expand)\n",
                hits.len() - limit
            ));
        }
        out.push('\n');
    }

    if total_hits == 0 {
        return Ok(format!(
            "No cross-reference graph connections found for '{symbol}'.\n"
        ));
    }

    Ok(out)
}

/// Queries figures across bundles and optionally renders cropped PNGs on demand (`--render`).
pub fn query_figures_formatted(
    bundles: &[PathBuf],
    query: &str,
    render_pdf: Option<&Path>,
    dpi: f32,
    pdfium_dir: Option<&Path>,
    limit: usize,
) -> Result<String> {
    let q_lower = query.trim().to_ascii_lowercase();

    struct FigureHit {
        bundle_dir: PathBuf,
        bundle_name: String,
        fig_id: String,
        caption: String,
        img_rel: String,
        loc: String,
        page_str: String,
        bbox_str: String,
        score: u8,
    }

    let mut hits: Vec<FigureHit> = Vec::new();

    for bundle_dir in bundles {
        let fig_tsv = bundle_dir.join("index").join("figures.tsv");
        let Ok(content) = fs::read_to_string(&fig_tsv) else {
            continue;
        };
        let bundle_name = bundle_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("bundle")
            .to_string();

        for line in content.lines().skip(1) {
            if !q_lower.is_empty() && !line.to_ascii_lowercase().contains(&q_lower) {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 5 {
                continue;
            }
            let score = score_caption_or_id_hit(cols[0], cols[1], &q_lower);
            hits.push(FigureHit {
                bundle_dir: bundle_dir.clone(),
                bundle_name: bundle_name.clone(),
                fig_id: cols[0].to_string(),
                caption: cols[1].to_string(),
                img_rel: cols[2].to_string(),
                loc: cols[3].to_string(),
                page_str: cols[4].to_string(),
                bbox_str: cols.get(5).copied().unwrap_or("").to_string(),
                score,
            });
        }
    }

    if hits.is_empty() {
        return Ok(format!("No figures matched '{query}'.\n"));
    }

    hits.sort_by_key(|a| std::cmp::Reverse(a.score));

    let mut out = String::new();
    for hit in hits.into_iter().take(limit) {
        let img_full = hit.bundle_dir.join(&hit.img_rel);
        let mut status_tag = if !hit.img_rel.is_empty() && img_full.exists() {
            "rendered".to_string()
        } else {
            "lazy (not yet rendered)".to_string()
        };

        if let Some(pdf_path) = render_pdf {
            if !hit.img_rel.is_empty() && !img_full.exists() {
                if let (Ok(page_num), Some(bbox)) =
                    (hit.page_str.parse::<usize>(), parse_bbox_csv(&hit.bbox_str))
                {
                    render_single_figure_crop(
                        pdf_path, pdfium_dir, page_num, bbox, &img_full, dpi,
                    )?;
                    status_tag = "rendered on-demand".to_string();
                }
            }
        }

        out.push_str(&format!(
            "[{bundle_name}] {fig_id} | {caption} | p. {page_str} ({loc}) | PNG: `{}` [{status_tag}]\n",
            img_full.display(),
            bundle_name = hit.bundle_name,
            fig_id = hit.fig_id,
            caption = hit.caption,
            page_str = hit.page_str,
            loc = hit.loc,
        ));
    }

    Ok(out)
}

fn parse_bbox_csv(s: &str) -> Option<BBox> {
    let parts: Vec<f32> = s
        .split(',')
        .filter_map(|p| p.trim().parse::<f32>().ok())
        .collect();
    if parts.len() == 4 {
        Some(BBox::new(parts[0], parts[1], parts[2], parts[3]))
    } else {
        None
    }
}

/// High-speed parallel search across one or more bundles using SIMD (`memchr::memmem`) for literals
/// and `Regex` for pattern queries, with token-safe truncation and overflow summaries.
pub fn parallel_bundle_search(
    bundles: &[PathBuf],
    pattern: &str,
    kind: &str,
    ignore_case: bool,
    limit: usize,
    compact: bool,
) -> Result<String> {
    let re = RegexBuilder::new(pattern)
        .case_insensitive(ignore_case)
        .build()
        .map_err(|e| ChipdocError::InvalidPageRange(format!("Invalid regex '{pattern}': {e}")))?;

    let is_literal = !ignore_case
        && !pattern.chars().any(|c| {
            matches!(
                c,
                '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '^' | '$' | '|' | '\\'
            )
        });
    let simd_finder = is_literal.then(|| memmem::Finder::new(pattern.as_bytes()));

    let matches_line = |line: &str| -> bool {
        if let Some(ref finder) = simd_finder {
            finder.find(line.as_bytes()).is_some()
        } else {
            re.is_match(line)
        }
    };

    let k = kind.trim().to_ascii_lowercase();
    let search_regs = k == "all" || k.starts_with("reg");
    let search_tabs = k == "all" || k.starts_with("tab");
    let search_figs = k == "all" || k.starts_with("fig");
    let search_xref = k == "all" || k.starts_with("xref");
    let search_secs = k == "all" || k.starts_with("sec") || k == "md";

    let mut out = String::new();

    for bundle_dir in bundles {
        let bundle_name = bundle_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("bundle");

        // 1. TSV index searches
        for (enabled, label, filename) in [
            (
                search_regs,
                "Registers (`index/registers.tsv`)",
                "registers.tsv",
            ),
            (search_tabs, "Tables (`index/tables.tsv`)", "tables.tsv"),
            (search_figs, "Figures (`index/figures.tsv`)", "figures.tsv"),
            (search_xref, "Cross-Refs (`index/xref.tsv`)", "xref.tsv"),
        ] {
            if !enabled {
                continue;
            }
            let tsv_path = bundle_dir.join("index").join(filename);
            let Ok(content) = fs::read_to_string(&tsv_path) else {
                continue;
            };
            let all_matches: Vec<&str> = content
                .lines()
                .skip(1)
                .filter(|line| matches_line(line))
                .collect();
            if !all_matches.is_empty() {
                out.push_str(&format!(
                    "=== [{bundle_name}] {label} ({} hits) ===\n",
                    all_matches.len()
                ));
                for m in all_matches.iter().take(limit) {
                    if compact && m.len() > 180 {
                        out.push_str(&format!("  {}...\n", &m[..177]));
                    } else {
                        out.push_str(&format!("  {m}\n"));
                    }
                }
                if all_matches.len() > limit {
                    out.push_str(&format!(
                        "  ... +{} more matches in `{filename}` (pass `--limit` to expand)\n",
                        all_matches.len() - limit
                    ));
                }
                out.push('\n');
            }
        }

        // 2. Parallel scan of `sections/*.md` using Rayon
        let sections_dir = bundle_dir.join("sections");
        if search_secs && sections_dir.exists() {
            let mut md_paths: Vec<PathBuf> = fs::read_dir(&sections_dir)
                .into_iter()
                .flatten()
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("md"))
                .collect();
            md_paths.sort();

            let file_hits: Vec<(String, Vec<(usize, String)>)> = md_paths
                .par_iter()
                .filter_map(|path| {
                    let rel_name = path.file_name()?.to_str()?.to_string();
                    let content = fs::read_to_string(path).ok()?;
                    if let Some(ref finder) = simd_finder {
                        finder.find(content.as_bytes())?;
                    }
                    let hits: Vec<(usize, String)> = content
                        .lines()
                        .enumerate()
                        .filter(|(_, line)| matches_line(line))
                        .map(|(idx, line)| (idx + 1, line.to_string()))
                        .collect();
                    if hits.is_empty() {
                        None
                    } else {
                        Some((rel_name, hits))
                    }
                })
                .collect();

            if !file_hits.is_empty() {
                let total_md_hits: usize = file_hits.iter().map(|(_, h)| h.len()).sum();
                out.push_str(&format!(
                    "=== [{bundle_name}] Markdown Sections (`sections/*.md`, {total_md_hits} hits across {} files) ===\n",
                    file_hits.len()
                ));
                let mut shown = 0usize;
                for (rel_name, hits) in &file_hits {
                    for (line_no, line_txt) in hits {
                        if shown >= limit {
                            break;
                        }
                        let formatted = if compact && line_txt.len() > 160 {
                            format!("{}...", &line_txt[..157])
                        } else {
                            line_txt.clone()
                        };
                        out.push_str(&format!("  sections/{rel_name}:{line_no}: {formatted}\n"));
                        shown += 1;
                    }
                }
                if total_md_hits > shown {
                    out.push_str(&format!(
                        "  ... +{} more section matches omitted (refine pattern or use `chipdoc section`).\n",
                        total_md_hits - shown
                    ));
                }
                out.push('\n');
            }
        }
    }

    if out.is_empty() {
        out.push_str(&format!("No matches found for '{pattern}'.\n"));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hw_hex_normalizes_c_and_vendor_formats() {
        assert_eq!(parse_hw_hex("0x41C00004"), Some(0x41C0_0004));
        assert_eq!(parse_hw_hex("41C00004h"), Some(0x41C0_0004));
        assert_eq!(parse_hw_hex("0x0400"), Some(0x400));
        assert_eq!(parse_hw_hex("400h"), Some(0x400));
        assert_eq!(parse_hw_hex("FSUB_0"), None);
    }

    #[test]
    fn test_normalized_hw_query_splits_peripheral_prefixes() {
        let q1 = NormalizedHwQuery::parse("GPIO.FSUB_0");
        assert_eq!(q1.peripheral_hint.as_deref(), Some("GPIO"));
        assert_eq!(q1.symbol, "FSUB_0");
        assert!(q1.explicit_sep);

        let q2 = NormalizedHwQuery::parse("GPIO_FSUB_0");
        assert_eq!(q2.peripheral_hint.as_deref(), Some("GPIO"));
        assert_eq!(q2.symbol, "FSUB_0");
        assert!(!q2.explicit_sep);

        let q3 = NormalizedHwQuery::parse("0x400");
        assert_eq!(q3.hex_value, Some(0x400));
    }

    #[test]
    fn test_hw_template_vs_concrete_instance_matching() {
        assert_eq!(hw_tokens_compatible("GPIOx_MODER", "GPIOA_MODER"), Some(3));
        assert_eq!(hw_tokens_compatible("GPIOx_MODER", "GPIO_MODER"), Some(3));
        assert_eq!(hw_tokens_compatible("USART_CR1", "USART1_CR1"), Some(3));
        assert_eq!(hw_tokens_compatible("DMA_CCRx", "DMA1_CCR1"), Some(3));
        assert_eq!(hw_tokens_compatible("TIM1", "TIM14"), None);
        assert_eq!(hw_tokens_compatible("TIM1_CR1", "TIM14_CR1"), None);
        assert_eq!(hw_tokens_compatible("CCR1", "CCR2"), None);
    }

    #[test]
    fn test_bounded_pin_and_underscore_bitfield_queries() {
        assert!(contains_hw_token_bounded("PA1", "PA1"));
        assert!(contains_hw_token_bounded("PA1_TX", "PA1"));
        assert!(!contains_hw_token_bounded("PA10", "PA1"));
        assert!(!contains_hw_token_bounded(
            "[Table 4-1] PA10 | TIM1_CH3",
            "PA1"
        ));

        let tmp = std::env::temp_dir().join(format!("chipdoc_query_test_{}", std::process::id()));
        let idx_dir = tmp.join("index");
        let _ = fs::create_dir_all(&idx_dir);

        let regs_tsv = "peripheral\tregister\toffset\treset\tbits\tfield\taccess\tfield_reset\tdescription\tlocation\tpage\n\
SYSCTL\tSWDCFG\t0x1040\t0x0\t1-0\tSWDP_MODE\tR/W\t0h\tSWD port mode\tsections/02-sysctl.md:40\t223\n\
FLASH\tFLASH_CR\t0x0014\t0xC0000000\t27\tOBL_LAUNCH\tR/W\t0\tOption byte launch\tsections/03-flash.md:80\t95\n\
FLASH\tFLASH_CR\t0x0014\t0xC0000000\t31\tLOCK\tR/W\t1\tFLASH_CR lock\tsections/03-flash.md:92\t96\n";
        fs::write(idx_dir.join("registers.tsv"), regs_tsv).expect("write registers.tsv");

        let xref_tsv = "symbol\tkind\tperipheral\tsummary\tlocation\tpage\n\
PA10\ttable_row\tIOMUX\t[Table 12] PA10 | USART1_RX\tsections/04-pin.md:20\t25\n\
PA1\ttable_row\tIOMUX\t[Table 12] PA1 | TIM2_CH2\tsections/04-pin.md:11\t25\n";
        fs::write(idx_dir.join("xref.tsv"), xref_tsv).expect("write xref.tsv");

        let bundles = vec![tmp.clone()];
        let out_swdp =
            query_registers_formatted(&bundles, "SWDP_MODE", 10, false).expect("query SWDP_MODE");
        assert!(out_swdp.contains("SYSCTL.SWDCFG"));
        assert!(out_swdp.contains("SWDP_MODE"));

        let out_obl =
            query_registers_formatted(&bundles, "OBL_LAUNCH", 10, false).expect("query OBL_LAUNCH");
        assert!(out_obl.contains("FLASH.FLASH_CR"));
        assert!(out_obl.contains("p. 95-96"));
        assert!(out_obl.contains("OBL_LAUNCH"));
        assert!(out_obl.contains("LOCK"));

        let out_pa1 = query_xref_formatted(&bundles, "PA1", 10).expect("query xref PA1");
        assert!(out_pa1.contains("PA1"));
        assert!(!out_pa1.contains("PA10"));

        let _ = fs::remove_dir_all(&tmp);
    }
}
