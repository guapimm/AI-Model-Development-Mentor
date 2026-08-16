🌍 Autres langues → [English](../README.md)

# AI Model Mentor (Français)

> **Transformez votre assistant de codage IA en un architecte full-stack prudent fort de 10 ans d'expérience — des prompts purs, zéro dépendance.**

---

## Qu'est-ce que c'est ?

Un **framework 100 % basé sur des prompts** qui transforme votre assistant de codage IA en **architecte full-stack & mentor de développement fort de 10 ans d'expérience**, pensé pour les débutants en programmation sans aucune base.

Il impose à l'IA de suivre des « règles d'airain » — faire de la *Sécurité d'abord, de la Logique transparente, de la Documentation d'abord, de l'Efficacité des tokens, de l'Avancement par étapes et de la Maîtrise des ressources* son comportement par défaut. Résultat : une IA qui ne se contente pas *d'écrire du code*, mais écrit un code **sûr, maintenable et documenté**.

> ⚠️ Compatible multi-outils : opencode, Claude Code, Codex, Cursor, etc. Consultez [COMPATIBILITY.md](./COMPATIBILITY.md) pour le guide de chargement de chaque outil.

## Modules principaux (compatibles multi-outils)

| Module | Fichier | Rôle |
|--------|---------|------|
| 🧑‍🏫 Rôle de mentor | [AGENTS.md](./prompts/AGENTS.md) | Persona d'architecte-mentor full-stack + 6 règles d'airain + liste de contrôle de sécurité et de performances ★ indispensable |
| 🛡️ Normes de sécurité | [security.md](./prompts/security.md) | 8 domaines de sécurité : gestion des clés secrètes / validation des entrées / base de données / XSS / système de fichiers / requêtes externes / gestion des exceptions / performances et ressources |
| 🎨 Style d'interaction | [style.md](./prompts/style.md) | Analogies de la vie courante, étiquettes de phase, confirmer avant d'agir, complexité progressive |
| 📋 Flux de développement | [workflow.md](./prompts/workflow.md) | Système documentaire / estimation des ressources / conception de la base de données / protocole de positionnement front-end / déploiement et reprise après sinistre / boucle de test et d'autocontrôle / ancrage de version |

## 📦 Plus de documents

- [COMPATIBILITY.md](./COMPATIBILITY.md) — guide de chargement pour chaque outil d'IA (opencode / Claude Code / Codex / Cursor, etc.)
- [Prompt-Complet-du-Mentor.md](./prompts/Prompt-Complet-du-Mentor.md) — prompt consolidé tout-en-un (tous les modules fusionnés)

### Les 6 règles d'airain

1. **Le code est la documentation** — tout le code porte des commentaires expliquant le « pourquoi »
2. **La sécurité d'abord** — pas de secrets codés en dur, validation stricte des entrées, requêtes paramétrées, protection XSS
3. **Zéro modification destructrice** — analyser les dépendances d'abord, marquer les modifications [Obligatoire] / [Optionnelle]
4. **Exécution par étapes** — jamais plus de 300 lignes par réponse, attendre la confirmation à chaque étape
5. **Isolation modulaire** — 500 lignes maximum par fichier, prévoir des interfaces d'extension
6. **La performance et les ressources d'abord** — émettre un plan d'indexation en parallèle de la conception de la base de données ; pagination par défaut pour les interfaces de requête ; estimation des ressources sur trois niveaux (mémoire / disque / puissance de calcul) au début du projet ; mécanisme de libération pour les opérations gourmandes en mémoire

## ⬇️ Installation et utilisation de mentor CLI

**Option A : binaire Go (recommandé, zéro dépendance, multiplateforme)**

Téléchargez l'exécutable `mentor` correspondant à votre plateforme depuis GitHub Releases (v0.1.0, prend en charge Windows / Linux / macOS), puis placez-le dans votre PATH :

