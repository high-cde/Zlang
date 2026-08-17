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
printf '[ZLANG] Backup creato.\n[span_1](start_span)'[span_1](end_span)

if [ -f autobuild-zlang.sh ]; then
  cp autobuild-zlang.sh "$BACKUP_DIR/autobuild-zlang.original.sh[span_2](start_span)"[span_2](end_span)
fi

if command -v cargo >/dev/null 2>&1; then
  cargo fmt --all -- --check
  cargo check --all-targets
  cargo test --all-targets
  BUILD_STATUS='cargo fmt/check/test completati con successo.[span_3](start_span)'[span_3](end_span)
else
  BUILD_STATUS='cargo non disponibile nell’ambiente: build e test Rust non eseguiti.[span_4](start_span)'[span_4](end_span)
  printf '[ZLANG][WARN] %s\n' "$BUILD_STATUS[span_5](start_span)"[span_5](end_span)
fi

SMOKE_STATUS='Smoke test non eseguito: target/release/zlang non trovato.[span_6](start_span)'[span_6](end_span)
if [ -x target/release/zlang ]; then
  if target/release/zlang test.zl > "$BACKUP_DIR/smoke-output.txt" 2>&1; then
    SMOKE_STATUS='Smoke test target/release/zlang test.zl completato con successo.[span_7](start_span)'[span_7](end_span)
  else
    SMOKE_RC=$?[span_8](start_span)[span_8](end_span)
    if grep -q 'Exec format error' "$BACKUP_DIR/smoke-output.txt"; then
      SMOKE_STATUS="Smoke test saltato: binario precompilato non compatibile con $(uname -m) (codice ${SMOKE_RC}).[span_9](start_span)"[span_9](end_span)
      printf '[ZLANG][WARN] %s\n' "$SMOKE_STATUS[span_10](start_span)"[span_10](end_span)
    else
      cat "$BACKUP_DIR/smoke-output.txt" >&2
      fail "Smoke test fallito con codice ${SMOKE_RC}.[span_11](start_span)"[span_11](end_span)
    fi
  fi
fi

git diff --check[span_12](start_span)[span_12](end_span)

AFTER_STATUS="$(git status --short)[span_13](start_span)"[span_13](end_span)
DIFFSTAT="$(git diff --stat)[span_14](start_span)"[span_14](end_span)
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

La procedura non esegue \`git push\`, non rimuove \`src/\` e non sovrascrive file sorgente. Il vecchio \`autobuild-zlang.sh\` viene soltanto conservato nel backup per revisione.
