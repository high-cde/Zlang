# Specifica del linguaggio ZLang — Core v1

**Stato:** implementato nel compilatore e nella VM a registri di ZLang `v2026.2.0`.

Questa specifica descrive il **core realmente eseguibile**. Moduli, stringhe, liste, mappe, funzioni, eccezioni, rete, registry, filesystem, ZPM e syscall di sistema appartengono alla roadmap progettuale e non fanno parte del contratto stabile del core v1.

## 1. Obiettivo del core

ZLang Core v1 è un linguaggio minimale e deterministico per espressioni intere. Compila sorgente `.zl` in bytecode `ZREG` e lo esegue attraverso una VM con registri limitati, checksum, policy di capability e audit trail.

| Caratteristica | Stato v1 |
|---|---|
| Interi `i64` | Implementata |
| Variabili `let` | Implementata |
| Operatori `+ - * /` | Implementati |
| Negazione unaria | Implementata |
| Precedenza e parentesi | Implementate |
| `print` | Implementato; richiede `ConsoleWrite` |
| Commenti `#` | Implementati |
| Moduli/funzioni/cicli | Non implementati |
| Stringhe/collezioni | Non implementate |
| Syscall host/rete/filesystem | Non implementate |

## 2. Esempio eseguibile

```zlang
# Telemetria numerica deterministica.
let altitude = 408
let correction = altitude / 6
let result = correction + 4
print result
```

Compilazione ed esecuzione:

```bash
zlang compile telemetry.zl telemetry.zreg
zlang exec telemetry.zreg --audit
```

## 3. Lessico

Gli identificatori iniziano con una lettera ASCII o `_` e possono contenere lettere, cifre e `_`. I letterali numerici sono interi decimali firmati tramite negazione unaria. Gli spazi sono ignorati; newline e `;` separano le istruzioni. Un commento inizia con `#` e prosegue fino alla fine della riga.

| Token | Significato |
|---|---|
| `let` | Introduce un binding immutabile nel core v1 |
| `print` | Emette il valore di un’espressione sull’output autorizzato |
| `+ - * /` | Operatori aritmetici |
| `=` | Assegnazione nell’istruzione `let` |
| `(` `)` | Raggruppamento delle espressioni |
| `#` | Inizio commento |

## 4. Grammatica EBNF

```ebnf
program        = { newline } , { statement , { newline } } , EOF ;
statement      = let_statement | print_statement ;
let_statement  = "let" , identifier , "=" , expression ;
print_statement= "print" , expression ;
expression     = term , { ("+" | "-") , term } ;
term           = factor , { ("*" | "/") , factor } ;
factor         = integer | identifier | "-" , factor | "(" , expression , ")" ;
identifier     = ( letter | "_" ) , { letter | digit | "_" } ;
integer        = digit , { digit } ;
newline        = "\n" | ";" ;
```

## 5. Semantica

Le espressioni usano aritmetica intera signed a 64 bit. `*` e `/` hanno precedenza su `+` e `-`; operatori con la stessa precedenza sono valutati da sinistra a destra. Le variabili sono risolte dal compilatore e mappate su registri della VM; un identificatore non definito produce un errore di compilazione.

Il core v1 dispone di un massimo di 16 registri fisici per modulo. Un programma o un’espressione che richieda più registri viene rifiutato in compilazione con un errore esplicito; non viene eseguito alcun spilling implicito su memoria host.

Overflow aritmetico e divisione per zero generano un errore runtime controllato. Non causano panic del processo Rust e vengono registrati nell’audit della VM.

## 6. Errori

| Categoria | Esempio | Codice CLI |
|---|---|---:|
| Sorgente | Carattere non supportato, parentesi mancante | `65` |
| Compilazione | Variabile non definita, budget registri superato | `65` |
| I/O | File sorgente o modulo non leggibile | `66` |
| Runtime | Divisione per zero, overflow, capability negata | `70` |
| Utilizzo | Comando CLI non valido | `64` |

## 7. Estensioni non implementate

Le seguenti aree restano intenzioni di progetto e non devono essere invocate dal core v1: funzioni, moduli, import, tipi compositi, eccezioni, byte/string literal, controllo di flusso, processo, filesystem, rete, registry, telemetria orbitale, Z-Chain e package management. Ogni estensione richiederà una proposta di linguaggio, semantica bytecode, capability, limiti di risorsa, test e aggiornamento di versione prima di essere promossa nel core stabile.
