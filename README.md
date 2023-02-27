# E-voting alternatif

Implémentation d’un POC d’un système de e-voting avec la méthode Borda.

## SÉCURITÉ

CE PROJET EST INACTIF, AUCUNE MISE À JOURS DE SÉCURITÉ N'EST EFFECTUÉ.

## Fonctionnalité

 - Modèle de rapport de TB
 - Construction de PDF indiquant les différances entre 2 commits.
 - Construction uniquement d'un sous-fichier LaTeX.
 - Indication du tag git de version (v[0-9]*) dans le PDF.
 - Auto publication de pre-release lors du push sur main.
 - Auto publication de release lors du push sur avec un tag de version.

## Dépendance

### POC

- `cargo` et `rust` en nightly

```sh
# installation de rustup, cargo et rustc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# passage en nightly
rustup toolchain install nightly
rustup default nightly
```

### Rapport

Le paquet `texlive-full` (sous Debian/Ubuntu) contient toutes les dépendences. Une installation plus
spécifique peut fonctionner aussi, mais n'est pas documenté.

Le Makefile utilise les programmes suivants (doivent être dans le path) :
 - `find`
 - `latexdiff` et `latexdiff-vc`
 - `sed`
 - `make`
 - `latexmk`
 - `zip`
 - `cargo`, `rustc`

La construction peut également se faire à travers un container docker, grâce au scipt `make-on-docker.sh`
qui remplace la commande `make` dans les examples suivants. (Seul docker est une dépendance dans ce cas)

### Execution du POC

Lancement du serveur

```sh
cd app
cargo run --bin board
```

L’interface d’administration du serveur se trouve sur http://localhost:8085/, grâce à cette interface l’ajout d‘option et de votant est possible.

Lancement du client

```sh
cd app
cargo run --bin client
```

### Test de performence

```sh
cd app && cargo run --release --bin nizk-phase1
cd app && cargo run --release --bin nizk-phase2
cd app && cargo run --release --bin phase-1-size
cd app && cargo run --release --bin result-1
cd app && cargo run --release --bin result-2
```

## Construction du rapport

### Rapport

Attention la construction dépends dur dépôt git, voir section `Dépôt git`

Pour construire tout le rapport:

```sh
make report
```

Pour construire tout les PDFs de base:

```sh
make pdf
```

Pour automatiquement reconstruire le rapport lorsqu'un fichier change:

```sh
cd latex && make report.pdf.pvc
```

Pour construire juste une sous-partie du rapport:

```sh
cd latex && make <dir>/<subfile>.pdf
```
