# ZDOS, Zlang e ZRetro

## Presentazione

Zlang è il compilatore e runtime bytecode del prototipo ZDOS x86_64. ZRetro è l’ambiente TUI e la DSL dichiarativa retro del repository ZDOS. Sono componenti collegati, ma non equivalenti: Zlang definisce il linguaggio e il contratto ZLB2; ZDOS fornisce il target e il runtime; ZRetro organizza progetti, scene e target retro.

> Una capacità viene dichiarata disponibile soltanto quando esistono contratto, implementazione, test e una prova riproducibile.

## Esempio Zlang

```zlang
# examples/hello.zlang
emit Ciao dal programma Zlang nativo
emit Il kernel ZDOS ha eseguito questo bytecode
```

```sh
python3 tools/zlangc.py examples/hello.zlang \
  --bytecode /tmp/hello.zlb \
  --header /tmp/hello.h
```

Il profilo ZLB2 v2.5 rifiuta la sintassi non definita. Il runtime valida magic `ZLB2`, versione, opcode, lunghezze e terminazione `HALT`.

## Esempio ZRetro

Il progetto di riferimento è [`Meteor Patrol`](https://github.com/high-cde/ZDOS/blob/main/zretro/projects/meteor-patrol/main.zretro). L’IDE locale può essere avviata dal repository ZDOS:

```sh
python3 zretro/ide/zretro.py console --root zretro/projects
python3 zretro/ide/zretro.py run zretro/projects/meteor-patrol/main.zretro
python3 zretro/ide/zretro.py build zretro/projects/meteor-patrol/main.zretro
```

La prima versione produce IR e manifest verificabili. I backend per binari nativi, emulatori e target hardware sono attività successive e non devono essere presentati come già disponibili.

## Contratto tra i repository

| Componente | Contratto |
|---|---|
| Zlang | File `.zlang`, compilatore `zlangc.py`, bytecode ZLB2 v2.5 |
| ZDOS | Runtime bare-metal, target x86_64, boot QEMU e CI |
| ZRetro | File `.zretro`, IR, manifest e target retro dichiarati |
| Hub | Pagina pubblica documentaria; nessuna build remota o capability implicita |

La pagina pubblica collegata dal tasto **ZRetro** è [zdos-hub.it/retro-computing.html](https://zdos-hub.it/retro-computing.html). Il collegamento è informativo: non esegue codice, non carica ROM e non concede accesso al nodo.

## Sicurezza e diritti

Il bridge `storage.read-v1` è read-only e confinato a un namespace autorizzato. Non implica accesso raw al disco, syscall universali o scrittura persistente. ZRetro deve usare manifest espliciti e, per un futuro publish, identità ZDOS, capability `hub.project.publish` e approvazione.

La sezione retro può descrivere la storia di Atari, Commodore 64 e Amiga, ma non deve distribuire ROM, BIOS, software proprietario o materiale protetto senza autorizzazione.

## Riferimenti pubblici

* [Repository Zlang](https://github.com/high-cde/Zlang)
* [Specifiche del linguaggio](https://github.com/high-cde/Zlang/blob/main/docs/language-spec.md)
* [Specifiche ZLB2](https://github.com/high-cde/Zlang/blob/main/docs/bytecode-spec.md)
* [Profilo ZDOS x86_64](https://github.com/high-cde/Zlang/blob/main/docs/zdos-x86_64-profile.md)
* [Repository ZDOS](https://github.com/high-cde/ZDOS)
* [Documentazione ZRetro](https://github.com/high-cde/ZDOS/blob/main/docs/ZRETRO.md)
* [Demo Meteor Patrol](https://github.com/high-cde/ZDOS/blob/main/zretro/projects/meteor-patrol/main.zretro)
* [Pagina Retro Computing](https://zdos-hub.it/retro-computing.html)

**Zlang · ZDOS · ZRetro — Build what you can prove.**
