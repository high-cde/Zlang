# ABI syscall ZDOS — Stato del core ZREG v1

**Stato attuale:** nessuna syscall host è implementata o esposta dalla VM ZREG v1.

Il core a registri può eseguire soltanto operazioni aritmetiche e `EMIT`, quest’ultima limitata alla capability `ConsoleWrite`. Non esistono istruzioni per shell, filesystem, processi, registry, rete, tempo, telemetria o Z-Chain. Un modulo ZREG v1 non può ottenere tali accessi attraverso bytecode valido.

## Capability implementate

| Capability | Istruzione | Effetto | Stato |
|---|---|---|---|
| `ConsoleWrite` | `EMIT src` | Aggiunge il valore di un registro all’output della VM | Implementata |

`EMIT` viene eseguita solo se la capability è dichiarata nell’header del modulo e autorizzata dalla `CapabilityPolicy` del runtime. Ogni diniego viene registrato nell’audit.

## Principi vincolanti per una syscall futura

Una syscall ZDOS potrà entrare in una versione successiva solo con tutti i seguenti elementi: identificatore stabile, versione ABI, capability dedicata, contratto argomenti/risultato, errori deterministici, limiti di risorsa, audit event, policy predefinita deny-by-default, test negativi e documentazione aggiornata.

| Area proposta | Capability futura indicativa | Stato |
|---|---|---|
| Logging strutturato | `LogWrite` | Non implementata |
| Filesystem | `FsRead`, `FsWrite` con scope path | Non implementata |
| Processi | `ProcessSpawn` con allowlist | Non implementata |
| Registry | `RegistryRead`, `RegistryWrite` con scope key | Non implementata |
| Networking | `NetConnect`, `NetSend`, `NetReceive` con endpoint policy | Non implementata |
| Eventi e tempo | `EventEmit`, `ClockRead` | Non implementata |
| Ledger/Z-Chain | `LedgerSubmit` con policy firmata | Non implementata |

> Una richiesta proveniente da un sorgente ZLang, modulo bytecode o canale remoto non riceverà accesso host implicito. L’ABI futuro resterà deny-by-default e potrà essere abilitato soltanto dalla policy del deployer.
