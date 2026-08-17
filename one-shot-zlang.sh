#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
BACKUP_DIR="${ROOT}/.zlang-backups/${STAMP}"
REPORT="${ROOT}/one-shot-report-${STAMP}.md"

fail() {
  printf '[ZLANG][ERROR] %s\n' "$*" >&2
  exit 1
}

trap 'printf "[ZLANG][ERROR] procedura interrotta alla riga %s\n" "$LINENO" >&2' ERR

printf '[ZLANG] Repository: %s\n' "$ROOT"
printf '[ZLANG] Backup: %s\n' "$BACKUP_DIR"

[ -f Cargo.toml ] || fail 'Cargo.toml non trovato.'
[ -d .git ] || fail 'Repository Git non trovato.'

mkdir -p "$BACKUP_DIR"
git status --short > "$BACKUP_DIR/status-before.txt"
git diff --binary > "$BACKUP_DIR/working-tree.patch" || true
git ls-files -z | tar --null -T - -czf "$BACKUP_DIR/tracked-files.tar.gz"
printf '[ZLANG] Backup creato.\n'

if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all -- --check
  cargo check --all-targets
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --all-targets
  cargo build --release
  BUILD_STATUS='cargo fmt/check/clippy/test/build release completati con successo.'
else
  BUILD_STATUS='cargo non disponibile nell’ambiente: verifiche Rust e build release non eseguite.'
  printf '[ZLANG][WARN] %s\n' "$BUILD_STATUS"
fi

SMOKE_STATUS='Smoke test non eseguito: target/release/zlang non trovato.'
if [ -x target/release/zlang ]; then
  if target/release/zlang test.zl > "$BACKUP_DIR/smoke-output.txt" 2>&1; then
    SMOKE_STATUS='Smoke test target/release/zlang test.zl completato con successo.'
  else
    SMOKE_RC=$?
    if grep -q 'Exec format error' "$BACKUP_DIR/smoke-output.txt"; then
      SMOKE_STATUS="Smoke test saltato: binario precompilato non compatibile con $(uname -m) (codice ${SMOKE_RC})."
      printf '[ZLANG][WARN] %s\n' "$SMOKE_STATUS"
    else
      cat "$BACKUP_DIR/smoke-output.txt" >&2
      fail "Smoke test fallito con codice ${SMOKE_RC}."
    fi
  fi
fi

git diff --check

AFTER_STATUS="$(git status --short)"
DIFFSTAT="$(git diff --stat)"
cat > "$REPORT" <<EOF
# Report one-shot ZLang

- **Data UTC:** ${STAMP}
- **Repository:** ${ROOT}
- **Backup:** ${BACKUP_DIR}
- **Verifica Rust:** ${BUILD_STATUS}
- **Smoke test:** ${SMOKE_STATUS}

## Stato Git dopo la procedura

\`\`\`
${AFTER_STATUS:-working tree pulito}
\`\`\`

## Diff statistico

\`\`\`
${DIFFSTAT:-nessuna modifica non committata}
\`\`\`

## Sicurezza

La procedura non esegue \`git push\`, non rimuove sorgenti e non sovrascrive file del progetto.
EOF

printf '[ZLANG] Controlli completati. Report: %s\n' "$REPORT"
printf '[ZLANG] Nessun push remoto eseguito.\n'
