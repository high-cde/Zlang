# Profilo Zlang ZLB2 v2.5 per ZDOS x86_64

Il compilatore `tools/zlangc.py` implementa il profilo iniziale di bytecode **ZLB2 v2.5**. Il suo obiettivo non è sostituire la futura toolchain Zlang completa: fornisce un contratto piccolo, eseguibile e verificato per il primo prototipo ZDOS avviabile in QEMU.

## Utilizzo

```sh
python3 tools/zlangc.py esempio.zlang \
  --header /percorso/zlang_program.h \
  --bytecode /percorso/esempio.zlb
```

Il compilatore produce un bytecode binario e un header C destinato al kernel freestanding. Il kernel ZDOS può così eseguire lo stesso programma Zlang durante il boot, senza delegare l’esecuzione a Linux o a un interprete esterno.

## Linguaggio attivo

L’unica istruzione attiva è:

```zlang
emit Messaggio da stampare sulla console seriale
```

Commenti introdotti da `#` e righe vuote sono consentiti. Variabili, funzioni, moduli, filesystem, rete, processi e syscall non appartengono ancora a questo profilo; la loro sintassi viene rifiutata dal compilatore.

## Formato ZLB2 v2.5

| Campo | Valore |
|---|---|
| Magic | `ZLB2` |
| Versione | `2.5` |
| Opcode `0x01` | `EMIT`, seguito da `u16` little-endian e payload UTF-8 |
| Opcode `0xff` | `HALT` |

Il contratto completo, i controlli di integrità e i limiti del prototipo sono documentati in `ZDOS/os/x86_64/ARCHITECTURE.md`. Ogni estensione futura deve introdurre opcode, semantica, test negativi e una strategia di compatibilità versionata prima di essere dichiarata disponibile.
