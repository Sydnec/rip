# Projet de Routage Réseau RIP

## Description

Ce projet est une simulation d'un réseau de routeurs utilisant le protocole RIP.

## Structure du Projet

La structure du projet est la suivante :
```
.
├── config
│   ├── routeur-client.yaml
│   ├── routeur-r1.yaml
│   ├── routeur-r2.yaml
│   ├── routeur-r3.yaml
│   ├── routeur-r4.yaml
│   ├── routeur-r5.yaml
│   ├── routeur-r6.yaml
│   └── routeur-serveur.yaml
└── src
    ├── config_reader.py
    ├── interface.py
    ├── main.py
    ├── neighbor.py
    ├── route.py
    ├── router.py
    ├── routing_table.py
    └── utils.py
```


### Dossiers et Fichiers

- `config/` : Ce dossier contient les fichiers de configuration YAML pour chaque routeur dans le réseau.
  - `routeur-client.yaml` : Configuration du client.
  - `routeur-serveur.yaml` : Configuration du serveur.
  - `routeur-r1.yaml` à `routeur-r6.yaml` : Configurations des différents routeurs.

- `src/` : Ce dossier contient les fichiers source du projet.
  - `config_reader.py` : Module pour lire les fichiers de configuration YAML.
  - `interface.py` : Classe pour définir et gérer les interfaces réseau des routeurs.
  - `main.py` : Point d'entrée principal du programme.
  - `neighbor.py` : Classe pour gérer les voisins des routeurs.
  - `route.py` : Classe pour définir les routes.
  - `router.py` : Classe pour définir les routeurs et leurs comportements.
  - `routing_table.py` : Classe pour gérer la table de routage des routeurs.
  - `utils.py` : Module utilitaire contenant des fonctions d'assistance.

## Installation

1. Clonez le dépôt :

```bash
git clone https://github.com/sydnec/rip.git
cd rip
```

2. Installez les dépendances requises. Assurez-vous d'utiliser un environnement virtuel pour isoler les dépendances du projet :
```bash
python -m venv env
source env/bin/activate  # Pour Windows utilisez `env\Scripts\activate`
pip install PyYAML
```

## Utilisation
Exécutez le programme principal :
```bash
python src/main.py
```
## Auteur
Sydnec