#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
input="$repo_root/release/public-portfolio-summary.md"
output="$repo_root/release/public-portfolio-summary.pdf"

pandoc "$input" \
  --from markdown \
  --to pdf \
  --pdf-engine=xelatex \
  --metadata title="CSSI Pulse Guard Public Portfolio Summary" \
  --metadata author="Common Sense Systems, Inc." \
  --metadata date="$(date +%Y-%m-%d)" \
  --variable geometry:margin=1in \
  --variable colorlinks=true \
  --variable linkcolor=blue \
  --variable urlcolor=blue \
  --output "$output"
