# Zlang — linguaggio applicativo nativo per ZDOS

[![Profilo](https://img.shields.io/badge/profilo-ZLB0%20v1-1f6feb?style=for-the-badge)](docs/zdos-x86_64-profile.md)
[![Target](https://img.shields.io/badge/target-ZDOS%20x86__64-2ea043?style=for-the-badge)](https://github.com/high-cde/ZDOS/tree/main/os/x86_64)
[![Boot](https://img.shields.io/badge/boot-QEMU%20verificato-d29922?style=for-the-badge)](https://github.com/high-cde/ZDOS/tree/main/os/x86_64)
[![Licenza](https://img.shields.io/badge/licenza-da%20definire-6e7681?style=for-the-badge)](https://github.com/high-cde/Zlang)

> **Zlang è il linguaggio applicativo del prototipo ZDOS x86_64.** Oggi il profilo verificato compila un programma `.zlang` in bytecode **ZLB0 v1**, lo incorpora nel kernel bare-metal e lo esegue durante il boot in QEMU. Il progetto cresce da questa base concreta: prima contratti piccoli, testabili e sicuri; poi nuove capacità.

![Pipeline Zlang → ZDOS](https://raw.githubusercontent.com/high-cde/ZDOS/main/os/x86_64/assets/zdos-zlang-pipeline.png)

## 🚦 Stato reale, in una pagina

| Area | Disponibile oggi | Non ancora disponibile |
|---|---|---|
| Sintassi | `emit <testo>`, commenti `#`, righe vuote | Variabili, funzioni, moduli, tipi, controllo di flusso |
| Compilazione | `tools/zlangc.py` genera bytecode ZLB0 v1 e header C | Compilatore ottimizzante, package manager, linker applicativo |
| Runtime | Validazione di magic, versione, opcode, lunghezze e HALT | Heap, file system, rete, processi, eccezioni, scheduler |
| Target | Kernel ZDOS bare-metal x86_64 in QEMU | Loader persistente, applicazioni esterne autonome, hardware fisico |
| Verifica | Test Python, build freestanding, controllo Multiboot2 e smoke test QEMU | CI remota e matrice completa di regressione |

> **Regola di trasparenza:** non dichiarare una funzionalità “supportata” finché non ha contratto, implementazione, test negativo e prova ripetibile.

## 🧭 Perché Zlang esiste

Un sistema operativo non diventa affidabile aggiungendo soltanto funzioni. Diventa affidabile quando ogni passaggio tra sorgente, compilatore, runtime e macchina ha una responsabilità precisa. Zlang nasce per essere l’interfaccia applicativa di ZDOS: un linguaggio con un modello di esecuzione esplicito, un confine netto verso il kernel e una crescita controllata delle capacità.

Il primo obiettivo non è simulare un sistema completo. È dimostrare un fatto fondamentale: **un programma scritto in Zlang può attraversare una toolchain propria, essere incluso in un’immagine bootabile e venire eseguito da un kernel ZDOS senza dipendere da Linux a runtime**.

## ⚡ Avvio rapido — dalla sorgente al boot

Clona entrambi i repository come directory affiancate. Il `Makefile` di ZDOS individua il compilatore Zlang in questa posizione.

```sh
git clone https://github.com/high-cde/Zlang.git
git clone https://github.com/high-cde/ZDOS.git

cd ZDOS/os/x86_64
make clean
make all
make verify
sh tools/verify_qemu.sh
```

Per il build completo sono richiesti `python3`, `gcc`, `binutils`, `make`, `grub-mkrescue`, `xorriso` e `qemu-system-x86_64`.

La prova QEMU deve produrre esattamente il seguente percorso osservabile:

```text
ZDOS x86_64 bootstrap
Zlang runtime v1 ready
ZDOS: native Zlang program executed
ZDOS: Zlang halted cleanly
```

## ✍️ Il primo programma Zlang

Il profilo ZLB0 v1 è intenzionalmente minimale. Ogni programma è composto da commenti oppure da istruzioni `emit`.

```zlang
# examples/hello.zlang
emit Ciao dal programma Zlang nativo
emit Questo testo verrà interpretato dal kernel ZDOS
```

Il compilatore host produce due output: un file bytecode per l’ispezione e un header C da incorporare nel kernel.

```sh
python3 tools/zlangc.py examples/hello.zlang \
  --bytecode /tmp/hello.zlb \
  --header /tmp/hello.h
```

Se la sintassi non appartiene ancora al profilo, il compilatore fallisce deliberatamente.

```zlang
let risposta = 42
```

```text
zlangc: errore: ... sintassi non supportata; il profilo ZLB0 v1 accetta solo 'emit <testo>'
```

Questo comportamento è una garanzia: un programma non viene interpretato “per caso” come qualcosa di diverso da ciò che lo sviluppatore ha chiesto.

## 🧠 Il contratto ZLB0 v1

Il bytecode è un piccolo protocollo tra compilatore e kernel. La semplicità serve a rendere ogni regola ispezionabile.

| Campo | Valore | Ragione |
|---|---|---|
| Magic | `ZLB0` | Riconosce il formato senza ambiguità |
| Versione | `1` | Permette evoluzioni compatibili |
| `0x01` | `EMIT` | Trasferisce testo UTF-8 alla console seriale |
| `u16` little-endian | Lunghezza del payload | Evita letture oltre il buffer |
| `0xff` | `HALT` | Rende la fine del programma deterministica |

Il runtime nel kernel rifiuta magic, versione, opcode, lunghezza o terminazione non validi. Questa strategia **default-deny** è il seme della sicurezza futura: una capacità non definita non viene mai eseguita implicitamente.

## 🔬 Dal concetto al sistema: i cinque livelli

| Livello | Domanda didattica | Risposta nel prototipo |
|---|---|---|
| 1. Sorgente | Che cosa vuole fare il programma? | Dichiarare messaggi con `emit` |
| 2. Compilatore | Come diventa una forma eseguibile? | `zlangc.py` genera ZLB0 v1 |
| 3. Contratto | Come si evita l’ambiguità? | Magic, versione, opcode, lunghezze, HALT |
| 4. Kernel | Chi possiede l’accesso alla macchina? | Kernel ZDOS, non il bytecode direttamente |
| 5. Verifica | Come sappiamo che è successo davvero? | ISO, QEMU e output seriale obbligatorio |

> Il bytecode non riceve accesso arbitrario a shell, rete, credenziali o file. Le future syscall dovranno essere capability esplicite, ristrette, auditate e disabilitate per default.

## 🛡️ Limiti di sicurezza e maturità

Il prototipo è un **nucleo avviabile**, non un sistema operativo generale. Non include ancora processi, isolamento di memoria, scheduler, filesystem, rete, driver, loader esterno, package manager o API di sistema pubbliche. Il programma Zlang viene incorporato nel kernel soltanto per dimostrare una catena nativa completa e verificabile.

Questi limiti non sono difetti nascosti: sono confini di progetto. Prima di esporre una nuova capacità a Zlang, il progetto dovrà definire soggetto, capability, scope, allowlist, quota, timeout, limite di risposta, evento di audit, modalità di errore e test negativi.

## 🗺️ Roadmap ragionata

| Soglia | Risultato richiesto | Evidenza minima |
|---|---|---|
| **A — Contenitore esterno** | Caricare un file ZLB0 esterno in sola lettura | Parsing robusto, checksum e test di file malformato |
| **B — Valori e memoria** | Aggiungere variabili e aritmetica con limiti | Test di overflow, errori runtime e risorse esaurite |
| **C — Capability syscall** | Esporre log/tempo/input con policy default-deny | Allowlist, audit e test di capability negata |
| **D — Programmi multipli** | Gestire esecuzioni isolate e cooperative | Scheduler minimo, limiti di tempo e regressioni QEMU |
| **E — Distribuzione** | Rendere installabile il sistema su target definiti | Immagine firmata, release immutabile e documentazione di recupero |

## 🧪 Verifica locale

I test del compilatore si eseguono senza dipendenze esterne:

```sh
python3 -m unittest discover -s tests -p 'test_*.py' -v
```

Il controllo completo del prototipo coinvolge anche il kernel e QEMU. Le istruzioni sono raccolte nel [README di ZDOS x86_64](https://github.com/high-cde/ZDOS/tree/main/os/x86_64).

## 📚 Riferimenti

[1] [Profilo tecnico ZLB0 v1](docs/zdos-x86_64-profile.md)
[2] [Architettura ZDOS x86_64](https://github.com/high-cde/ZDOS/blob/main/os/x86_64/ARCHITECTURE.md)
[3] [Guida operativa e build del prototipo](https://github.com/high-cde/ZDOS/tree/main/os/x86_64)
[4] [Repository ZDOS](https://github.com/high-cde/ZDOS)
[5] [Repository Zlang](https://github.com/high-cde/Zlang)

---

**ZDOS + Zlang:** una base piccola, un contratto chiaro, una prova reale. Da qui si costruisce il resto. 🚀
