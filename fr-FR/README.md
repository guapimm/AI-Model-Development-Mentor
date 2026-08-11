🌍 Autres langues → [English](../README.md)

# AI Model Mentor (Français)

> **Transformez votre assistant de codage IA en un architecte full-stack prudent fort de 10 ans d'expérience — des prompts purs, zéro dépendance.**

---

## Qu'est-ce que c'est ?

Un **framework 100 % basé sur des prompts** qui transforme votre assistant de codage IA en **architecte full-stack & mentor de développement fort de 10 ans d'expérience**, pensé pour les débutants en programmation sans aucune base.

Il impose à l'IA de suivre des « règles d'airain » — faire de la *Sécurité d'abord, de la Logique transparente, de la Documentation d'abord, de l'Efficacité des tokens et de l'Avancement par étapes* son comportement par défaut. Résultat : une IA qui ne se contente pas *d'écrire du code*, mais écrit un code **sûr, maintenable et documenté**.

> ⚠️ Actuellement pris en charge : **Xiaomi MIMO CLI**. Des versions optimisées pour d'autres produits (Claude Code, Cursor, etc.) sont prévues — laissez un commentaire si vous en avez besoin.

## Modules principaux (version Xiaomi MIMO)

| Module | Fichier | Rôle |
|--------|---------|------|
| 🧑‍🏫 Rôle de mentor | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | Persona d'architecte-mentor + 6 règles d'airain + liste de contrôle de sécurité ★ indispensable |
| 🛡️ Normes de sécurité | [security.md](./xiaomi-mimo/security.md) | 8 domaines de sécurité : clés secrètes / validation des entrées / base de données / XSS / système de fichiers / requêtes externes / gestion des erreurs / performances |
| 🎨 Style d'interaction | [style.md](./xiaomi-mimo/style.md) | Analogies de la vie courante, étiquettes de phase, confirmer avant d'agir, complexité progressive |
| 📋 Flux de développement | [workflow.md](./xiaomi-mimo/workflow.md) | Système documentaire / protocole de positionnement front-end / déploiement et retour arrière / boucle de test / ancrage de version |

### Les 6 règles d'airain

1. **Le code est la documentation** — tout le code porte des commentaires expliquant le « pourquoi »
2. **La sécurité d'abord** — pas de secrets codés en dur, validation stricte des entrées, requêtes paramétrées, protection XSS
3. **Zéro modification destructrice** — analyser les dépendances d'abord, marquer les modifications [Obligatoire] / [Optionnelle]
4. **Exécution par étapes** — jamais plus de 300 lignes par réponse, attendre la confirmation à chaque étape
5. **Isolation modulaire** — 500 lignes maximum par fichier, prévoir des interfaces d'extension
6. **Efficacité des tokens** — générer un résumé du contexte + un code de reprise après chaque conversation

## Démarrage rapide (3 étapes)

```bash
# 1. Copiez le rôle de mentor dans votre projet (renommez-le)
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2. (Recommandé) Ajoutez aussi les normes sécurité / style / workflow
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
```

3. Lancez Xiaomi MIMO et dites :

> « Je suis un vrai débutant. Voici ma Spécification des besoins du projet : nom du projet ____, objectifs principaux ____, rôles des utilisateurs ____, processus d'utilisation principaux ____, données à stocker ____. Commencez par la Phase 0 : préparation de l'environnement et choix de la pile technique, et guidez-moi étape par étape. »

L'IA avancera selon « Conception → Logique principale → Interface → Tests », en attendant votre confirmation à chaque étape.

## Structure des fichiers

```
AI_Model_Development_Mentor/
├── README.md            # Page d'accueil + sélecteur de langue
├── LICENSE              # Licence MIT
├── zh-CN/               # Chinois
├── en-US/               # Anglais
└── fr-FR/               # Français (cette page)
    └── xiaomi-mimo/     # Version Xiaomi MIMO
        ├── AGENTS.md    # Rôle de mentor (FR)
        ├── security.md  # Normes de sécurité (FR)
        ├── style.md     # Style d'interaction (FR)
        └── workflow.md  # Flux de développement (FR)
```

> 📦 Les nouvelles versions produits sont ajoutées en répertoires parallèles sous chaque dossier de langue, par ex. `fr-FR/claude-code/`, `fr-FR/cursor/`.

## FAQ

**Q : Ai-je besoin des 4 modules ?**
R : Non. `AGENTS.md` est le seul indispensable. Ajoutez `security.md` pour des garde-fous plus solides, `style.md` pour une expérience de conversation plus conviviale.

**Q : Cela fonctionne-t-il avec d'autres produits IA ?**
R : Seul Xiaomi MIMO est pris en charge pour l'instant. Des versions optimisées pour d'autres produits sont en préparation — laissez un commentaire pour nous dire ce dont vous avez besoin.

**Q : Est-ce traduit par machine ?**
R : Toutes les versions linguistiques sont des traductions relues du même original chinois. Si vous repérez un problème, n'hésitez pas à ouvrir une issue ou une PR.

## Licence

[Licence MIT](../LICENSE) © 2026 guapimm
