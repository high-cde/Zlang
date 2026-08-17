# ZLang v2026.2.0 — Verifica di compilazione

**Data:** 17 agosto 2026
**Piattaforma di verifica:** Linux x86-64
**Toolchain:** Rust 1.75.0, Cargo 1.75.0

## Esito

La compilazione da sorgente di ZLang è stata verificata con successo. La verifica è stata effettuata dopo `cargo clean`, quindi senza riutilizzare artefatti di build precedenti.

| Controllo | Esito |
|---|---|
| Formattazione Rust | Superata con `cargo fmt --all -- --check` |
| Controllo di compilazione | Superato con `cargo check --all-targets` |
| Test automatici | Superati: 4/4 con `cargo test --all-targets` |
| Build ottimizzata | Superata con `cargo build --release` |
| Smoke test CLI | Superato con output, handshake LEO e primitive Z-Chain dimostrative |
| Artefatti legacy ARM | Rimossi dal tracking Git; non più usati per validare la build |

## Binario verificato

| Proprietà | Valore |
|---|---|
| Percorso | `target/release/zlang` |
| Formato | ELF 64-bit PIE, x86-64, GNU/Linux |
| Dimensione | circa 13 MB |
| SHA-256 | `32f54ec7e042b69d2a4182b96733f38ceb6601eee7d153dfe0df517e59dcd96b` |

## Correzioni applicate

Il `Cargo.lock` è stato rigenerato in un formato compatibile con la toolchain Rust disponibile. Il codice sorgente attivo è stato formattato e `src/lib.rs` ora esporta `lexer` e `parser`, rendendo il percorso sorgente utilizzabile dai test di integrazione.

È stato aggiunto `tests/compiler_pipeline.rs`, che verifica tokenizzazione, parsing, compilazione delle istruzioni supportate e l’esecuzione end-to-end della CLI. È stata inoltre aggiunta una pipeline GitHub Actions in `.github/workflows/ci.yml` per eseguire format, check, test e Clippy nelle modifiche future.

Gli artefatti sotto `target/`, in precedenza tracciati nel repository, sono stati rimossi dall’indice Git mediante `git rm -r --cached target`. Il contenuto resta disponibile localmente per il binario compilato, ma non verrà più incluso nei commit futuri grazie a `.gitignore`.

## Comandi riproducibili

```bash
cargo fmt --all -- --check
cargo check --all-targets
cargo test --all-targets
cargo build --release

printf 'emit smoke\norbit_sync\nzchain verified\n' > /tmp/zlang-smoke.zl
./target/release/zlang /tmp/zlang-smoke.zl
```

## Limiti noti

La build è verificata per il percorso sorgente attivo e per le quattro funzionalità coperte dai test. Il runtime resta un prototipo: il compilatore attivo genera pseudo-istruzioni stringa e diverse funzionalità presenti nella specifica completa — bytecode binario, ABI syscall stabilizzato, type checker completo e integrazioni reali — richiedono ulteriore implementazione.

## Stato per il commit

Le modifiche di compilazione, test, CI e pulizia degli artefatti sono presenti localmente e non sono state inviate automaticamente al repository remoto. Prima del commit finale è opportuno includere soltanto i file tecnici pertinenti e tenere separati gli articoli o draft editoriali non richiesti dalla build.
