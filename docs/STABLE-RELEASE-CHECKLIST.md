# ZLang — Checklist per la prima release stabile

**Obiettivo proposto:** `v1.0.0`
**Stato corrente:** prototipo avanzato con build Rust riproducibile, test di integrazione iniziali e percorso esecutivo limitato.
**Regola di rilascio:** una voce contrassegnata come **bloccante** deve essere completata, verificata e documentata prima del tag stabile.

> Una release stabile non è una promessa sul numero di feature. È un contratto sulla prevedibilità: ciò che viene dichiarato deve compilare, essere testato, gestire gli errori in modo definito e restare compatibile entro le regole pubblicate.

## 1. Perimetro della prima release stabile

La prima release stabile non deve tentare di implementare l’intera visione descritta dalla whitepaper. Deve invece dichiarare e stabilizzare un **core minimo**, coerente e utilizzabile.

| Area | Impegno per `v1.0.0` | Stato richiesto |
|---|---|---|
| Runtime | Esecuzione deterministica degli script supportati | Bloccante |
| Sintassi | Sottoinsieme sintattico formalmente definito | Bloccante |
| Compilatore | Trasformazione sorgente → istruzioni/bytecode con errori strutturati | Bloccante |
| VM | Semantica documentata per ogni istruzione supportata | Bloccante |
| CLI | Interfaccia `zlang` stabile, help e codici di uscita | Bloccante |
| Sicurezza | Nessuna syscall privilegiata non governata | Bloccante |
| Distribuzione | Build Linux x86-64 riproducibile e verifica checksum | Bloccante |
| Architetture aggiuntive | ARM64/Termux | Obiettivo post-1.0, salvo test completi |
| Blockchain e rete | Primitive sperimentali | Fuori dal core stabile, salvo implementazione reale |

La documentazione pubblica deve distinguere esplicitamente il **core stabile** dalle funzionalità sperimentali o progettuali. Termini come “Z-Chain”, “orbitale”, networking o registry non devono essere annunciati come operativi finché non esistono implementazione, test e gestione degli errori verificabili.

## 2. Gate P0 — Architettura e contratto del linguaggio

| ID | Requisito bloccante | Evidenza di chiusura |
|---|---|---|
| P0-01 | Unico percorso sorgente canonico sotto `src/` | Nessun modulo Rust vuoto o duplicato fuori dal percorso attivo |
| P0-02 | Semver e policy di compatibilità pubblicati | Sezione README e file `CHANGELOG.md` aggiornati |
| P0-03 | Grammatica del core stabile allineata all’implementazione | `docs/language-spec.md` segnala in modo netto ciò che è supportato |
| P0-04 | Ogni istruzione runtime ha semantica, input, output ed errore definiti | Tabella opcode/instruction + test corrispondenti |
| P0-05 | Funzionalità non implementate non vengono pubblicizzate come disponibili | Revisione README, whitepaper, esempi e release notes |
| P0-06 | `Cargo.toml` contiene descrizione, repository, edition, license o campo esplicito di licenza da definire | Manifest validato con `cargo package --allow-dirty` |

## 3. Gate P0 — Gestione degli errori e robustezza

Il runtime stabile non deve terminare il processo con `panic!`, `unwrap()` o `expect()` in risposta a input utente, file non disponibili, identificatori sconosciuti, conversioni invalide o divisioni per zero.

| ID | Requisito bloccante | Evidenza di chiusura |
|---|---|---|
| P0-10 | Tipo `ZlangError` o equivalente per lexer, parser, compilatore, VM e I/O | Errori tipizzati con messaggio e categoria |
| P0-11 | Posizione nel sorgente per gli errori di parsing | Test con riga, colonna e messaggio atteso |
| P0-12 | Codici di uscita CLI documentati | Test di processo per successo, errore sorgente e errore I/O |
| P0-13 | Divisone per zero, variabili assenti e istruzioni sconosciute non causano panic | Test negativi dedicati |
| P0-14 | Nessuna capacità di sistema reale è simulata come successo | Mock, demo e funzionalità disabilitate restituiscono stato esplicito |

## 4. Gate P0 — Qualità, test e CI

