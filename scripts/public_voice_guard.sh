#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

mapfile -t files < <(
  git ls-files --cached --others --exclude-standard |
    grep -E '^(Cargo\.toml|LICENSE|README\.md|\.github/workflows/|crates/|scripts/)' |
    grep -v '^scripts/public_voice_guard\.sh$'
)

if [ "${#files[@]}" -eq 0 ]; then
  exit 0
fi

patterns=(
  'p[i]lls?'
  'Q5L R-[0-9]+'
  'projects/'"b-suite/"
  'hold'"ing/"
  'frame'"works/"
  'B:[0-9]+'
  'I[0-9]+-I[0-9]+'
  'GOV-'
  'DECISION [0-9]+'
  'implementation-'"open gate"
  '0\.1\.0-'"skeleton"
  'M1-Mz'
  'PENDING-'"OPENEVOLVE-RUN"
  'PENDING-'"FIRST-CONTRIBOT-CYCLE"
  'SCOPE-BNPM-'
  'DOMAIN-'
  'IP-TRANS-'
  'Co-Authored-By:'
  $'—'
)

failed=0
for pattern in "${patterns[@]}"; do
  if grep -InE -- "$pattern" "${files[@]}"; then
    failed=1
  fi
done

if grep -RInE -- 'OffBrandVoiceCreep|BannedRegressionVocab' crates/bratch/src crates/bratch/tests; then
  failed=1
fi

canonical_description='Prompt lookup tool. Agent names a regression signature from a fixed list; bratch returns the prompt for that regression signature. The prompt tells the agent how to check the diff against a baseline for that regression.'

if ! grep -Fq "$canonical_description" crates/bratch/Cargo.toml README.md; then
  echo "public description missing from Cargo.toml or README.md" >&2
  failed=1
fi

if [ "$failed" -ne 0 ]; then
  echo "public voice guard failed" >&2
  exit 1
fi
