# Tableau d'estimation des ressources du projet (obligatoire en Phase 0)

> Rempli au démarrage du projet, guidé par l'IA mentor, comme base pour le choix de la pile technologique et la planification du déploiement.
> Après remplissage, archivez ce tableau dans `docs/architecture.md` et tenez-le à jour dans les phases suivantes.

## 1. Informations de base du projet

| Élément | Valeur |
|---------|--------|
| Nom du projet | |
| Nombre de lignes de code estimé | (moins de 500 lignes active le « mode léger », avec un seul README.md) |
| Échelle d'utilisateurs cible | Personnel / petite équipe / produit public |
| Pic d'utilisateurs simultanés | |
| Type de données | Texte brut / images / audio-vidéo / fichiers volumineux |

## 2. Estimation des ressources sur trois niveaux

| Dimension | Minimum (dev/démo) | Recommandé (petit lancement) | Haute disponibilité (produit public) |
|-----------|--------------------|------------------------------|--------------------------------------|
| Mémoire | | | |
| Disque | | | |
| Cœurs CPU | | | |
| Bande passante | | | |
| Base de données | SQLite / en mémoire | MySQL / PostgreSQL | Cluster + séparation lecture/écriture |

## 3. Dépendances aux services tiers

| Service | Usage | Requis ? | Offre gratuite suffisante ? |
|---------|-------|----------|------------------------------|
| Serveur cloud | | | |
| Stockage d'objets (fichiers/images) | | | |
| SMS / e-mail | | | |
| Paiement | | | |
| Autre | | | |

## 4. Plan performance et ressources

- [ ] Les endpoints de liste paginent par défaut ; pas de scan complet des tables
- [ ] La conception de la base de données inclut un plan d'index
- [ ] Les opérations sur fichiers/données volumineux utilisent le streaming
- [ ] Les opérations mémoire lourdes disposent d'un mécanisme de libération explicite
- [ ] Les requêtes externes définissent des délais d'expiration et des politiques de nouvelle tentative

## 5. Estimation du coût mensuel

| Élément | Minimum | Recommandé |
|---------|---------|------------|
| Serveur | | |
| Stockage | | |
| Services tiers | | |
| **Total** | | |
