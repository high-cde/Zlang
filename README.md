# zdos-zlang: Linguaggio di Sistema per ZDOS

Questo repository contiene il progetto ZLang, una macchina virtuale (VM) a bytecode, la toolchain associata e le integrazioni previste con ZDOS e Discord. L'obiettivo è fornire un pacchetto completo per lo sviluppo di script, demoni e tool di orchestrazione nativi per il sistema operativo ZDOS.

> **Stato di implementazione.** Il profilo oggi verificato per l’avvio nativo di ZDOS è **ZLB0 v1**: compila esclusivamente istruzioni `emit <testo>` nel tool `tools/zlangc.py` ed è eseguito dal prototipo bare-metal x86_64 in QEMU. Variabili, funzioni, moduli, syscall, package manager e le altre estensioni descritte in questo documento restano pianificate finché non saranno implementate e coperte da test. Istruzioni e contratto del profilo attivo sono disponibili in [docs/zdos-x86_64-profile.md](docs/zdos-x86_64-profile.md).

## Indice

1.  [Identità del Linguaggio](#1-identità-del-linguaggio)
2.  [Specifica del Linguaggio](#2-specifica-del-linguaggio)
    *   [Tipi e Valori](#21-tipi-e-valori)
    *   [Variabili e Assegnazione](#22-variabili-e-assegnazione)
    *   [Funzioni](#23-funzioni)
    *   [Controllo di Flusso](#24-controllo-di-flusso)
    *   [Moduli](#25-moduli)
    *   [Errori ed Eccezioni](#26-errori-ed-eccezioni)
3.  [Grammatica (EBNF minimale)](#3-grammatica-ebnf-minimale)
4.  [VM e Runtime](#4-vm-e-runtime)
    *   [Modello](#41-modello)
    *   [Formato Bytecode](#42-formato-bytecode)
    *   [Set di Istruzioni Base](#43-set-di-istruzioni-base-estratto)
    *   [Syscall ZDOS](#44-syscall-zdos)
5.  [Implementazione: Struttura del Repository](#5-implementazione-struttura-del-repository)
    *   [Repo Principale](#51-repo-principale)
    *   [Toolchain](#52-toolchain)
6.  [Integrazione con ZDOS](#6-integrazione-con-zdos)
    *   [Installazione in ZDOS](#61-installazione-in-zdos)
    *   [Wrapper Shell](#62-wrapper-shell)
    *   [Registry ZDOS](#63-registry-zdos)
    *   [Demoni ZDOS in ZLang](#64-demoni-zdos-in-zlang)
7.  [Package Manager ZPM](#7-package-manager-zpm)
    *   [File di Configurazione](#71-file-di-configurazione)
    *   [Comandi](#72-comandi)
8.  [Integrazione con Discord](#8-integrazione-con-discord)
    *   [Architettura](#81-architettura)
    *   [Flusso](#82-flusso)
    *   [Sicurezza](#83-sicurezza)
9.  [Esempio Completo: `chain_node.zlang`](#9-esempio-completo-chain_nodezlang)
10. [Roadmap di Implementazione](#10-roadmap-di-implementazione)

---

## 1. Identità del Linguaggio

**ZLang** è un linguaggio di sistema nativo progettato per ZDOS, ideale per scripting, demoni, orchestrazione di tool e la creazione di client/nodi blockchain. È un linguaggio interpretato con una VM a bytecode, garantendo portabilità su diverse architetture come Linux/Termux, vecchi x86 e ARM.

-   **Nome**: ZLang
-   **Estensione file**: `.zlang`
-   **Ruolo**: Linguaggio di sistema nativo ZDOS, scripting, daemon, orchestrazione tool, client/nodi blockchain.
-   **Target**: Interprete + VM a bytecode, portabile su Linux/Termux, x86 vecchi, ARM.

## 2. Specifica del Linguaggio

### 2.1. Tipi e Valori

ZLang supporta i seguenti tipi primitivi e literal:

| Tipo      | Descrizione                               |
| :-------- | :---------------------------------------- |
| `int`     | Intero a 64 bit                           |
| `float`   | Virgola mobile a 64 bit                   |
| `bool`    | Valori booleani (`true` / `false`)        |
| `str`     | Stringhe UTF-8                            |
| `bytes`   | Sequenze di byte                          |
| `list`    | Liste eterogenee                          |
| `map`     | Mappe con chiave `str` e valore generico  |
| `func`    | Funzioni                                  |

Esempi di literal:

```zlang
let a = 42
let pi = 3.14
let ok = true
let name = "High"
let data = 0xDEADBEEF
let arr = [1, 2, 3]
let cfg = { "id": "node-1", "port": 8080 }
```

### 2.2. Variabili e Assegnazione

Le variabili possono essere dichiarate con `let` e opzionalmente tipizzate. La riassegnazione è supportata.

**Dichiarazione**:

```zlang
let x = 10
let msg: str = "hello"
```

**Riassegnazione**:

```zlang
x = x + 1
```

### 2.3. Funzioni

Le funzioni sono definite con la parola chiave `func`, possono accettare parametri tipizzati e restituire un valore. Sono supportate anche le funzioni anonime.

**Definizione**:

```zlang
func add(a: int, b: int) -> int {
    return a + b
}

func log_system(msg: str) {
    sys.log("core", msg)
}
```

**Funzioni anonime**:

```zlang
let f = func(x: int) -> int {
    return x * 2
}
```

### 2.4. Controllo di Flusso

ZLang include costrutti per il controllo di flusso come `if/else`, `for` e `while`.

```zlang
if x > 5 {
    sys.log("test", "x > 5")
} else {
    sys.log("test", "x <= 5")
}

for i in 0..10 {
    sys.log("loop", "i=" + str(i))
}

while x < 100 {
    x = x + 10
}
```

### 2.5. Moduli

I moduli permettono di organizzare il codice e gestire le dipendenze.

**Dichiarazione modulo**:

```zlang
module chain.node
```

**Import**:

```zlang
import sys
import net
import chain.util
```

### 2.6. Errori ed Eccezioni

La gestione degli errori avviene tramite `throw` e `try/catch`.

```zlang
func risky() {
    if something_wrong {
        throw "bad state"
    }
}

func main() {
    try {
        risky()
    n    } catch err {
        sys.log("err", "caught: " + err)
    }
}
```

## 3. Grammatica (EBNF minimale)

La grammatica di ZLang è definita in EBNF:

```ebnf
Program      = { Statement } ;

Statement    = VarDecl | Assign | FuncDecl | IfStmt | ForStmt | WhileStmt
             | ImportStmt | ModuleStmt | ExprStmt | ThrowStmt | TryCatch ;

VarDecl      = "let" Identifier [ ":" Type ] "=" Expression ;
Assign       = Identifier "=" Expression ;
FuncDecl     = "func" Identifier "(" [ ParamList ] ")" [ "->" Type ] Block ;
ParamList    = Param { "," Param } ;
Param        = Identifier ":" Type ;

IfStmt       = "if" Expression Block [ "else" Block ] ;
ForStmt      = "for" Identifier "in" Expression ".." Expression Block ;
WhileStmt    = "while" Expression Block ;

ImportStmt   = "import" Identifier { "." Identifier } ;
ModuleStmt   = "module" Identifier { "." Identifier } ;

ThrowStmt    = "throw" Expression ;
TryCatch     = "try" Block "catch" Identifier Block ;

Block        = "{" { Statement } "}" ;

ExprStmt     = Expression ;

Expression   = LogicOr ;
LogicOr      = LogicAnd { "||" LogicAnd } ;
LogicAnd     = Equality { "&&" Equality } ;
Equality     = Relational { ("==" | "!=") Relational } ;
Relational   = Additive { ("<" | ">" | "<=" | ">=") Additive } ;
Additive     = Multiplicative { ("+" | "-") Multiplicative } ;
Multiplicative = Unary { ("*" | "/" | "%") Unary } ;
Unary        = [ "!" | "-" ] Primary ;
Primary      = Literal
             | Identifier
             | "(" Expression ")"
             | Primary "." Identifier
             | Primary "(" [ ArgList ] ")" ;

ArgList      = Expression { "," Expression } ;

Literal      = IntLiteral | FloatLiteral | StringLiteral
             | BoolLiteral | ListLiteral | MapLiteral ;

ListLiteral  = "[" [ Expression { "," Expression } ] "]" ;
MapLiteral   = "{" [ StringLiteral ":" Expression { "," StringLiteral ":" Expression } ] "}" ;

Type         = "int" | "float" | "bool" | "str" | "bytes" | "list" | "map" | "func" ;

Identifier   = Letter { Letter | Digit | "_" } ;
```

## 4. VM e Runtime

### 4.1. Modello

La VM di ZLang è basata su stack e include registri interni come `IP` (instruction pointer), `SP` (stack pointer) e `FP` (frame pointer). La memoria è divisa in segmenti per codice (bytecode), heap (oggetti) e stack (valori e frame di chiamata).

### 4.2. Formato Bytecode

Il formato del bytecode compilato include un header (`ZBC0`, versione), tabelle per costanti e simboli, e una sezione codice con un array di istruzioni. Ogni istruzione è composta da 1 byte di opcode e 0-8 byte di operandi.

### 4.3. Set di Istruzioni Base (Estratto)

Il set di istruzioni include operazioni per stack, variabili, aritmetica, logica, controllo di flusso e strutture dati.

| Categoria    | Istruzioni                                                                |
| :----------- | :------------------------------------------------------------------------ |
| **Stack**    | `PUSH_CONST idx`, `POP`                                                   |
| **Variabili**| `LOAD_LOCAL idx`, `STORE_LOCAL idx`, `LOAD_GLOBAL idx`, `STORE_GLOBAL idx`|
| **Operazioni**| `ADD`, `SUB`, `MUL`, `DIV`, `MOD`, `EQ`, `NEQ`, `LT`, `GT`, `LE`, `GE`, `AND`, `OR`, `NOT` |
| **Controllo**| `JMP addr`, `JMPIFFALSE addr`, `CALL func_idx, argc`, `RET`               |
| **Strutture**| `NEW_LIST n`, `NEW_MAP n`, `GET_INDEX`, `SET_INDEX`                       |
| **Syscall**  | `SYS_CALL id, argc` (per `sys.`, `net.`, ecc.)                            |

### 4.4. Syscall ZDOS

Le syscall ZDOS permettono a ZLang di interagire con il sistema operativo. Esempi includono:

| Syscall        | Funzione ZLang corrispondente                               |
| :------------- | :---------------------------------------------------------- |
| `SYS_LOG`      | `sys.log(tag, msg)`                                         |
| `SYS_EXEC`     | `sys.exec(cmd, args)`                                       |
| `SYSEXECCAPTURE`| (Non specificato)                                           |
| `SYSREGGET`    | `sys.registry.get(key)`                                     |
| `SYSREGSET`    | (Non specificato)                                           |
| `SYSEVENTEMIT` | (Non specificato)                                           |
| `SYSTIMENOW`   | (Non specificato)                                           |
| `SYSNETCONNECT`| (Non specificato)                                           |
| `SYSNETSEND`   | (Non specificato)                                           |
| `SYSNETRECV`   | (Non specificato)                                           |

## 5. Implementazione: Struttura del Repository

### 5.1. Repo Principale

Il repository `zdos-zlang` è organizzato come segue:

```
zdos-zlang/
├── compiler/             # Componenti del compilatore (lexer, parser, AST, codegen)
│   ├── lexer.rs
│   ├── parser.rs
│   ├── ast.rs
│   ├── typecheck.rs      # (Opzionale/estendibile)
│   └── codegen.rs        # (AST → bytecode)
├── vm/                   # Implementazione della macchina virtuale
│   ├── vm.rs
│   ├── bytecode.rs
│   ├── value.rs
│   └── syscalls.rs
├── runtime/              # Librerie standard ZLang
│   ├── stdlib_sys.zlang
│   ├── stdlib_net.zlang
│   └── stdlib_fs.zlang
├── cli/                  # Interfaccia a riga di comando (CLI) di ZLang
│   └── main.rs           # (zlang CLI)
├── examples/             # Esempi di codice ZLang
│   ├── hello.zlang
│   ├── daemon_logger.zlang
│   └── chain_node.zlang
├── docs/                 # Documentazione del progetto
│   ├── language-spec.md
│   ├── bytecode-spec.md
│   └── syscalls.md
└── zpm/                  # Package manager ZLang (zpm)
    └── zpm.rs
```

### 5.2. Toolchain

Il linguaggio di implementazione è **Rust**. La build e i comandi CLI sono gestiti come segue:

**Build**:

```bash
cargo build --release
# Produce: target/release/zlang
```

**Comandi CLI**:

```bash
# Esegui script
zlang run file.zlang

# Compila in bytecode
zlang build file.zlang -o file.zbc

# Esegui bytecode
zlang exec file.zbc
```

## 6. Integrazione con ZDOS

### 6.1. Installazione in ZDOS

ZLang si integra con ZDOS seguendo percorsi standard:

-   **Binario**: `/usr/local/bin/zlang`
-   **Stdlib**: `/opt/zdos/zlang/stdlib/`
-   **Config**: `/etc/zdos/zlang.conf`

### 6.2. Wrapper Shell

Uno script wrapper `zlang.sh` facilita l'esecuzione di ZLang in ZDOS:

```sh
#!/bin/sh
exec /usr/local/bin/zlang "$@"
```

Questo permette comandi come:

```bash
zlang run /opt/zdos/scripts/boot.zlang
```

### 6.3. Registry ZDOS

ZLang può accedere al Registry ZDOS per configurazioni e dati di sistema:

```zlang
let node_cfg = sys.registry.get("chain.node")
sys.log("cfg", "id=" + node_cfg["id"])
```

L'implementazione di `SYSREGGET` e `SYSREGSET` mappa su file YAML/JSON in `/etc/zdos/registry/` o su un demone ZDOS.

### 6.4. Demoni ZDOS in ZLang

È possibile implementare demoni ZDOS direttamente in ZLang, come l'esempio `zdos-chain-daemon.zlang`:

```zlang
module zdos.chain.daemon

import sys
import net

func main() {
    sys.log("chain-daemon", "starting")

    let cfg = sys.registry.get("chain.node")
    start_node(cfg)

    while true {
        tick()
        sys.sleep(1000)
    }
}

func start_node(cfg) {
    sys.log("chain-daemon", "node id=" + cfg["id"])
    // init networking, peers, ecc.
}

func tick() {
    // logica periodica
}
```

Questi demoni possono essere avviati da ZDOS all'avvio del sistema.

## 7. Package Manager ZPM

**zpm** è il package manager per ZLang, gestisce dipendenze e build dei progetti.

### 7.1. File di Configurazione

Ogni progetto ZLang utilizza un file `zpm.toml` per la configurazione:

```toml
[package]
name = "chain-node"
version = "0.1.0"
entry = "src/main.zlang"

[deps]
net = "core"
sys = "core"
```

### 7.2. Comandi

| Comando      | Descrizione                                       |
| :----------- | :------------------------------------------------ |
| `zpm init`   | Inizializza un nuovo progetto ZLang               |
| `zpm build`  | Compila il progetto in bytecode (`build/chain-node.zbc`) |
| `zpm run`    | Esegue lo script principale (`src/main.zlang`)    |

## 8. Integrazione con Discord

L'integrazione con Discord avviene tramite un repository separato, `zdos-discord-bridge`.

### 8.1. Architettura

-   **Repo**: `zdos-discord-bridge`
-   **Componenti**:
    -   Bot Discord (Node.js o Python)
    -   Daemon locale `zlang-daemon` che riceve richieste (es. via HTTP/gRPC), esegue script ZLang e restituisce output/log.

### 8.2. Flusso

1.  L'utente su Discord invia un comando: `!zdos run chain.status`
2.  Il Bot Discord chiama `zlang-daemon` con lo script da eseguire (o il nome di uno script pre-registrato) e i parametri.
3.  `zlang-daemon` esegue:

    ```bash
    zlang run /opt/zdos/scripts/chain_status.zlang
    ```

4.  L'output viene raccolto e rimandato al bot Discord.

### 8.3. Sicurezza

Per garantire la sicurezza:

-   Solo script whitelisted in `/etc/zdos/discord-allowed-scripts.json` possono essere eseguiti.
-   Nessuna esecuzione arbitraria di codice inviato da Discord è permessa.
-   I ruoli Discord sono mappati a permessi (es. gli admin possono avviare/fermare demoni).

## 9. Esempio Completo: `chain_node.zlang`

Un esempio di nodo blockchain implementato in ZLang:

```zlang
module chain.node

import sys
import net

func main() {
    sys.log("chain.node", "boot")

    let cfg = sys.registry.get("chain.node")
    sys.log("chain.node", "id=" + cfg["id"] + " port=" + str(cfg["port"]))

    init_network(cfg)
    loop()
}

func init_network(cfg) {
    sys.log("chain.node", "init network")
    // net.bind, net.connect peers, ecc.
}

func loop() {
    while true {
        process_incoming()
        produceblockif_needed()
        sys.sleep(500)
    }
}

func process_incoming() {
    // placeholder: leggere messaggi, tx, ecc.
}

func produceblockif_needed() {
    // placeholder: logica consenso/produzione blocchi
}
```

## 10. Roadmap di Implementazione

La roadmap per l'implementazione di ZLang e del suo ecosistema include i seguenti passi:

1.  Creazione del repository `zdos-zlang`.
2.  Implementazione del lexer secondo la grammatica definita.
3.  Implementazione del parser per generare l'AST.
4.  Implementazione di un interprete diretto dell'AST per il debug iniziale.
5.  Definizione del formato bytecode (`bytecode-spec.md`).
6.  Implementazione del codegen (AST → bytecode).
7.  Implementazione della VM stack-based con il set di istruzioni base.
8.  Implementazione delle syscalls ZDOS (`sys.log`, `sys.exec`, `sys.registry.`, `sys.event.`).
9.  Scrittura della libreria standard base (`sys`, `net`, `fs`) in parte nativa, in parte ZLang.
10. Implementazione della CLI `zlang` (`run`, `build`, `exec`).
11. Integrazione in ZDOS (percorsi di installazione, wrapper shell, demoni ZLang).
12. Creazione di `zpm` (parsing `zpm.toml`, comandi `init`, `build`, `run`).
13. Creazione del repository `zdos-discord-bridge` (bot Discord, daemon `zlang-daemon`, whitelist script).
14. Scrittura di script reali (`chain_node.zlang`, `chain_status.zlang`, script di manutenzione ZDOS).

Questo percorso fornisce una base solida per iniziare a scrivere codice senza ulteriori decisioni concettuali.
