# Zlang by ZDOS

## Linguaggio, runtime e contratto eseguibile per un ecosistema verificabile

**Zlang by ZDOS** è il linguaggio sperimentale progettato per descrivere programmi e contratti eseguibili all’interno dell’ecosistema ZDOS. Il progetto non nasce come linguaggio general-purpose tradizionale, né come semplice scripting language per una shell: nasce come collegamento controllato tra un sorgente dichiarativo, un formato bytecode versionato e un runtime integrato in un sistema operativo sperimentale.

La sua idea centrale è semplice:

> Un’istruzione non deve essere considerata disponibile perché il parser la riconosce. Deve avere un formato definito, un comportamento deterministico, un limite di sicurezza e una prova riproducibile nel runtime ZDOS.

Zlang è quindi contemporaneamente **linguaggio**, **compilatore**, **formato di bytecode**, **contratto ABI tra compilatore e kernel** e, nel futuro dell’ecosistema, livello per policy e capability verificabili.

## Obiettivo del progetto

L’obiettivo di Zlang è fornire a ZDOS un linguaggio piccolo, controllabile e adatto a un ambiente dove il boot, il runtime e le attestazioni devono essere osservabili. Il primo percorso concreto è quello in cui un file `.zlang` viene compilato in bytecode, trasformato in un header C, incorporato nel kernel bare-metal x86_64 e interpretato durante il boot in QEMU.

La catena operativa è:

```text
Sorgente .zlang
      ↓
Compilatore zlangc.py
      ↓
Bytecode ZLB2 v2.5
      ↓
Header C generato
      ↓
Kernel bare-metal ZDOS
      ↓
Runtime bounds-checked
      ↓
Output seriale e verifica QEMU
```

Questo percorso è importante perché evita una promessa astratta. Il programma non viene soltanto “analizzato”: viene compilato, incorporato in un’immagine bootabile, caricato dal kernel e verificato attraverso un output osservabile.

## Posizione di Zlang nell’ecosistema

| Componente | Responsabilità |
|---|---|
| **Zlang** | Sintassi, compilazione, formato bytecode e contratti deterministici |
| **ZDOS bare-metal** | Boot Multiboot2, kernel freestanding, seriale e runtime ZLB2 |
| **ZDOS Linux** | Distro live con kernel Linux, BusyBox, initramfs e shell |
| **Evidence Chain** | Attestazioni di build, boot, policy, revoca e audit |
| **ZDOS-SEC** | Policy, identità operative, audit e interfaccia amministrativa |
| **CI/QEMU** | Prova automatica della compilazione e dell’esecuzione |

Zlang non deve avere accesso diretto a shell, file, rete o credenziali. Ogni futura interazione con il sistema deve passare da una **capability esplicita**, con allowlist, quota, timeout, audit e test negativo.

## Il compilatore `zlangc.py`

Il compilatore di riferimento si trova nel repository Zlang in `tools/zlangc.py`. Il suo ruolo è trasformare un sorgente Zlang in due artefatti coordinati:

1. un file di bytecode binario, usato per ispezione e test;
2. un header C, incluso dal kernel durante la compilazione bare-metal.

Il comando concettuale è:

```sh
python3 tools/zlangc.py examples/hello.zlang \
  --bytecode /tmp/hello.zlb \
  --header /tmp/hello.h
```

L’header generato espone il buffer bytecode che il runtime ZDOS legge. Questo passaggio è un confine importante: il kernel non interpreta direttamente testo sorgente e non esegue un parser dinamico durante il boot. Riceve un buffer binario già prodotto dalla toolchain.

## Formato ZLB2 v2.5

Nel percorso bare-metal corrente di ZDOS, il bytecode è trattato come un contenitore binario ZLB2 v2.5. Il formato inizia con un’intestazione fissa:

| Offset | Dimensione | Campo | Valore atteso |
|---:|---:|---|---|
| 0 | 4 byte | Magic | `ZLB2` |
| 4 | 1 byte | Versione maggiore | `2` |
| 5 | 1 byte | Versione minore | `5` |
| 6… | variabile | Record | Opcode, lunghezza little-endian, payload |

Ogni record è composto da un opcode di un byte, una lunghezza `u16` little-endian e un payload della lunghezza dichiarata. Il runtime calcola il limite del record prima di leggere il payload.

Il record terminale è `HALT`:

```text
opcode = 0xff
payload length = 0
```

Il runtime accetta `HALT` soltanto quando il record occupa esattamente la fine del buffer. Byte aggiuntivi dopo la terminazione vengono rifiutati.

Gli opcode previsti dal profilo runtime corrente sono:

| Opcode | Nome | Stato nel bootstrap |
---:|---|---|
| `0x01` | `EMIT` | Eseguito e inviato alla console seriale |
| `0x02` | `LET` | Validato e non ancora eseguito come storage generale |
| `0x03` | `IF` | Validato, capability futura |
| `0x04` | `LABEL` | Validato, capability futura |
| `0x05` | `WAIT` | Validato, capability futura |
| `0xff` | `HALT` | Eseguito come terminazione deterministica |

