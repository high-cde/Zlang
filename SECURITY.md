# Security Policy

## Ambito

Questa policy riguarda il repository `high-cde/Zlang` e la relativa documentazione. Il progetto può includere script, automazioni, componenti sperimentali o integrazioni con sistemi esterni; per questo motivo ogni utilizzo deve avvenire in ambienti autorizzati e controllati.

## Segnalazione responsabile

Se individui un problema di sicurezza, non aprire una issue pubblica con dettagli sfruttabili. Contatta il maintainer del repository indicando una descrizione sintetica, i passaggi di riproduzione e l'impatto stimato.

## Buone pratiche

| Area | Pratica consigliata |
|---|---|
| Segreti | Non versionare token, chiavi private, password o file `.env`. |
| Dipendenze | Aggiornare librerie e toolchain con cadenza regolare. |
| Esecuzione | Testare script e automazioni in sandbox o ambienti isolati. |
| Permessi | Applicare il principio del privilegio minimo. |
| Log | Evitare la stampa di credenziali, cookie o dati personali. |

## Disclaimer operativo

Qualsiasi componente orientato a sicurezza, automazione o rete deve essere usato solo su sistemi propri o con autorizzazione esplicita. L'obiettivo del repository è documentale, sperimentale o difensivo, non l'abuso di sistemi terzi.
