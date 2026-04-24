#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
input="$repo_root/release/public-portfolio-summary.md"
header="$repo_root/release/public-portfolio-summary-pdf-header.tex"
output="$repo_root/release/public-portfolio-summary.pdf"
tmpdir="$(mktemp -d)"
body="$tmpdir/public-portfolio-summary-body.md"
export XDG_CACHE_HOME="$tmpdir/cache"

trap 'rm -rf "$tmpdir"' EXIT

awk 'BEGIN{removed=0} NR==1 && $0 ~ /^# / {next} {print}' "$input" > "$body"

pandoc "$body" \
  --from markdown \
  --to pdf \
  --pdf-engine=xelatex \
  --metadata title="CSSI Pulse Guard Public Portfolio Summary" \
  --metadata author="Common Sense Systems, Inc." \
  --metadata date="$(date +%Y-%m-%d)" \
  --variable geometry:margin=0.85in \
  --variable fontsize=11pt \
  --variable colorlinks=true \
  --variable linkcolor=PulseNavy \
  --variable urlcolor=PulseNavy \
  --include-in-header="$header" \
  --output "$output"