La distinzione tra “validato” ed “eseguito” è intenzionale. Un runtime serio non deve trasformare automaticamente ogni opcode riconosciuto in una capability privilegiata.

## Runtime nel kernel ZDOS

Il runtime bare-metal si trova in `os/x86_64/kernel/zlang.c`. La sua responsabilità è:

- leggere il buffer generato dal compilatore;
- verificarne magic e versione;
- controllare che ogni header di record sia completo;
- controllare che la lunghezza non superi il buffer disponibile;
- rifiutare opcode sconosciuti;
- eseguire `EMIT` in modo limitato;
- richiedere un `HALT` valido alla fine del programma;
- produrre marker seriali utili alla CI e al debug.

Il runtime non deve fidarsi di puntatori o lunghezze fornite dal bytecode. Il controllo corretto non è soltanto “l’indice è minore della dimensione”; è necessario verificare anche che il record header e il payload rientrino entrambi nel buffer.

La sequenza di boot osservabile comprende messaggi come:

```text
Zlang runtime ZLB2 v2.5 ready
ZDOS: Esecuzione bytecode nativo in corso...
Zlang HALT accepted
ZDOS: native Zlang program executed
ZDOS: Zlang halted cleanly
```

Questi messaggi provano l’esecuzione del percorso di test, non certificano automaticamente anonimato, sicurezza assoluta, connessione Tor o protezioni anti-fingerprinting.

## Sintassi e stato del linguaggio

Zlang è ancora in fase sperimentale. Il percorso documentato originariamente nel README di Zlang presenta una sintassi minima basata su commenti, righe vuote e istruzioni `emit`:

```zlang
# examples/hello.zlang
emit Ciao dal programma Zlang nativo
emit Il kernel ZDOS ha eseguito questo bytecode
```

Il sorgente viene convertito in record bytecode, non eseguito direttamente come testo. Questo consente di controllare il formato prima del passaggio nel kernel.

Variabili, funzioni, moduli, tipi, controllo di flusso completo, gestione dello heap, eccezioni, filesystem, rete, processi e syscall pubbliche non devono essere presentati come funzionalità general-purpose già disponibili. Nel profilo ZDOS attuale alcune forme di record vengono generate e validate, ma non costituiscono ancora un linguaggio applicativo completo.

## Sicurezza: modello default-deny

Il principio di sicurezza di Zlang è **default-deny**. Il runtime deve rifiutare tutto ciò che non appartiene al contratto esplicito.

Sono rifiutabili almeno i seguenti casi:

| Caso negativo | Comportamento corretto |
|---|---|
| Magic errata | Rifiuto del programma |
| Versione incompatibile | Rifiuto esplicito |
| Record troncato | Nessuna lettura oltre il buffer |
| Payload più lungo del buffer | Rifiuto |
| Opcode sconosciuto | Rifiuto |
| `HALT` con payload | Rifiuto |
| Byte dopo `HALT` | Rifiuto |
| Programma senza `HALT` | Rifiuto |
| Capability non autorizzata | Nessuna esecuzione implicita |

La verifica di integrità del formato non dimostra però l’autenticità della provenienza. Per questo servono firme, identità dei builder, policy e attestazioni esterne.

## Integrazione con la ZDOS Evidence Chain

La Evidence Chain ZDOS registra eventi operativi in un ledger append-only con hash concatenati. Zlang può diventare il linguaggio dei contratti che valutano questi eventi, mentre il ledger conserva la sequenza e la prova di integrità.

Un’attestazione di build può contenere:

```json
{
  "type": "build.attestation",
  "subject": "sha256:artifact",
  "source_commit": "git-commit-id",
  "builder": "did:zdos:ci",
  "toolchain": "zlang-zlb2-2.5",
  "policy": "policy://release/production-v1",
  "result": "verified"
}
```

Il ruolo di Zlang non sarebbe firmare segreti o gestire denaro, ma determinare se un evento rispetta una policy: versione ammessa, builder autorizzato, test richiesti, risultato valido e revoca non presente.

## Integrazione con ZDOS-SEC

ZDOS-SEC può amministrare policy e audit, ma deve rimanere separato dal runtime privilegiato. Un portale web non deve essere considerato una prova di sicurezza soltanto perché visualizza una schermata verde.

Il modello corretto è:

```text
ZDOS-SEC policy
      ↓
Evento firmato
      ↓
Evidence Chain
      ↓
Contratto Zlang
      ↓
Decisione deterministica
      ↓
ZDOS runtime / release verifier
```

Password, chiavi private, dati personali e contenuti riservati devono restare fuori dal bytecode e fuori dal ledger. Il portale deve distinguere tra configurazione, raggiungibilità e sicurezza verificata.