```bash
mentor install                        # Assistant interactif : choisir la langue → choisir le module (agent par défaut) → détection automatique de l'outil
mentor install --lang zh-CN --modules agent,security --cli claude-code --dir ./proj
mentor add workflow                   # Ajouter un module
mentor list                           # Afficher les modules installés
mentor detect                         # Détecter l'outil d'IA utilisé par le projet
mentor pack                           # Générer un répertoire de skill compatible
```

`mentor` écrit automatiquement le bon nom de fichier et le bon emplacement selon l'outil : opencode/Codex → `AGENTS.md`, Claude Code → `CLAUDE.md`, Cursor → `.cursor/rules/`.

**Option B : copie manuelle**

Suivez les instructions de [COMPATIBILITY.md](./COMPATIBILITY.md) pour copier les fichiers de `prompts/` vers les emplacements correspondants de votre projet.

> Commandes prises en charge : `install` / `add` / `remove` / `list` / `detect` / `pack` ; modules : agent (par défaut) / security / style / workflow / complete ; outils : opencode / claude-code / codex / cursor / other.

## 📖 Guide d'utilisation (opencode)

### Aperçu des commandes

| Scénario | Opération |
|----------|-----------|
| Développement quotidien | Entrer dans le projet → opencode charge AGENTS.md automatiquement → dialogue normal |
| Projet à long terme | Conserver les règles dans AGENTS.md (mettre à jour avec `/init`) |
| Déconnexion inattendue | Reprendre avec `opencode --continue`, les règles sont toujours là |
| Ouvrir volontairement une nouvelle session | Il suffit de lancer `opencode` — AGENTS.md est chargé automatiquement |

### Structure des fichiers du projet

```
📁 my-project/
├── 📄 AGENTS.md          ← Prompt principal
├── 📄 security.md        ← Normes de sécurité
├── 📄 workflow.md        ← Normes de flux de travail
├── 📄 style.md           ← Style d'interaction
└── 📁 src/
```

---

### Démonstrations par scénario

#### Scénario 1 : Écrire du code au quotidien (charger uniquement AGENTS.md)

> Vous : « Aide-moi à écrire une API qui récupère la liste des utilisateurs »

À charger : AGENTS.md (déjà chargé automatiquement, aucune action requise)

L'IA fera automatiquement :

- du code avec commentaires en français
- cochera la liste de contrôle de sécurité avant de sortir le code
- une exécution par étapes (≤300 lignes)
- maximum 500 lignes par fichier

#### Scénario 2 : Écrire les interfaces de connexion/inscription (charger AGENTS.md + security.md)

> Vous : « Aide-moi à implémenter la fonctionnalité de connexion utilisateur, en suivant les exigences de security.md »

À charger :

```bash
@security.md
```

En plus, l'IA :

- stocke les mots de passe hachés avec bcrypt
- définit une durée d'expiration pour le JWT Token
- protège contre la force brute (limitation des échecs de connexion)
- protège contre les injections SQL (requêtes paramétrées)

#### Scénario 3 : Démarrer un projet de zéro (charger AGENTS.md + workflow.md)

> Vous : « Je veux créer un système de blog, aide-moi à construire le squelette du projet en te référant à workflow.md »

À charger :

```bash
@workflow.md
```

En plus, l'IA :

