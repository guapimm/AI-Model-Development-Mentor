# Normes du flux de développement

## 1. Initialisation du projet et système documentaire

### Mode léger (moins de 500 lignes de code)
Seul `README.md` est requis, incluant : présentation du projet, pile technique, structure des tables principales, liste des interfaces, étapes de déploiement, tableau récapitulatif de l'estimation des ressources.

### Mode standard (500 lignes de code ou plus)
Créer la structure documentaire suivante au lancement du projet :

```
📁 /docs/
├── architecture.md      # Justification du choix de la pile technique (analogies de la vie courante), schéma d'architecture système (Mermaid), structure des répertoires
├── resource_estimate.md # Tableau d'estimation des ressources du projet (trois niveaux : mémoire / disque / configuration, seuils de montée en charge)
├── dev_log.md           # Journal de développement : horodatage, changements, résultats des tests, problèmes connus et solutions
├── api_interface.md     # Contrat d'interface front-end/back-end (URL, paramètres, valeurs de retour, scénarios d'exception)
└── SNAPSHOT.md          # Instantané principal (≤ 200 lignes) : versions de la pile technique, liste des tables, chemins d'API, diagrammes de flux métier
```

### Livrable obligatoire de la phase 0 : tableau d'estimation des ressources du projet
Obligatoire une fois le besoin confirmé, avant l'écriture du code, au format standard :

| Niveau de configuration | Pic de mémoire d'exécution | Occupation disque initiale | Croissance disque estimée par an | Exigence CPU minimale | Scénario adapté |
|-------------------------|----------------------------|----------------------------|----------------------------------|-----------------------|-----------------|
| Niveau minimal | XX Mo | XX Mo | XX Mo | 1 cœur | Développement solo, faible volume d'accès |
| Niveau recommandé | XX Mo | XX Mo | XX Mo | 2 cœurs | Usage quotidien, moins de 100 accès |
| Niveau haute performance | XX Mo | XX Go | XX Go | 4 cœurs | Accès concurrents, environnement de production |
- Condition de déclenchement de la montée en charge : indiquer à partir de quels seuils de nombre d'utilisateurs / volume de données une mise à niveau de la configuration est nécessaire
- Estimation de la consommation de tokens : estimation de la plage de consommation de tokens sur l'ensemble du flux du projet

Optimisation des tokens : générer un【Résumé du contexte】à la fin de chaque conversation (progression, noms de variables, tâches en attente, code de reprise), chaque résumé étant limité à 100 caractères.

## 2. Normes obligatoires de conception de la base de données

- Émettre en parallèle un plan de conception d'index lors de la conception de la structure des tables ; les champs de requête essentiels doivent être indexés
- Estimer le volume de données de chaque table ; au-delà de 100 000 lignes, fournir à l'avance une solution de partitionnement / optimisation
- Définir la longueur et le type des champs selon les besoins, afin d'éviter un gaspillage excessif de l'espace de stockage
- Configurer obligatoirement une limite maximale pour le pool de connexions de la base de données, afin d'éviter que l'épuisement des connexions ne fasse planter le service

## 3. Protocole de positionnement visuel du front-end

Avant d'écrire le code front-end, émettre d'abord les informations de positionnement suivantes :

### 1. Schéma de mise en page de la page
Utiliser un wireframe ASCII ou un arbre de composants Mermaid pour clarifier la structure de la page.

### 2. Table de correspondance des éléments d'interface

| Position visuelle | Nom du composant | Chemin de fichier correspondant | Classe CSS/ID | Description fonctionnelle |
|-------------------|------------------|--------------------------------|---------------|---------------------------|
| À droite de la barre de navigation supérieure | UserAvatar | /src/components/Header.tsx | .user-avatar | Avatar utilisateur et menu déroulant |

### 3. Table de correspondance des événements front-end

| Nom | Action | Interface back-end appelée | Effet attendu |
|-----|--------|---------------------------|---------------|
| Bouton de connexion | Clic | POST /api/login | Redirection vers la page d'accueil, stockage du Token |

## 4. Mécanismes de déploiement et de reprise après sinistre

### Sauvegarde locale
- Fournir un script de sauvegarde en un clic `backup.sh`, qui exporte le code + la configuration + la base de données vers `./local_backup/`
- Avant chaque déploiement, vérifier qu'une sauvegarde locale existe, sinon refuser le déploiement

### Retour arrière progressif sur serveur cloud
- Avant de déployer un nouveau code, compresser automatiquement l'ancienne version en `backup_horodatage.zip`
- Opération de retour arrière d'urgence en trois étapes :
  1. `./rollback.sh latest` — décompresser la sauvegarde la plus récente
  2. `docker-compose restart` (ou `pm2 restart all`)
  3. `./health_check.sh` — afficher l'état du service
- Enregistrer dans `dev_log.md` l'horodatage des sauvegardes, leurs chemins et les opérations de retour arrière

### Isolation des environnements
- Distinguer les configurations de développement et de production
- Signaler à l'avance les éléments de configuration de sécurité à modifier en environnement de production

## 5. Élargissement du besoin et suggestions

Après avoir réalisé la fonctionnalité demandée par l'utilisateur, émettre une « Fiche de suggestions d'amélioration des fonctionnalités » :

- ✅ **Résumé des fonctionnalités réalisées** — explique clairement les fonctionnalités actuellement disponibles
- 🔮 **Alerte sur les risques potentiels** — accès concurrents, cohérence des données, dépendances tierces, etc.
- 🚀 **Fonctionnalités d'extension recommandées** — avec priorité P0/P1/P2, difficulté de mise en œuvre ⭐, effet attendu
- ⚡ **Suggestions d'optimisation des performances** — indiquer la priorité, par exemple P0 ajouter un index sur les champs essentiels, P1 ajouter un cache pour les données à fort accès
- ⚠️ **Guide anti-pièges pour débutants** — erreurs fréquentes, précautions d'utilisation

## 6. Boucle fermée de tests et d'auto-vérification

### Cas de test minimal vérifiable
Fournir des étapes de vérification que l'utilisateur peut exécuter manuellement, par exemple :
> « Cliquer sur le bouton de connexion, saisir un compte et un mot de passe corrects, vérifier que la redirection vers la page d'accueil réussit »

### Déclaration de cohérence logique
Après l'émission du code, il faut obligatoirement déclarer :
> « J'ai vérifié : ① les portées de variables sont correctes ② le traitement asynchrone est complet ③ la capture des exceptions est couvrante ④ aucune fuite d'informations sensibles ⑤ aucun goulot d'étranglement de performance évident ⑥ l'occupation mémoire est maîtrisée »

## 7. Ancrage de version

À chaque jalon accompli, émettre un message de commit Git conforme :
```
feat: module de connexion utilisateur terminé
implémentation de l'authentification par jeton JWT (JWT Token)
ajout du stockage par hachage des mots de passe
validation du formulaire de connexion côté front-end
Author: AI Assistant
Date: 2026-08-08
```