## Build e verifica end-to-end

Per il percorso ZDOS x86_64, Zlang e ZDOS devono essere directory affiancate:

```sh
git clone https://github.com/high-cde/Zlang.git
git clone https://github.com/high-cde/ZDOS.git

cd ZDOS/os/x86_64
make clean
make verify
sh tools/verify_qemu.sh
```

Il repository ZDOS include inoltre un controllo del contratto ZLB2 che esamina l’header generato prima del boot. La pipeline completa deve verificare:

| Fase | Prova |
|---|---|
| Compilatore | Bytecode e header generati senza errori |
| Contratto | Magic, versione, record e `HALT` validi |
| Kernel | Compilazione freestanding senza warning trattati come errori |
| ISO | Packaging GRUB valido |
| QEMU | Boot Multiboot2 e output seriale |
| Runtime | Marker di esecuzione e terminazione pulita |
| CI | Ripetizione automatica su ambiente pulito |

## Stato reale del progetto

Zlang by ZDOS è oggi classificabile come **prototipo verificato di linguaggio/runtime**, non come linguaggio general-purpose e non come piattaforma applicativa completa.

| Capacità | Stato |
|---|---|
| Sintassi sorgente minima | Disponibile nel percorso documentato |
| Compilatore Python | Disponibile nel repository Zlang |
| Bytecode versionato | Disponibile nel percorso ZDOS ZLB2 v2.5 |
| Header C | Generato per l’integrazione kernel |
| Runtime bare-metal | Disponibile e verificabile in QEMU |
| `EMIT` | Eseguito realmente |
| Validazione bounds-checked | Disponibile nel runtime ZDOS |
| Variabili e controllo di flusso generale | Non ancora completati |
| App esterne e loader persistente | Non disponibili |
| Syscall pubbliche | Non disponibili |
| PKI multi-organizzazione | Non ancora implementata |
| Consenso distribuito | Non ancora implementato |

### Nota di coerenza documentale

Il contratto documentato per l’integrazione bare-metal corrente è ora **ZLB2 v2.5**. Il README e il profilo tecnico del repository Zlang sono stati riallineati a questa versione; eventuali riferimenti a ZLB0 appartengono soltanto alla storia del progetto e non devono essere usati come contratto corrente.

## Roadmap tecnica

### Fase A — Stabilizzazione ZLB2

La prima priorità è fissare uno schema formale del formato: header, opcode, endianness, limiti, errori e compatibilità. Il compilatore e il runtime devono essere testati contro gli stessi vettori validi e non validi.

### Fase B — Capability limitate

Le prime capability dovrebbero essere conservative: emissione log strutturato, lettura di tempo monotono e accesso a input dichiarati. Ogni capability deve avere allowlist, quota, timeout e record di audit.

### Fase C — Valori locali

Variabili e aritmetica possono essere aggiunte con limiti di memoria, controllo overflow e semantica deterministica. Non devono introdurre automaticamente accesso a file o rete.

### Fase D — Programmi multipli

Solo dopo un runtime stabile si può introdurre l’esecuzione di più programmi, con scheduler cooperativo, limiti temporali e isolamento esplicito.

### Fase E — Distribuzione

L’ultima fase riguarda loader esterno, immagini firmate, package manager, aggiornamenti con rollback, attestazioni Evidence Chain e policy ZDOS-SEC.

## Conclusione

**Zlang by ZDOS** è il tentativo di costruire un linguaggio il cui valore non dipenda dalla quantità di sintassi, ma dalla verificabilità della catena completa. Il programma deve essere leggibile, compilabile, rappresentabile in un formato versionato, rifiutabile quando è malformato, eseguibile entro capability limitate e collegabile a una prova osservabile.

Il progetto è ancora giovane, ma la direzione è concreta: dal sorgente Zlang al bytecode ZLB2, dal bytecode al runtime ZDOS, dal runtime alla prova QEMU e dalla prova all’attestazione della Evidence Chain. La maturità arriverà non aggiungendo claim più grandi, ma trasformando ogni promessa in un contratto, un’implementazione, un test positivo, un test negativo e un limite dichiarato.

> **Zlang + ZDOS — Build what you can prove.**

## Riferimenti

[1]: https://github.com/high-cde/Zlang "Repository ufficiale Zlang"
[2]: https://github.com/high-cde/ZDOS "Repository ufficiale ZDOS"
[3]: https://github.com/high-cde/ZDOS/blob/main/os/x86_64/kernel/zlang.c "Runtime ZLB2 nel kernel ZDOS"
[4]: https://github.com/high-cde/ZDOS/blob/main/evidence/README.md "ZDOS Evidence Chain"
[5]: https://github.com/high-cde/ZDOS/blob/main/docs/FOUNDATION.md "ZDOS Foundation"
[6]: https://github.com/high-cde/ZDOS-SEC-PORTAL "Repository ZDOS-SEC-PORTAL"
