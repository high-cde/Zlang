# Audit finale del codice sorgente ZLang

**Data:** 17 agosto 2026
**Ambito:** sorgenti Rust, entry point, script di build, test, artefatti e coerenza repository
**Modifiche al codice durante l’audit:** nessuna

## Esito sintetico

Il repository è in uno stato pubblicabile come **prototipo documentato**, ma non ancora come runtime di produzione. Il codice sorgente attivo è compatto e leggibile, tuttavia la struttura contiene più percorsi paralleli e molti moduli placeholder vuoti. La specifica documentale è significativamente più ampia dell’implementazione effettivamente collegata al binario principale.

| Severità | Risultato |
|---|---|
| Critica | Nessuna vulnerabilità remota dimostrata nell’audit statico |
| Alta | `autobuild-zlang.sh` è distruttivo: cancella `src/` e termina con `git push` |
| Alta | Nessun test Rust/CI rilevato; `cargo` non è disponibile nell’ambiente |
| Media | 274 file generati sotto `target/` risultano tracciati nel repository |
| Media | Parser e VM attivi usano `panic`, `unwrap` e `expect` su input o runtime |
| Media | Esistono due architetture sorgente parallele e moduli vuoti |
| Bassa | `src/api.rs` è esplicitamente disabilitato e gli esempi chain contengono placeholder |

## 1. Percorso esecutivo attivo

Il binario Cargo principale è definito in `src/main.rs` e importa i moduli esposti da `src/lib.rs`. Il percorso attivo è sostanzialmente:

```text
file sorgente
  → src/compiler::Compiler
  → Vec<String> di pseudo-istruzioni
  → src::vm::ZVirtualMachine
  → output o primitive simulate
```

Il compilatore attivo riconosce principalmente `emit`, `orbit_sync` e `zchain`. La VM interpreta istruzioni stringa come `PRINT_STDOUT`, `SPACEX_LEO_HANDSHAKE` e `ZCHAIN_SIGN`. Questo è sufficiente per una demo del flusso sorgente-runtime, ma non equivale ancora alla pipeline bytecode binario documentata.

## 2. Divergenza architetturale

Sono presenti directory di primo livello (`compiler/`, `vm/`, `zpm/`, `cli/`) e moduli analoghi sotto `src/`. I file di primo livello risultano in gran parte vuoti o non collegati al binario principale, mentre il percorso compilato utilizza soprattutto `src/`.

Questa è la principale priorità architetturale: scegliere un solo percorso canonico e collegare in modo verificabile lexer, parser, AST, type checker, code generator, bytecode e VM.

## 3. Rischi tecnici rilevati

### 3.1 Script di autobuild distruttivo

`autobuild-zlang.sh` contiene `rm -rf src`, ricrea i sorgenti da heredoc e termina con `git push -u origin main`. Non deve essere eseguito in un working tree contenente modifiche senza backup e revisione. Lo script sicuro `one-shot-zlang.sh` creato nel repository non cancella `src/` e non esegue push automatici.

### 3.2 Error handling non robusto

Il lexer usa `unwrap()` durante il parsing numerico. `src/main.rs` usa `expect()` nella lettura del file. Il parser termina il processo con `std::process::exit(1)` in caso di errore. La VM e lo script storico usano ulteriori `unwrap()` per variabili mancanti, conversioni numeriche e divisione.

Per un runtime di produzione, questi casi devono diventare errori strutturati con posizione nel sorgente, categoria, messaggio e codice d’errore. La divisione per zero, l’identificatore sconosciuto e il tipo incompatibile non dovrebbero causare panic o terminazioni non tipizzate.

### 3.3 Syscall e integrazioni simulate

`src/api.rs` restituisce `http disabled`. La firma Z-Chain del modulo runtime stampa messaggi simulati e non dimostra una transazione reale. L’handshake orbitale è una pseudo-istruzione dimostrativa. Queste funzionalità devono essere descritte pubblicamente come mock, demo o placeholder fino a quando non esista un’integrazione reale testata.

### 3.4 Test e CI

Non sono state rilevate directory `tests/` o workflow CI; è presente soltanto `test.zl`. Nell’ambiente di audit `cargo` e `rustc` non sono installati, quindi non è stato possibile eseguire `cargo fmt`, `cargo check` o `cargo test`. Il binario precompilato sotto `target/release/zlang` è ARM64, mentre l’ambiente è x86_64, e non può essere eseguito localmente.

## 4. Artefatti generati tracciati

Il repository traccia 274 file sotto `target/`, inclusi binari, fingerprint e artefatti di compilazione. Questi file dovrebbero essere rimossi dal versionamento e coperti da `.gitignore`; la patch pubblicata ha aggiunto `.gitignore`, ma è necessaria una pulizia storica separata con `git rm -r --cached target` prima di una futura release pulita.

## 5. Raccomandazioni prima della produzione

| Priorità | Azione |
|---|---|
| P0 | Disabilitare o riscrivere definitivamente l’autobuild distruttivo |
| P0 | Rimuovere `target/` dal tracking Git e mantenere soltanto sorgenti e lockfile |
| P1 | Scegliere un’unica architettura tra `src/` e directory di primo livello |
| P1 | Aggiungere test unitari per lexer, parser, compilatore e VM |
| P1 | Introdurre CI con format, clippy, check, test e build release |
| P1 | Sostituire `panic`, `unwrap` ed `expect` con errori strutturati |
| P2 | Stabilizzare il formato bytecode e l’ABI delle syscall |
| P2 | Separare chiaramente mock/demo da integrazioni operative |
| P2 | Aggiungere test cross-platform per Linux x86-64 e ARM64 |

## Verdetto finale

Il codice sorgente è sufficiente per sostenere una narrazione di **prototipo avanzato e direzione architetturale**, non ancora una dichiarazione di runtime completo o production-ready. Il README e la whitepaper aggiornati sono coerenti con questa posizione trasparente.

La priorità assoluta non è aggiungere nuove feature dimostrative, ma consolidare il percorso esecutivo, la gestione degli errori, la CI e la separazione tra sorgente, artefatti generati e mock d’integrazione.
