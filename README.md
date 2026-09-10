# Zlang

**Compilatore e runtime bytecode per il prototipo ZDOS x86_64.**

[![Validate Zlang](https://github.com/high-cde/Zlang/actions/workflows/validate.yml/badge.svg)](https://github.com/high-cde/Zlang/actions/workflows/validate.yml)
[![Bytecode](https://img.shields.io/badge/bytecode-ZLB2%20v2.5-7c3aed?style=for-the-badge&logo=rust&logoColor=white)](docs/zdos-x86_64-profile.md)
[![Target](https://img.shields.io/badge/target-ZDOS%20x86__64-2563eb?style=for-the-badge&logo=linux&logoColor=white)](https://github.com/high-cde/ZDOS/tree/main/os/x86_64)
[![Boot](https://img.shields.io/badge/boot-QEMU%20verified-059669?style=for-the-badge&logo=qemu&logoColor=white)](https://github.com/high-cde/ZDOS/actions)
[![License](https://img.shields.io/badge/license-MIT-64748b?style=for-the-badge)](LICENSE)

> Zlang è il compilatore e il runtime bytecode del prototipo ZDOS x86_64. Il percorso verificato è concreto: un file `.zlang` viene compilato in bytecode **ZLB2 v2.5**, incorporato in un kernel bare-metal ed eseguito durante il boot in QEMU.

## In breve

Zlang definisce il linguaggio sorgente, il compilatore e il contratto bytecode utilizzato dal runtime ZDOS. Il progetto privilegia un comportamento esplicito: ciò che non appartiene al profilo supportato viene rifiutato invece di essere interpretato implicitamente.

| Fase | Componente | Evidenza osservabile |
|---|---|---|
| **Sorgente** | File `.zlang`, ad esempio `examples/hello.zlang` | Istruzioni `emit <testo>` |
| **Compilazione** | `tools/zlangc.py` | Bytecode ZLB2 v2.5 e header C |
| **Runtime** | `kernel/zlang.c` nel target ZDOS | Validazione di magic, versione, opcode e `HALT` |
| **Sistema** | ZDOS bare-metal x86_64 | ELF Multiboot2 e ISO GRUB |
| **Verifica** | QEMU, seriale e GitHub Actions | Output di boot riproducibile |

**Stato attuale:** Zlang non è ancora un linguaggio general-purpose e ZDOS non è ancora un sistema operativo general-purpose. Variabili, funzioni, filesystem nativo, rete, processi, driver, loader esterno e syscall pubbliche sono ancora limitati o futuri. La capability Linux confinata `storage.read-v1` è disponibile separatamente tramite bridge read-only.

## Avvio rapido

Clona Zlang e ZDOS nella stessa directory, quindi esegui la verifica del target bare-metal:

```sh
git clone https://github.com/high-cde/Zlang.git
git clone https://github.com/high-cde/ZDOS.git

cd ZDOS/os/x86_64
make clean
make verify
sh tools/verify_qemu.sh
```

L’esecuzione completa deve produrre questa sequenza seriale:

```text
ZDOS x86_64 bootstrap
Zlang runtime ZLB2 v2.5 ready
ZDOS: native Zlang program executed
ZDOS: Zlang halted cleanly
```

Per la build sono richiesti `python3`, `gcc`, `binutils`, `make`, `grub-mkrescue`, `xorriso` e `qemu-system-x86_64`.

## Scrivere il primo programma

Il profilo ZLB2 v2.5 è volutamente minimale. Un programma può contenere commenti, righe vuote, istruzioni `emit` e il record capability `storage.read`:

```zlang
# examples/hello.zlang
emit Ciao dal programma Zlang nativo
emit Il kernel ZDOS ha eseguito questo bytecode
```

Compila l’esempio in bytecode e header C:

```sh
python3 tools/zlangc.py examples/hello.zlang \
  --bytecode /tmp/hello.zlb \
  --header /tmp/hello.h
```

Il compilatore rifiuta ciò che non appartiene ancora al profilo:

```zlang
let risposta = 42
```

La capability `storage.read` è valida soltanto con un path relativo al namespace autorizzato e viene eseguita nel percorso Linux dal bridge documentato più avanti.

```text
zlangc v2.5 error: sintassi sconosciuta; il profilo rifiuta istruzioni non definite dal contratto
```

Questo comportamento mantiene allineati linguaggio, contratto, runtime e documentazione.

## Supporto attuale e roadmap

| Area | Supportato oggi | Non ancora supportato |
|---|---|---|
| **Sintassi** | `emit <testo>`, `storage.read "path"`, commenti `#`, righe vuote | Funzioni, moduli, tipi e controllo di flusso |
| **Compilatore** | `zlangc.py`, bytecode ZLB2 v2.5 e header C | Ottimizzazioni, linker applicativo e package manager |
| **Runtime** | Magic, versione, opcode, lunghezze e `HALT` validati | Heap, gestione avanzata degli errori, scheduler ed eccezioni |
| **Sistema** | Kernel ZDOS bare-metal x86_64 e QEMU | Loader persistente, applicazioni esterne e hardware fisico |
| **Verifica** | Test Python, Multiboot2, boot seriale QEMU e CI | Matrice hardware e regressioni multi-target |

## Contratto ZLB2 v2.5

Il bytecode è il contratto esplicito tra compilatore e runtime. Ogni campo ha una funzione verificabile:

| Campo | Valore | Funzione |
|---|---|---|
| **Magic** | `ZLB2` | Identifica il formato senza ambiguità |
| **Versione** | `2.5` | Consente evoluzione e rifiuti espliciti |
| **Opcode** | `0x01` — `EMIT`; `0x06` — `STORAGE_READ` | Console seriale; richiesta read-only confinata al bridge Linux |
| **Lunghezza** | `u16` little-endian | Impedisce letture oltre il buffer |
| **Terminazione** | `0xff` — `HALT` | Rende deterministica la fine del programma |

Il runtime rifiuta magic, versione, opcode, lunghezza o terminazione non validi. Questo modello **default-deny** stabilisce il primo confine di sicurezza: ciò che non è definito dal contratto non viene eseguito implicitamente.

Il riferimento tecnico completo è il [profilo ZLB2 v2.5](docs/zdos-x86_64-profile.md).

## Catena di esecuzione

| Livello | Domanda | Risposta nel prototipo |
|---|---|---|
| **1. Sorgente** | Cosa descrive il programma? | Un messaggio o una richiesta `storage.read` |
| **2. Compilatore** | Come diventa bytecode? | `zlangc.py` genera ZLB2 v2.5 |
| **3. Contratto** | Come si evita l’ambiguità? | Magic, versione, opcode, lunghezze e `HALT` |
| **4. Kernel** | Chi controlla l’esecuzione? | Il runtime del kernel ZDOS |
| **5. Verifica** | Come sappiamo che funziona? | ISO, QEMU, output seriale e CI |

Il bytecode non riceve accesso diretto a shell, rete, credenziali o filesystem. Ogni futura syscall dovrà essere una capability esplicita, limitata, auditabile e disabilitata per default.

## Roadmap guidata dall’evidenza

Prima di dichiarare supportata una nuova capacità, il progetto richiede contratto, implementazione, limiti, test positivi e test di diniego.

| Soglia | Capacità prevista | Evidenza richiesta |
|---|---|---|
| **A — File ZLB2** | Caricamento di bytecode esterno in sola lettura | Parsing robusto, checksum e test di file malformato |
| **B — Valori** | Variabili e aritmetica locale | Limiti, overflow ed errori runtime controllati |
| **C — Capability** | `storage.read-v1` read-only con namespace | Allowlist, isolamento, quota, audit e test di diniego |
| **D — Più programmi** | Esecuzioni cooperative | Scheduler minimo, limiti di tempo e regressioni QEMU |
| **E — Distribuzione** | Target installabile | Immagine firmata, release immutabile e recupero documentato |

## Verifica locale e CI

Esegui i test del contratto del compilatore senza dipendenze esterne:

```sh
python3 -m unittest discover -s tests -p 'test_*.py' -v
```

GitHub Actions esegue inoltre formattazione, build, Clippy e test Rust. La verifica end-to-end viene completata nel [workflow ZDOS x86_64](https://github.com/high-cde/ZDOS/blob/main/.github/workflows/validate-x86_64.yml), che ricompila il kernel, valida l’header, crea l’ISO e avvia QEMU.

## Ecosistema ZDOS

La guida coordinata [ZDOS–Zlang–ZRetro](docs/ZDOS_ZRETRO_BRIDGE.md) documenta il rapporto tra sorgente, bytecode, runtime e IDE retro. Il tasto **ZRetro** conduce alla pagina pubblica [Retro Computing](https://zdos-hub.it/retro-computing.html).

| Componente | Ruolo | Collegamento |
|---|---|---|
| **ZDOS** | Kernel, distribuzione Linux e pipeline di boot | [Repository ZDOS](https://github.com/high-cde/ZDOS) |
| **Zlang** | Compilatore e contratto ZLB2 v2.5 | Questo repository |

La mappa dei contratti e le convenzioni documentali dell’ecosistema sono disponibili in [`docs/ECOSYSTEM.md`](https://github.com/high-cde/ZDOS/blob/main/docs/ECOSYSTEM.md) e [`docs/DOCUMENTATION_STYLE.md`](https://github.com/high-cde/ZDOS/blob/main/docs/DOCUMENTATION_STYLE.md).

La pipeline evolutiva completa è orchestrata dal repository ZDOS tramite [`scripts/evolve-zlang-evidence.sh`](https://github.com/high-cde/ZDOS/blob/main/scripts/evolve-zlang-evidence.sh). Il comando collega il sorgente Zlang, il bytecode ZLB2, il kernel ZDOS, il boot QEMU e una registrazione append-only nella Evidence Chain. L’attestazione contiene hash degli artefatti e commit dei repository; non implica networking, consenso distribuito o esecuzione remota.

## Documentazione

| Risorsa | Scopo |
|---|---|
| [`docs/language-spec.md`](docs/language-spec.md) | Specifica del linguaggio |
| [`docs/bytecode-spec.md`](docs/bytecode-spec.md) | Specifica del bytecode |
| [`docs/zdos-x86_64-profile.md`](docs/zdos-x86_64-profile.md) | Profilo di integrazione con ZDOS |
| [`docs/syscalls.md`](docs/syscalls.md) | Stato e contratto delle syscall |
| [`docs/ZLANG_BY_ZDOS.md`](docs/ZLANG_BY_ZDOS.md) | Guida Zlang nel contesto ZDOS |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Workflow per i contributi |
| [`SECURITY.md`](SECURITY.md) | Segnalazioni e principi di sicurezza |
| [`CHANGELOG.md`](CHANGELOG.md) | Modifiche rilevanti e baseline di rilascio |

## Contribuire

Mantieni le modifiche circoscritte, aggiorna la documentazione quando cambia il comportamento e includi test per ogni modifica al contratto o al compilatore. Prima di aprire una pull request:

```sh
python3 -m unittest discover -s tests -p 'test_*.py' -v
cargo fmt --all -- --check
cargo check --all-targets
git diff --check
```

Consulta [`CONTRIBUTING.md`](CONTRIBUTING.md), [`SECURITY.md`](SECURITY.md) e [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) prima di contribuire.

## Licenza

Questo progetto è distribuito secondo la licenza indicata in [`LICENSE`](LICENSE).

---

**Zlang · ZDOS** — *Build what you can prove.*


## storage.read-v1: capability read-only

Zlang v2.5 supporta il record ZLB2 `0x06` tramite la sintassi:

```zlang
storage.read ".zdos-persistence-marker"
```

Il percorso è relativo al namespace autorizzato. Path assoluti, `..`, NUL byte e accesso a `/dev` vengono rifiutati.

Nel percorso Linux verificato da ZDOS, la lettura reale avviene tramite il bridge confinato:

```sh
python3 tools/zlang_storage_read.py program.zlang \
  --root /mnt/data \
  --max-bytes 4096
```

Il bridge è read-only, applica un limite di dimensione, risolve il path dentro `/mnt/data` e restituisce `ZLANG_STORAGE_READ_OK` con path relativo, dimensione e hash del contenuto. La scrittura non è supportata.

Il runtime bare-metal ZDOS riconosce e valida l’opcode `storage.read`, ma dichiara esplicitamente che la capability richiede il bridge Linux finché il filesystem nativo del kernel non sarà disponibile. Non è accesso raw al disco e non implica che il target bare-metal possieda già un filesystem.

Verifica locale:

```sh
python3 tests/test_storage_read_v1.py
```

Questa milestone dimostra il contratto Zlang e la lettura confinata nel percorso Linux; non abilita syscall universali, accesso al filesystem host fuori dal namespace o scrittura persistente.
