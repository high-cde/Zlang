![Zlang — Native Runtime for ZDOS](https://capsule-render.vercel.app/api?type=waving&color=0:170b3a,40:7c3aed,72:2563eb,100:10b981&height=220&section=header&text=Zlang&fontSize=74&fontColor=ffffff&animation=fadeIn&fontAlignY=38&desc=Native%20bytecode%20runtime%20for%20ZDOS&descAlignY=60&descSize=20)

# Zlang · il linguaggio applicativo di ZDOS

[![Validate Zlang](https://github.com/high-cde/Zlang/actions/workflows/validate.yml/badge.svg)](https://github.com/high-cde/Zlang/actions/workflows/validate.yml)
[![Profilo](https://img.shields.io/badge/profilo-ZLB2%20v2.5-7c3aed?style=for-the-badge&logo=rust&logoColor=white)](docs/zdos-x86_64-profile.md)
[![Target](https://img.shields.io/badge/target-ZDOS%20x86__64-2563eb?style=for-the-badge&logo=linux&logoColor=white)](https://github.com/high-cde/ZDOS/tree/main/os/x86_64)
[![Boot](https://img.shields.io/badge/boot-QEMU%20verified-059669?style=for-the-badge&logo=qemu&logoColor=white)](https://github.com/high-cde/ZDOS/actions)
[![Stato](https://img.shields.io/badge/stato-prototipo%20verificato-f59e0b?style=for-the-badge&logo=github&logoColor=white)](https://github.com/high-cde/Zlang)

> **Zlang è un runtime bytecode nativo per il prototipo ZDOS x86_64.** Il percorso già dimostrato è concreto: un file `.zlang` diventa bytecode **ZLB2 v2.5**, viene incorporato in un kernel bare-metal e viene eseguito durante il boot in QEMU.

## ⚡ In 30 secondi: dal sorgente al boot

![Pipeline Zlang → ZDOS](https://raw.githubusercontent.com/high-cde/ZDOS/main/os/x86_64/assets/zdos-zlang-pipeline.png)

| Passaggio | Componente | Prova osservabile |
|---|---|---|
| 📝 **Sorgente** | `programs/boot.zlang` | Istruzioni `emit <testo>` |
| ⚙️ **Compilazione** | `tools/zlangc.py` | Bytecode ZLB2 v2.5 e header C |
| 🧠 **Runtime** | `kernel/zlang.c` | Validazione di magic, versione, opcode e HALT |
| 💿 **Sistema** | ZDOS bare-metal x86_64 | ELF Multiboot2 e immagine ISO GRUB |
| 🖥️ **Verifica** | QEMU + seriale | `ZDOS: native Zlang program executed` |

> **Stato preciso:** Zlang non è ancora un linguaggio general purpose né ZDOS un sistema operativo generale. Variabili, funzioni, file system, rete, processi, driver, loader esterno e syscall pubbliche sono tappe future, non funzionalità già dichiarate.

## 🚀 Avvio rapido

Clona Zlang e ZDOS come directory affiancate. Il build ZDOS individua il compilatore Zlang in questa posizione.

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

Per il build sono richiesti `python3`, `gcc`, `binutils`, `make`, `grub-mkrescue`, `xorriso` e `qemu-system-x86_64`.

## ✍️ Scrivi il primo programma

Il profilo ZLB2 v2.5 è volutamente piccolo. Un programma usa commenti oppure l’istruzione `emit`.

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

Il compilatore rifiuta invece ciò che non appartiene ancora al profilo.

```zlang
let risposta = 42
```

```text
zlangc: errore: ... sintassi non supportata; il profilo ZLB2 v2.5 accetta solo 'emit <testo>'
```

Questo rifiuto è una garanzia: il linguaggio non promette una capacità prima di avere contratto, implementazione e test.

## 🧩 Cosa è supportato oggi

| Area | ✅ Disponibile | ⏳ Prossimo, ma non ancora supportato |
|---|---|---|
| Sintassi | `emit <testo>`, commenti `#`, righe vuote | Variabili, funzioni, moduli, tipi, controllo di flusso |
| Compilatore | `zlangc.py`, bytecode ZLB2 v2.5, header C | Ottimizzazioni, linker applicativo, package manager |
| Runtime | Magic, versione, opcode, lunghezze e HALT validati | Heap, error handling avanzato, scheduler, eccezioni |
| Sistema | Kernel ZDOS bare-metal x86_64 e QEMU | Loader persistente, app esterne, hardware fisico |
| Verifica | Test Python, Multiboot2, boot seriale QEMU, CI | Matrice hardware e regressioni multi-target |

## 🧠 Il contratto ZLB2 v2.5

Il bytecode è il patto esplicito tra compilatore e kernel. Ogni campo esiste per rendere il comportamento controllabile.

| Campo | Valore | Perché conta |
|---|---|---|
| Magic | `ZLB2` | Riconosce il formato senza ambiguità |
| Versione | `2.5` | Permette evoluzioni compatibili e rifiuti espliciti |
| Opcode `0x01` | `EMIT` | Trasferisce testo UTF-8 alla console seriale |
| `u16` little-endian | Lunghezza payload | Evita letture oltre il buffer |
| Opcode `0xff` | `HALT` | Rende la terminazione deterministica |

Il runtime rifiuta magic, versione, opcode, lunghezza e terminazione non validi. Questo modello **default-deny** è il primo confine di sicurezza: ciò che non è definito dal contratto non viene eseguito implicitamente.

## 🔬 Cinque livelli, una catena leggibile

| Livello | Domanda | Risposta nel prototipo |
|---|---|---|
| 1. 📝 Sorgente | Cosa vuole fare il programma? | Dichiarare un messaggio con `emit` |
| 2. ⚙️ Compilatore | Come diventa eseguibile? | `zlangc.py` genera ZLB2 v2.5 |
| 3. 🧩 Contratto | Come si evita l’ambiguità? | Magic, versione, opcode, lunghezze e HALT |
| 4. 🧠 Kernel | Chi può parlare con la macchina? | Il kernel ZDOS, non il bytecode direttamente |
| 5. ✅ Verifica | Come sappiamo che funziona? | ISO, QEMU, output seriale e GitHub Actions |

> Il bytecode non riceve accesso diretto a shell, rete, credenziali o file. Ogni futura syscall dovrà essere una capability esplicita, limitata, auditabile e disabilitata per default.

## 🛡️ Confini attuali e direzione futura

Il prototipo è un **nucleo avviabile**. L’assenza di processi, isolamento di memoria, filesystem, rete, driver, loader esterno e package manager non è nascosta: è un limite dichiarato. Prima di rendere una nuova capacità disponibile a Zlang, il progetto dovrà stabilire soggetto, capability, scope, allowlist, quota, timeout, evento di audit, errore e test negativo.

| Soglia | Nuova capacità | Evidenza richiesta prima di dichiararla supportata |
|---|---|---|
| **A — File ZLB2** | Caricare bytecode esterno in sola lettura | Parsing robusto, checksum e test di file malformato |
| **B — Valori** | Variabili e aritmetica locale | Limiti, overflow ed errori runtime controllati |
| **C — Capability** | Log, tempo o input con policy | Allowlist, audit, quota, timeout e test di diniego |
| **D — Più programmi** | Esecuzioni cooperative | Scheduler minimo, limiti di tempo e regressioni QEMU |
| **E — Distribuzione** | Target installabile | Immagine firmata, release immutabile e recupero documentato |

## 🔁 Pipeline CI/CD ZLB2

Ogni modifica al compilatore passa prima dal contratto ZLB2: i test Python generano bytecode e header, verificano magic e versione `2.5`, controllano record e `HALT`, quindi eseguono i controlli Rust. Il workflow usa il nome **Test ZLB2 compiler contract** e deve restare verde prima di integrare il compilatore con ZDOS.

La verifica end-to-end del sistema viene completata nel [workflow ZDOS x86_64](https://github.com/high-cde/ZDOS/blob/main/.github/workflows/validate-x86_64.yml), che ricompila il kernel, valida l’header generato, crea l’ISO e avvia QEMU. Una release non deve essere attestata finché compiler, contratto, kernel e boot non hanno superato i rispettivi gate.

## 🧪 Verifica locale

Il contratto del compilatore è coperto da test senza dipendenze esterne:

```sh
python3 -m unittest discover -s tests -p 'test_*.py' -v
```

Il workflow GitHub Actions esegue inoltre formattazione, build, Clippy e test Rust. La prova completa del sistema passa dal [README ZDOS x86_64](https://github.com/high-cde/ZDOS/tree/main/os/x86_64), dal [laboratorio teorico-pratico](https://github.com/high-cde/ZDOS/blob/main/os/x86_64/LEARNING_PATH.md) e dalla [validazione CI](https://github.com/high-cde/ZDOS/actions).

## 🛰️ Ecosistema ZDOS

| Componente | Ruolo | Collegamento |
|---|---|---|
| 🧠 ZDOS | Kernel, distro Linux e pipeline di boot | [Repository ZDOS](https://github.com/high-cde/ZDOS) |
| 🛰️ ZDOS-SEC | HUD, feed, ledger locale e stream Socket.IO | [Repository ZDOS-SEC-PORTAL](https://github.com/high-cde/ZDOS-SEC-PORTAL) |
| ⚙️ **Zlang** | Compilatore e contratto ZLB2 v2.5 | Questo repository |

La guida di stile e la mappa dei contratti dell’ecosistema sono disponibili in [`docs/ECOSYSTEM.md`](https://github.com/high-cde/ZDOS/blob/main/docs/ECOSYSTEM.md) e [`docs/DOCUMENTATION_STYLE.md`](https://github.com/high-cde/ZDOS/blob/main/docs/DOCUMENTATION_STYLE.md).

## 📚 Riferimenti

[1] [Profilo tecnico ZLB2 v2.5](docs/zdos-x86_64-profile.md)
[6] [Documento completo Zlang by ZDOS](docs/ZLANG_BY_ZDOS.md)
[7] [ZDOS Evidence Chain](https://github.com/high-cde/ZDOS/tree/main/evidence)
[2] [Architettura ZDOS x86_64](https://github.com/high-cde/ZDOS/blob/main/os/x86_64/ARCHITECTURE.md)
[3] [Guida operativa ZDOS x86_64](https://github.com/high-cde/ZDOS/tree/main/os/x86_64)
[4] [Laboratorio ZDOS x86_64 + Zlang](https://github.com/high-cde/ZDOS/blob/main/os/x86_64/LEARNING_PATH.md)
[5] [Workflow di validazione Zlang](https://github.com/high-cde/Zlang/actions/workflows/validate.yml)

---

**Zlang + ZDOS** · _Build what you can prove._ ✨

![Footer](https://capsule-render.vercel.app/api?type=waving&color=0:10b981,42:2563eb,100:7c3aed&height=120&section=footer)
