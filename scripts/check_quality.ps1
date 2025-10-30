#!/usr/bin/env pwsh

# Script de vérification de qualité Rust
# Vérifie TOUTES les exigences: format, warnings, tests, clippy

Write-Host "🔍 RUST QUALITY CHECK - ZERO TOLERANCE" -ForegroundColor Yellow
Write-Host "=======================================" -ForegroundColor Yellow

$ErrorCount = 0

# 1. Formatting check
Write-Host "`n📐 Checking code formatting..." -ForegroundColor Cyan
$FormatResult = cargo fmt --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Code formatting FAILED!" -ForegroundColor Red
    Write-Host "Run: cargo fmt" -ForegroundColor Yellow
    $ErrorCount++
} else {
    Write-Host "✅ Code formatting PASSED" -ForegroundColor Green
}

# 2. Clippy check (warnings as errors)
Write-Host "`n🔧 Running Clippy (warnings as errors)..." -ForegroundColor Cyan
$ClippyResult = cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy FAILED!" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✅ Clippy PASSED" -ForegroundColor Green
}

# 3. Build check (dev)
Write-Host "`n🔨 Building in dev mode..." -ForegroundColor Cyan
$BuildResult = cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Dev build FAILED!" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✅ Dev build PASSED" -ForegroundColor Green
}

# 4. Build check (release)
Write-Host "`n🚀 Building in release mode..." -ForegroundColor Cyan
$ReleaseBuildResult = cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Release build FAILED!" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✅ Release build PASSED" -ForegroundColor Green
}

# 5. Unit tests
Write-Host "`n🧪 Running unit tests..." -ForegroundColor Cyan
$TestResult = cargo test --lib
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Unit tests FAILED!" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✅ Unit tests PASSED" -ForegroundColor Green
}

# 6. Integration tests
Write-Host "`n🔗 Running integration tests..." -ForegroundColor Cyan
$IntegrationResult = cargo test --tests
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Integration tests FAILED!" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✅ Integration tests PASSED" -ForegroundColor Green
}

# 7. Doc tests
Write-Host "`n📚 Running doc tests..." -ForegroundColor Cyan
$DocTestResult = cargo test --doc
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Doc tests FAILED!" -ForegroundColor Red
    $ErrorCount++
} else {
    Write-Host "✅ Doc tests PASSED" -ForegroundColor Green
}

# 8. Check for ignored tests
Write-Host "`n⚠️  Checking for ignored tests..." -ForegroundColor Cyan
$IgnoredTestsOutput = cargo test -- --ignored 2>&1
$IgnoredTests = $IgnoredTestsOutput | Select-String "([1-9]\d*) ignored"
if ($IgnoredTests) {
    Write-Host "❌ IGNORED TESTS FOUND!" -ForegroundColor Red
    Write-Host $IgnoredTests -ForegroundColor Yellow
    $ErrorCount++
} else {
    Write-Host "✅ No ignored tests" -ForegroundColor Green
}

# Final result
Write-Host "`n" -NoNewline
Write-Host "===============================================" -ForegroundColor Yellow
if ($ErrorCount -eq 0) {
    Write-Host "🎉 ALL QUALITY CHECKS PASSED! PROJECT IS CLEAN!" -ForegroundColor Green
    Write-Host "✅ Zero warnings, zero ignored tests, all builds successful" -ForegroundColor Green
    exit 0
} else {
    Write-Host "💥 QUALITY CHECK FAILED! $ErrorCount errors found" -ForegroundColor Red
    Write-Host "❌ Fix all issues before proceeding" -ForegroundColor Red
    exit 1
}