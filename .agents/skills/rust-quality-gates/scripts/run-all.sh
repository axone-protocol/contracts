#!/usr/bin/env bash
set -euo pipefail

tasks=(format lint test test-coverage schema docs)

if [ "$#" -gt 0 ]; then
  tasks=("$@")
fi

for task in "${tasks[@]}"; do
  echo "==> cargo make ${task}"
  cargo make "${task}"
done
