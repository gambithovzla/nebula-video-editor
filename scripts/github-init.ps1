# Creates the GitHub repo and pushes this folder. Run once after: gh auth login
$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

gh auth status | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Error "Run 'gh auth login' first, then re-run this script."
    exit 1
}

if (git remote get-url origin 2>$null) {
    Write-Host "Remote 'origin' already exists. Pushing..."
    git push -u origin main
    exit $LASTEXITCODE
}

gh repo create nebula-video-editor `
    --public `
    --description "Nebula: GPU-first multilayer video editor (Rust + Tauri) — early scaffold" `
    --source . `
    --remote origin `
    --push

Write-Host "Done. Set Cargo.toml [workspace.package] repository to your new URL if needed."