| ID | Requisito bloccante | Evidenza di chiusura |
|---|---|---|
| P0-20 | `cargo fmt --all -- --check` passa | Workflow CI verde |
| P0-21 | `cargo check --all-targets` passa | Workflow CI verde |
| P0-22 | `cargo test --all-targets` passa | Workflow CI verde |
| P0-23 | `cargo clippy --all-targets --all-features -- -D warnings` passa | Workflow CI verde |
| P0-24 | Copertura del core >= 80% per lexer, parser, compiler e VM | Report di coverage archiviato nella release candidate |
| P0-25 | Test end-to-end della CLI | Script validi, invalidi, file assente e codici di uscita |
| P0-26 | Ogni esempio nel repository è eseguibile o chiaramente classificato come pseudocodice | Job CI dedicato agli esempi |
| P0-27 | Test di regressione per ogni bug corretto prima di `v1.0.0` | Riferimento issue → test nel PR o commit |

## 5. Gate P0 — Bytecode e VM

Il bytecode è stabile soltanto quando un artefatto prodotto da una versione compatibile del compilatore può essere riconosciuto, validato ed eseguito o rifiutato con un errore chiaro dal runtime.

| ID | Requisito bloccante | Evidenza di chiusura |
|---|---|---|
| P0-30 | Header bytecode con magic number, versione e integrità | Test di file valido, corrotto e incompatibile |
| P0-31 | Specifica degli opcode completata per il core | Documento versione `1.0` e test per opcode |
| P0-32 | Serializzazione/deserializzazione deterministica | Golden tests su bytecode noto |
| P0-33 | VM con limiti espliciti di stack, istruzioni e memoria | Test di superamento limiti e messaggi controllati |
| P0-34 | Compatibilità bytecode dichiarata | Regole SemVer e matrice compiler/runtime |
| P0-35 | Nessuna istruzione sconosciuta viene ignorata silenziosamente | Errore runtime verificabile |

## 6. Gate P0 — Sicurezza e capacità

La prima release stabile può essere priva di syscall privilegiate reali, ma non può esporle senza un modello di autorizzazione.

| ID | Requisito bloccante | Evidenza di chiusura |
|---|---|---|
| P0-40 | Classificazione delle syscall: core, sperimentale, disabilitata | Documento ABI e stato di ogni syscall |
| P0-41 | Capability manifest per ogni azione privilegiata implementata | Test di autorizzazione e negazione |
| P0-42 | Nessuna esecuzione arbitraria di shell o rete da input non fidato | Review di sicurezza e test negativi |
| P0-43 | Logging di audit per syscall abilitate | Evento con script, versione, richiesta ed esito |
| P0-44 | Policy di disclosure delle vulnerabilità | `SECURITY.md` verificato e contatto di sicurezza attivo |
| P0-45 | Dipendenze analizzate per vulnerabilità note | `cargo audit` o alternativa documentata in CI/release |

## 7. Gate P1 — Distribuzione e compatibilità

| ID | Requisito | Evidenza di chiusura |
|---|---|---|
| P1-01 | Build release Linux x86-64 | Binario, SHA-256 e istruzioni di verifica |
| P1-02 | Build ARM64 | CI cross-build + smoke test su runner o hardware reale |
| P1-03 | Pacchetto tarball con binario e documentazione | `zlang-v1.0.0-<target>.tar.gz` firmato o con checksum |
| P1-04 | SBOM o elenco dipendenze della release | File SPDX/CycloneDX o equivalente |
| P1-05 | Riproducibilità della build | Due build pulite con artefatti identici o differenze motivate |

La release `v1.0.0` deve includere almeno Linux x86-64. ARM64 può essere presentato come experimental finché non esistono build, test e supporto operativo verificabili.

## 8. Gate P1 — Documentazione e esperienza sviluppatore

| ID | Requisito | Evidenza di chiusura |
|---|---|---|
| P1-10 | README con quick start che funziona da clone pulito | Test manuale o CI documentale |
| P1-11 | Tutorial “primo script” | Script incluso ed eseguito in CI |
| P1-12 | Riferimento CLI | Opzioni, codici d’uscita, esempi e compatibilità |
| P1-13 | Riferimento linguaggio aggiornato | Nessuna feature non implementata dichiarata come stabile |
| P1-14 | Riferimento bytecode e VM | Versione, opcode, errori e compatibilità |
| P1-15 | Guida migrazione da release pre-1.0 | Rotture note e istruzioni di aggiornamento |
| P1-16 | CHANGELOG curato | Sezione Added, Changed, Fixed, Security e Known limitations |
| P1-17 | Licenza esplicita | File `LICENSE` presente e compatibile con la distribuzione desiderata |

## 9. Piano temporale suggerito

Il piano usa settimane di lavoro effettivo e deve essere adattato alla disponibilità del team. Non rilasciare `v1.0.0` se un gate P0 resta aperto.

