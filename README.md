
TSPTW — Solveur Rust pour Traveling Salesman Problem with Time Windows
=====================================================================

Brève description
-----------------
Bibliothèque et application en Rust pour résoudre des instances de TSPTW (Traveling Salesman Problem with Time Windows).
Le projet contient plusieurs algorithmes d'optimisation, des outils d'évaluation, un petit moteur HPO et une interface graphique légère.

Lancer l'interface graphique
---------------------------
Pour démarrer l'interface graphique, exécuter depuis la racine du dépôt :

```bash
cargo run --release -- --gui
```

Notes :
- Le binaire principal accepte l'option `--gui` pour lancer la GUI.
- L'exécution en mode release est recommandée pour de meilleures performances.

Utilisation en ligne de commande
-------------------------------
Le même exécutable peut être utilisé en mode console pour lancer des expériences ou résoudre des instances sans interface. Lancez :

```bash
cargo run --release -- --help
```

Structure du dépôt (aperçu)
--------------------------
- `src/algorithms/` : implémentations d'algorithmes méta-heuristiques (ACO, GA, Hill Climbing, Simulated Annealing, VNS).
- `src/eval/` : fonctions et métriques d'évaluation (lexicographic, weighted, random, utilitaires).
- `src/gui/` : code de l'interface graphique (fenêtres, composants, onglets, état du solveur).
- `src/hpo/` : recherche d'hyperparamètres (grid search, bayesian optimizer).
- `src/initializer/` : initialisateurs aléatoires et heuristiques pour solutions initiales.
- `src/io/` : lecture/écriture d'instances et solutions.
- `src/neighborhood/` : opérateurs de voisinage (swap, 2-opt, utils).
- `src/runner/` et `src/solver/` : orchestration des runs, stratégies combinées (ex. GA+SA).
- `src/shared/` et `src/utils/` : types partagés, utilitaires et logging.
- `data/` : instances d'exemple et solutions (fichiers `.sol` et répertoires d'instances).

Exemples et données
--------------------
Le répertoire `data/` contient plusieurs instances de test et solutions générées par des heuristiques. Utilisez-les pour expérimenter avec la GUI ou la ligne de commande.

Que fait la bibliothèque ? (en bref)
----------------------------------
- Fournit plusieurs méthodes de recherche locale et méta-heuristiques adaptées au TSPTW.
- Permet d'évaluer et comparer les solutions selon plusieurs critères (lexicographique, poids, aléatoire pour tests).
- Offre un petit cadre pour tuner les hyperparamètres (HPO) et pour composer des stratégies (ex. initialisation + recherche locale).
- Propose une interface graphique pour visualiser routes, fenêtres temporelles et statistiques de performance.

Contribuer
---------
Les contributions sont bienvenues : bugs, améliorations, nouvelles heuristiques ou visualisations. Ouvrez une issue ou une pull request.

Licence
-------
Voir le fichier `LICENSE` si présent dans le dépôt.

Exemples rapides
----------------
1. Lancer l'interface graphique (exemple minimal) :

```bash
cargo run --release -- --gui
```

2. Obtenir l'aide en ligne de commande pour voir les options disponibles :

```bash
cargo run --release -- --help
```

3. Données d'exemple :
- Les instances et solutions d'exemple se trouvent dans le dossier `data/` (par ex. `data/inst1/` et fichiers `.sol`).
- Pour tester rapidement, lancez la GUI puis chargez une instance depuis `data/` via l'interface (ou utilisez les options CLI si disponibles — voir `--help`).

Captures d'écran
----------------
Vous pouvez ajouter des captures d'écran de la GUI dans `docs/screenshots/` (ce dossier n'existe pas par défaut) pour illustrer :
- Visualisation des routes et fenêtres temporelles
- Onglet des métriques et statistiques
- Exemple de configuration d'un run

Si vous voulez, j'ajoute une capture d'écran de démonstration (je peux générer une image de remplacement ou intégrer une véritable capture si vous la fournissez).