- crée docs/architecture.md (choix de la pile technique + schéma d'architecture)
- crée docs/dev_log.md (modèle de journal de développement)
- crée docs/api_interface.md (modèle de contrat d'interface)
- crée docs/SNAPSHOT.md (instantané du projet)
- génère les scripts backup.sh et rollback.sh

#### Scénario 4 : Les explications de l'IA sont trop obscures (charger style.md)

> Vous : « À la manière de style.md, explique-moi ce qu'est JWT avec une analogie de la vie courante »

À charger :

```bash
@style.md
```

En plus, l'IA :

- explique JWT avec « la carte de membre d'un restaurant »
- ajoute l'étiquette de phase [📋 Analyse des besoins]
- donne d'abord la conclusion, ensuite le détail
- propose 2 à 3 solutions alternatives

#### Scénario 5 : Mise en production (charger AGENTS.md + workflow.md)

> Vous : « Selon les normes de déploiement de workflow.md, aide-moi à écrire la configuration de déploiement Docker »

À charger :

```bash
@workflow.md
```

En plus, l'IA :

- distingue les configurations de développement et de production
- génère docker-compose.yml
- génère health_check.sh
- rappelle les étapes de sauvegarde et de retour arrière

### ⚠️ Quand ne pas charger ?

| Cas où il ne faut pas charger | Raison |
|-------------------------------|--------|
| Poser une question purement technique (par ex. « Comment utiliser React useEffect ? ») | AGENTS.md suffit, ajouter workflow serait plutôt une gêne |
| Modifier un style CSS | Les normes de sécurité et le flux de déploiement ne sont pas nécessaires |
| Demander à l'IA de traduire un texte | Aucun module n'est nécessaire |
| Refactoriser légèrement du code existant | La liste de contrôle de sécurité d'AGENTS.md couvre déjà ce besoin |

### 💡 Résumé en une phrase

> AGENTS.md est la peau par défaut, les trois autres sont des plugins d'effets spéciaux — ne les activez que lorsque c'est nécessaire, sinon laissez-les désactivés pour économiser des tokens et rester léger.

## Démarrage rapide (3 étapes)

```bash
# 1. Copiez le rôle de mentor dans votre projet (renommez-le)
cp prompts/AGENTS.md AGENTS.md

# 2. (Recommandé) Ajoutez aussi les normes sécurité / style / workflow
cp prompts/security.md security.md
cp prompts/style.md style.md
cp prompts/workflow.md workflow.md
```

3. Lancez opencode et dites :

> « Je suis un vrai débutant. Voici ma Spécification des besoins du projet : nom du projet ____, objectifs principaux ____, rôles des utilisateurs ____, processus d'utilisation principaux ____, données à stocker ____. Commencez par la Phase 0 : préparation de l'environnement et choix de la pile technique, et guidez-moi étape par étape. »

L'IA avancera selon « Conception → Logique principale → Interface → Tests », en attendant votre confirmation à chaque étape.

## Structure des fichiers

```
AI_Model_Development_Mentor/
├── README.md            # Page d'accueil + sélecteur de langue
├── LICENSE              # Licence Apache-2.0
├── zh-CN/               # Chinois
├── en-US/               # Anglais
└── fr-FR/               # Français (cette page)
    ├── README.md        # Page d'accueil + guide d'utilisation
    ├── COMPATIBILITY.md # Guide de chargement par outil
    └── prompts/         # Contenu indépendant des outils
        ├── AGENTS.md    # Rôle de mentor (FR) ★ indispensable
        ├── security.md  # Normes de sécurité (FR)
        ├── style.md     # Style d'interaction (FR)
        ├── workflow.md  # Flux de développement (FR)
        └── Prompt-Complet-du-Mentor.md  # Prompt consolidé (FR)
```

> 📦 Les nouveaux outils s'ajoutent comme des lignes dans [COMPATIBILITY.md](./COMPATIBILITY.md) ; plus besoin de créer des répertoires par produit.

## FAQ

**Q : Ai-je besoin des 4 modules ?**
R : Non. `AGENTS.md` est le seul indispensable. Ajoutez `security.md` pour des garde-fous plus solides, `style.md` pour une expérience de conversation plus conviviale.

**Q : Cela fonctionne-t-il avec d'autres produits IA ?**
R : Oui. Les modules sont compatibles multi-outils (opencode, Claude Code, Codex, Cursor, etc.) — consultez [COMPATIBILITY.md](./COMPATIBILITY.md) pour le guide de chargement de chaque outil.

**Q : Est-ce traduit par machine ?**
R : Toutes les versions linguistiques sont des traductions relues du même original chinois. Si vous repérez un problème, n'hésitez pas à ouvrir une issue ou une PR.

## Licence

[Licence Apache-2.0](../LICENSE) © 2026 guapimm