| Finestra | Obiettivo | Output |
|---|---|---|
| Settimane 1–2 | Definizione core stabile | Scope firmato, grammatica ridotta, backlog P0/P1 |
| Settimane 3–5 | Error handling e CLI | Errori tipizzati, codici uscita, test negativi |
| Settimane 6–8 | Bytecode/VM | Header, opcode core, limiti runtime, golden tests |
| Settimane 9–10 | Sicurezza | Capability model, ABI syscall, audit, dependency review |
| Settimane 11–12 | Compatibilità e packaging | Build x86-64, checksum, tarball, release candidate `v1.0.0-rc.1` |
| Settimane 13–14 | Hardening | Bug bash, regressioni, feedback utenti, `rc.2` se necessario |
| Settimana 15 | Go/no-go | Verifica checklist, firma artefatti e decisione di release |
| Settimana 16 | Lancio stabile | Tag `v1.0.0`, GitHub Release, changelog e comunicazione |

## 10. Processo release candidate

Ogni release candidate deve essere costruita da una commit SHA immutabile e non da una working tree locale.

1. Bloccare nuove feature e accettare soltanto fix P0/P1.
2. Eseguire tutti i controlli CI su Linux x86-64 e, se dichiarato, ARM64.
3. Generare artefatti, checksum e changelog dalla commit candidata.
4. Eseguire smoke test manuale su ambiente pulito.
5. Aprire una finestra di feedback con issue label `release-candidate`.
6. Registrare bug, rischio, owner e decisione per ogni gate aperto.
7. Pubblicare `v1.0.0-rc.N` esclusivamente come prerelease GitHub.
8. Promuovere la stessa commit a `v1.0.0` soltanto se tutti i P0 sono chiusi.

## 11. Go / No-Go finale

La decisione deve essere presa in una review esplicita, non implicita in un push. Il responsabile della release firma la tabella seguente.

| Domanda | Condizione per GO |
|---|---|
| Il core dichiarato è implementato? | Tutti i P0-01…P0-06 chiusi |
| Gli errori sono controllati? | Tutti i P0-10…P0-14 chiusi |
| Build e test sono verdi? | Tutti i P0-20…P0-27 chiusi |
| Bytecode e VM sono versionati? | Tutti i P0-30…P0-35 chiusi |
| Le capacità sono sicure? | Tutti i P0-40…P0-45 chiusi |
| La distribuzione è verificabile? | P1-01 e P1-03 chiusi; P1-02 solo se ARM64 dichiarato stabile |
| La documentazione è onesta e completa? | P1-10…P1-17 chiusi o limitazioni dichiarate |
| Sono presenti blocker aperti? | Nessun blocker critico o alto non accettato formalmente |

> **Decisione:** se una risposta è “no”, la release resta una release candidate o una versione pre-stabile. La stabilità non si negozia con una data di marketing.

## 12. Checklist operativa per il giorno del lancio

- [ ] Branch `main` verde e protetto.
- [ ] Tag annotato e immutabile `v1.0.0` creato dalla commit approvata.
- [ ] CI completa superata sulla commit del tag.
- [ ] Binario Linux x86-64 e checksum SHA-256 pubblicati.
- [ ] Note di release, CHANGELOG e limitazioni note pubblicati.
- [ ] README, tutorial e riferimento CLI verificati da clone pulito.
- [ ] Security policy, licenza e contatti aggiornati.
- [ ] Release GitHub pubblicata come stabile, non prerelease.
- [ ] Comunicazione tecnica pronta per GitHub, Hacker News, X e LinkedIn.
- [ ] Canale per bug e feedback post-release attivo.
- [ ] Owner e piano di risposta agli incidenti comunicati al team.

## Riferimenti interni

- [README](https://raw.githubusercontent.com/high-cde/Zlang/main/README.md)
- [Whitepaper](https://raw.githubusercontent.com/high-cde/Zlang/main/ZLANG-WHITEPAPER.md)
- [Specifica del linguaggio](https://raw.githubusercontent.com/high-cde/Zlang/main/docs/language-spec.md)
- [Specifica del bytecode](https://raw.githubusercontent.com/high-cde/Zlang/main/docs/bytecode-spec.md)
- [Syscall ZDOS](https://raw.githubusercontent.com/high-cde/Zlang/main/docs/syscalls.md)
- [Security policy](https://raw.githubusercontent.com/high-cde/Zlang/main/SECURITY.md)
