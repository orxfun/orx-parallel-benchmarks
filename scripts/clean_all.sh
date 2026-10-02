#!/usr/bin/env bash
set -euo pipefail

dry_run=false
case "${1:-}" in
    --dry-run) dry_run=true ;;
    "") ;;
    *) printf 'Usage: %s [--dry-run]\n' "$0" >&2; exit 1 ;;
esac
if (( $# > 1 )); then
    printf 'Usage: %s [--dry-run]\n' "$0" >&2
    exit 1
fi

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

while IFS= read -r -d '' artifact; do
    if [[ "$dry_run" == true ]]; then
        printf 'Would remove: %s\n' "${artifact#"$repo_root/"}"
    else
        printf 'Removing: %s\n' "${artifact#"$repo_root/"}"
        rm -rf -- "$artifact"
    fi
done < <(find "$repo_root" \
    -type d -name .git -prune -o \
    -type d -name target -prune -print0)