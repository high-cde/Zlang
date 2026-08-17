# Resoconto tecnico di ZLang

**Autore:** Manus AI
**Repository analizzato:** [high-cde/Zlang](https://github.com/high-cde/Zlang)
**Data dell’analisi:** 17 agosto 2026

## Sintesi esecutiva

**ZLang** è presentato come un linguaggio di sistema nativo dell’ecosistema **ZDOS**, pensato per scripting, demoni, orchestrazione di strumenti e client o nodi blockchain. Il repository è scritto principalmente in **Rust** e propone una toolchain composta da compilatore, macchina virtuale a bytecode, runtime, librerie standard, esempi e package manager denominato **ZPM**. [1]

Il progetto possiede una visione architetturale chiara e una documentazione estesa. Il confronto tra documentazione e sorgenti mostra però che l’implementazione attuale è ancora una **versione prototipale**, non equivalente a tutte le funzionalità dichiarate nella specifica.

> **Valutazione complessiva:** ZLang dispone di una buona impostazione progettuale, ma deve ancora consolidare la toolchain reale, unificare i diversi percorsi del compilatore e trasformare le specifiche documentali in funzionalità operative verificabili.

## 1. Identità e obiettivi

| Aspetto | Descrizione |
|---|---|
| Nome | ZLang |
| Estensione prevista | `.zlang` |
| Linguaggio d’implementazione | Rust |
| Modello | Linguaggio interpretato o compilato verso bytecode, eseguito da VM |
| Ecosistema | ZDOS |
| Scenari d’uso | Script di sistema, demoni, automazione, networking e blockchain |
| Architetture dichiarate | Linux/Termux, ARM e vecchi sistemi x86 |
| Versione indicata | `2026.2.0`, “Sovereign Engine” |

L’obiettivo è offrire un linguaggio compatto ma vicino al sistema operativo, con primitive per logging, esecuzione di comandi, rete, registry di sistema ed eventuali operazioni su Z-Chain. ZLang è quindi concepito più come **linguaggio embedded per un sistema operativo** che come linguaggio general-purpose autonomo.

## 2. Struttura del repository

Il repository è organizzato secondo i principali livelli di una toolchain linguistica.

| Directory | Ruolo previsto |
|---|---|
| `compiler/` | Lexer, parser, AST, type checking e generazione del codice |
| `vm/` | Bytecode, valori runtime, macchina virtuale e syscall |
| `runtime/` | Librerie standard per sistema, rete e filesystem |
| `src/` | Entry point Rust e un secondo percorso per compilatore e runtime |
| `zpm/` | Package manager del linguaggio |
| `cli/` | Spazio previsto per l’interfaccia a riga di comando |
| `examples/` | Esempi di programmi e demoni ZLang |
| `docs/` | Specifiche del linguaggio, bytecode e syscall |

Un aspetto da chiarire è la presenza di **due livelli apparentemente sovrapposti**: da una parte i moduli in `compiler/`, `vm/` e `zpm/`, dall’altra moduli analoghi sotto `src/`. Il binario definito in `Cargo.toml` usa `src/main.rs`, che importa i moduli esportati da `src/lib.rs`; pertanto il percorso effettivamente usato dall’eseguibile non coincide completamente con la struttura descritta nel README. [2]

## 3. Linguaggio descritto dalla specifica

La specifica documenta un linguaggio con sintassi semplice e familiare. Sono previsti:

| Area | Funzionalità dichiarate |
|---|---|
| Variabili | Dichiarazione con `let` e riassegnazione |
| Tipi | `int`, `float`, `bool`, `str`, `bytes`, `list`, `map`, `func` |
| Funzioni | Funzioni nominate, parametri tipizzati e funzioni anonime |
| Controllo di flusso | `if/else`, `for`, `while` |
| Moduli | `module` e `import` |
| Errori | `throw` e `try/catch` |
| Espressioni | Operazioni aritmetiche, logiche, confronti e chiamate |
| Strutture dati | Liste, mappe e indicizzazione |

La grammatica EBNF include operatori aritmetici e logici, confronti, accesso a membri, chiamate di funzione, literal composti e annotazioni di tipo. [3]

Questa è però la **superficie linguistica progettata**, non ancora la sintassi interamente disponibile nel percorso esecutivo principale. Il lexer/parser presente sotto `src/` gestisce un sottoinsieme più piccolo: numeri interi, identificatori, `let`, `print`, operatori aritmetici, assegnazione e parentesi. Non risultano collegati a quel percorso tutti i costrutti documentati, quali stringhe, mappe, liste, funzioni, moduli, cicli, import ed eccezioni.

## 4. Compilatore e macchina virtuale

La documentazione descrive una VM basata su stack, con registri concettuali `IP`, `SP` e `FP`, memoria separata per codice, heap e stack, e bytecode con header, costanti, simboli e istruzioni. Il set di istruzioni previsto comprende operazioni aritmetiche e logiche, caricamento e salvataggio di variabili, salti, chiamate di funzione, strutture dati e syscall. [4]

Nel codice usato dall’entry point principale, il compilatore è invece molto più minimale. La funzione `compile` riconosce principalmente tre forme:

| Costrutto riconosciuto | Traduzione |
|---|---|
| `emit ...` | `PRINT_STDOUT ...` |
| `orbit_sync` | `SPACEX_LEO_HANDSHAKE` |
| `zchain ...` | `ZCHAIN_SIGN ...` |

La VM esegue queste istruzioni come stringhe, stampando l’output, simulando l’handshake orbitale oppure chiamando la funzione di firma Z-Chain. Questo dimostra il flusso **sorgente → compilatore → VM → runtime**, ma non costituisce ancora una VM a bytecode binario nel senso completo descritto dalla documentazione.

## 5. Runtime, syscall e ZPM

Il runtime attuale inizializza il sistema con un messaggio descrittivo. Il modulo ZPM contiene una funzione per simulare la firma di una transazione Z-Chain e la sua inclusione nel blocco Genesis.

La documentazione prevede syscall come `SYS_LOG`, `SYS_EXEC`, accesso al registry, gestione degli eventi, tempo e networking. Prevede inoltre comandi CLI come `zlang run`, `zlang build` e `zlang exec`, oltre a `zpm init`, `zpm build` e `zpm run`. Questi comandi rappresentano una direzione di prodotto coerente, ma devono essere verificati e implementati esplicitamente nella CLI effettiva prima di poterli considerare un’interfaccia stabile. [1] [5]

## 6. Esempi e integrazioni

Gli esempi inclusi mostrano tre direzioni principali: un programma introduttivo, un logger pensato come daemon e un nodo blockchain. Il runtime contiene inoltre librerie standard per sistema, rete e filesystem.

La documentazione prevede l’integrazione con ZDOS attraverso percorsi standard, registry di configurazione, demoni avviati al boot e wrapper shell. È documentata anche un’integrazione con Discord tramite un repository separato, in cui un bot invia richieste a un daemon locale.

Il modello di sicurezza proposto per Discord è corretto come impostazione: devono essere eseguiti soltanto script autorizzati, non codice arbitrario ricevuto dal bot, e i ruoli Discord devono essere associati a permessi distinti. Questa precauzione è essenziale per un linguaggio capace di invocare comandi di sistema. [1]

## 7. Stato attuale

| Area | Stato osservato | Implicazione |
|---|---|---|
| Documentazione | Ampia e strutturata | La visione tecnica è già delineata |
| Specifica sintattica | Ricca | Esiste una base per il design del linguaggio |
| Lexer/parser operativo | Sottoinsieme ridotto | La sintassi reale è più limitata della specifica |
| VM | Dimostrativa, con istruzioni stringa | Manca il bytecode binario completo descritto |
| CLI | Entry point minimale | I comandi documentati devono essere consolidati |
| Syscall ZDOS | In gran parte dichiarate | Serve un ABI stabile e un dispatcher reale |
| Package manager | Struttura iniziale | Mancano gestione completa di dipendenze e build |
| Test automatici | Da consolidare | Servono test riproducibili in CI |
| Build locale | Non eseguita nell’ambiente analizzato perché `cargo` non era disponibile | Gli artefatti precompilati non sostituiscono una build da sorgente |

Il punto più significativo è la distanza tra **specifica** e **implementazione**. Il README descrive un linguaggio completo, mentre il codice attivo del binario appare ancora come una dimostrazione del percorso minimo di esecuzione.

È inoltre presente almeno un’anomalia sintattica in un esempio `try/catch` del README. Questo suggerisce l’opportunità di verificare automaticamente tutti gli esempi durante la CI, così da evitare divergenze tra documentazione e compilatore.

## 8. Priorità di sviluppo consigliate

### 8.1 Unificare l’architettura

La prima priorità dovrebbe essere scegliere una sola architettura canonica: i moduli sotto `src/` oppure le directory di primo livello `compiler/`, `vm/` e `zpm/`. Successivamente occorre collegare lexer, parser, AST, type checker, code generator e VM in una pipeline unica.

### 8.2 Definire un MVP linguistico

È consigliabile stabilire un primo insieme minimo ma coerente di funzionalità: numeri, stringhe, variabili, funzioni, condizioni, cicli, moduli e gestione degli errori. Ogni caratteristica dovrebbe avere almeno un test di parsing, uno di compilazione e uno di esecuzione.

### 8.3 Stabilizzare bytecode e syscall

È necessario definire un formato binario versionato, un set di opcode stabile, una rappresentazione dei valori e un dispatcher delle syscall con controlli di sicurezza, limiti di risorse e gestione degli errori.

### 8.4 Consolidare CLI e ZPM

La CLI dovrebbe offrire comandi coerenti con la documentazione e messaggi d’errore strutturati. ZPM dovrebbe poi gestire manifest, dipendenze, build riproducibili, cache e versionamento dei pacchetti.

### 8.5 Introdurre CI e test end-to-end

Il progetto trarrebbe grande beneficio da una pipeline CI con `cargo fmt`, `cargo clippy`, test unitari, test degli esempi e build release su Linux x86-64 e ARM. Gli esempi documentali dovrebbero essere eseguiti automaticamente a ogni modifica.

## Conclusione

ZLang è un progetto con una **visione interessante e coerente**: offrire a ZDOS un linguaggio nativo, portabile e orientato all’automazione di sistema. La separazione concettuale tra compilatore, VM, runtime, syscall e package manager costituisce una buona base per lo sviluppo futuro.

Al momento, tuttavia, è più corretto definirlo un **prototipo avanzato con specifica estesa** che un linguaggio completo pronto per la produzione. Il valore principale del repository risiede nella direzione architetturale, nella documentazione e nel primo percorso dimostrativo di esecuzione. Il passaggio decisivo sarà unificare il codice, implementare progressivamente la grammatica documentata e aggiungere test automatici che dimostrino quali funzionalità siano realmente disponibili.

## Riferimenti

[1]: https://github.com/high-cde/Zlang/blob/main/README.md "ZLang README"
[2]: https://github.com/high-cde/Zlang/blob/main/Cargo.toml "ZLang Cargo manifest"
[3]: https://github.com/high-cde/Zlang/blob/main/docs/language-spec.md "ZLang language specification"
[4]: https://github.com/high-cde/Zlang/blob/main/docs/bytecode-spec.md "ZLang bytecode specification"
[5]: https://github.com/high-cde/Zlang/blob/main/docs/syscalls.md "ZLang syscall documentation"
