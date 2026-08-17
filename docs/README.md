# Documentazione tecnica di ZLang

Questa directory raccoglie le specifiche e le guide operative di ZLang. Il Core v1 è una filiera concreta: sorgente `.zl`, compilatore, modulo bytecode `ZREG`, VM a registri, policy di capability e audit. La documentazione distingue questo perimetro implementato dalle estensioni ancora progettuali.

| Documento | Scopo |
|---|---|
| [Whitepaper](https://raw.githubusercontent.com/high-cde/Zlang/main/ZLANG-WHITEPAPER.md) | Visione, posizionamento, sicurezza, casi d’uso e roadmap strategica |
| [Specifica del linguaggio](https://raw.githubusercontent.com/high-cde/Zlang/main/docs/language-spec.md) | Sintassi, tipi, costrutti e grammatica di riferimento |
| [Specifica del bytecode](https://raw.githubusercontent.com/high-cde/Zlang/main/docs/bytecode-spec.md) | Formato degli artefatti e modello di esecuzione della VM |
| [Syscall ZDOS](https://raw.githubusercontent.com/high-cde/Zlang/main/docs/syscalls.md) | Confine previsto tra runtime ZLang e capacità del sistema operativo |
| [Checklist release stabile](https://raw.githubusercontent.com/high-cde/Zlang/main/docs/STABLE-RELEASE-CHECKLIST.md) | Gate P0/P1, milestone, release candidate e decisione go/no-go per `v1.0.0` |
| [Architettura](./wiki/Architecture.md) | Principi organizzativi del repository |
| [Operazioni](./wiki/Operations.md) | Guida operativa e manutenzione |
| [Roadmap](./wiki/Roadmap.md) | Priorità di sviluppo e percorso di consolidamento |

> **Stato del progetto:** ZLang è un prototipo avanzato con Core ZREG v1 funzionante. La documentazione distingue esplicitamente le componenti implementate dal percorso architetturale ancora in evoluzione.
