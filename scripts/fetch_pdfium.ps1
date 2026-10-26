#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Stages the PDFium shared library into ./bin/ for chipdoc.
.DESCRIPTION
    Checks ./bin/pdfium.dll first, then checks local Python pypdfium2 caches if present,
    and otherwise downloads the latest bblanchon/pdfium-binaries release for the current OS/arch.
#>

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
$BinDir = Join-Path $RepoRoot "bin"
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null

if ($IsWindows -or $env:OS -eq "Windows_NT") {
    $TargetLib = Join-Path $BinDir "pdfium.dll"
    if (Test-Path $TargetLib) {
        Write-Host "[ok] PDFium library already present at $TargetLib"
        exit 0
    }

    # Check common local pypdfium2 cache paths first for instant offline staging
    $LocalCandidates = @(
        "D:\projects\docling\venv\Lib\site-packages\pypdfium2_raw\pdfium.dll"
    )
    foreach ($Candidate in $LocalCandidates) {
        if (Test-Path $Candidate) {
            Copy-Item -Path $Candidate -Destination $TargetLib -Force
            Write-Host "[ok] Copied local pdfium.dll from $Candidate -> $TargetLib"
            exit 0
        }
    }

    $ArchiveUrl = "https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-win-x64.tgz"
    $TempTgz = Join-Path ([System.IO.Path]::GetTempPath()) "pdfium-win-x64.tgz"
    $TempExtract = Join-Path ([System.IO.Path]::GetTempPath()) "pdfium-extract"
    New-Item -ItemType Directory -Force -Path $TempExtract | Out-Null

    Write-Host "Downloading $ArchiveUrl ..."
    curl.exe -sSL $ArchiveUrl -o $TempTgz
    tar -xzf $TempTgz -C $TempExtract
    Copy-Item -Path (Join-Path $TempExtract "bin\pdfium.dll") -Destination $TargetLib -Force
    Write-Host "[ok] Installed pdfium.dll -> $TargetLib"
} else {
    $TargetLib = Join-Path $BinDir "libpdfium.so"
    if (Test-Path $TargetLib) {
        Write-Host "[ok] PDFium library already present at $TargetLib"
        exit 0
    }

    $ArchiveUrl = "https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz"
    $TempTgz = "/tmp/pdfium-linux-x64.tgz"
    $TempExtract = "/tmp/pdfium-extract"
    New-Item -ItemType Directory -Force -Path $TempExtract | Out-Null

    Write-Host "Downloading $ArchiveUrl ..."
    curl -sSL $ArchiveUrl -o $TempTgz
    tar -xzf $TempTgz -C $TempExtract
    Copy-Item -Path "$TempExtract/lib/libpdfium.so" -Destination $TargetLib -Force
    Write-Host "[ok] Installed libpdfium.so -> $TargetLib"
}
