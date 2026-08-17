# Specifica del bytecode ZLang — ZREG v1

**Stato:** implementato nel core runtime `v2026.2.0`.
**Formato modulo:** `ZREG`.
**Modello di esecuzione:** macchina virtuale deterministica a registri, interamente implementata nel processo Rust di ZLang.

> ZREG non è una CPU hardware né un sostituto dell’isolamento del kernel. È un contratto di esecuzione verificabile: il runtime convalida il modulo prima dell’avvio, limita le risorse interne, richiede capability esplicite e registra l’esito di ogni istruzione.

## 1. Modello della VM

La VM dispone di un file fisso di **16 registri interi a 64 bit**, denominati `R0`–`R15`. Il compilatore assegna i registri al core linguistico supportato; il modulo dichiara quanti registri usa, fino al massimo del runtime. Non sono presenti accessi diretti a memoria host, puntatori, codice nativo, shell o rete nel core ZREG v1.

| Elemento | Contratto v1 |
|---|---|
| Valore | Intero signed a 64 bit |
| Registri massimi | 16 |
| Instruction pointer | Interno alla VM; avanza in ordine lineare nel v1 |
| Heap e stack guest | Non esposti nel core v1 |
| Risorse | Limite istruzioni e limite output configurabili nel runtime |
| Arithmetics | Controllata: overflow e divisione per zero sono errori runtime |
| Host access | Nessuna capability host disponibile nel core v1 |
| Audit | Un evento per istruzione, inclusi dinieghi e fallimenti |

Il comportamento è deterministico per lo stesso modulo ZREG, policy di capability e limiti runtime. Il consumo effettivo di CPU del processo resta soggetto al sistema operativo ospite; per isolamento di processi ostili sono necessari limiti OS/container aggiuntivi.

## 2. Formato binario

Tutti gli interi multibyte sono little-endian. Il checksum è SHA-256 dell’intero payload precedente al checksum.

| Offset | Campo | Dimensione | Descrizione |
|---|---:|---:|---|
| `0` | Magic | 4 byte | ASCII `ZREG` |
| `4` | Version | 2 byte | Versione formato; v1 = `1` |
| `6` | Register count | 1 byte | Registri dichiarati, tra `1` e `16` |
| `7` | Capability count | 1 byte | Numero di capability dichiarate |
| `8` | Code length | 4 byte | Lunghezza della sezione codice in byte |
| `12` | Capabilities | N byte | ID capability, ordinati e senza duplicati |
| `12 + N` | Code | variabile | Istruzioni codificate |
| finale | Checksum | 32 byte | SHA-256 del payload |

Il decoder rifiuta magic errato, versione incompatibile, checksum invalido, byte inattesi, capability sconosciute, registri fuori range, `HALT` non terminale e moduli senza `HALT` conclusivo.

## 3. Capability

| ID | Nome | Operazione protetta | Stato v1 |
|---:|---|---|---|
| `1` | `ConsoleWrite` | Istruzione `EMIT` verso output standard | Implementata |

Una capability deve essere sia dichiarata dal modulo sia autorizzata dalla `CapabilityPolicy` del runtime. La policy predefinita usata dalla CLI abilita solo `ConsoleWrite`; `deny_all()` rifiuta ogni capacità. Le syscall di filesystem, rete, processi, registry e telemetria non sono implementate nella VM v1 e non devono essere considerate disponibili.

## 4. Istruzioni v1

| Opcode | Istruzione | Operandi | Semantica |
|---:|---|---|---|
| `0` | `HALT` | — | Arresto normale; deve essere l’ultima istruzione |
| `1` | `LOAD_IMM` | `dest:u8`, `value:i64` | Carica un letterale intero in `dest` |
| `2` | `MOV` | `dest:u8`, `src:u8` | Copia il valore di `src` in `dest` |
| `3` | `ADD` | `dest,left,right:u8` | Somma controllata |
| `4` | `SUB` | `dest,left,right:u8` | Sottrazione controllata |
| `5` | `MUL` | `dest,left,right:u8` | Moltiplicazione controllata |
| `6` | `DIV` | `dest,left,right:u8` | Divisione intera controllata; zero non ammesso |
| `7` | `NEG` | `dest,src:u8` | Negazione controllata |
| `8` | `EMIT` | `src:u8` | Scrive il valore di `src`; richiede `ConsoleWrite` |

Le operazioni aritmetiche usano controlli `checked_*`. Overflow, `i64::MIN / -1` e divisione per zero non causano panic: la VM interrompe l’esecuzione con un errore strutturato e un evento di audit `Failed`.

## 5. Audit e limiti

Ogni istruzione genera un evento con sequenza, instruction pointer, nome istruzione e uno dei seguenti esiti:

| Esito | Significato |
|---|---|
| `Allowed` | Istruzione completata entro policy e limiti |
| `Denied` | Capability non dichiarata o rifiutata dalla policy |
| `Failed` | Errore bytecode, aritmetico o di limite runtime |

I limiti default sono `100.000` istruzioni e `64 KiB` di output. Il runtime rifiuta moduli che dichiarano già un numero di istruzioni superiore al budget. Il formato limita inoltre la sezione codice a `1 MiB`.

## 6. CLI di riferimento

```bash
# Compila ZLang core in un modulo ZREG verificato.
zlang compile program.zl program.zreg

# Convalida checksum/header e avvia la VM con policy console-only.
zlang exec program.zreg --audit

# Compila ed esegue in memoria.
zlang run program.zl --audit

# Mostra metadati del modulo senza eseguirlo.
zlang inspect program.zreg
```

## 7. Compatibilità

ZREG v1 è compatibile solo con runtime che dichiarano supporto alla versione `1`. Cambiamenti incompatibili al layout binario, alla semantica di un opcode o alla capability policy richiedono una nuova versione bytecode. L’aggiunta di opcode o capability non viene considerata compatibile con decoder v1 finché non esiste un meccanismo esplicito di feature negotiation.
