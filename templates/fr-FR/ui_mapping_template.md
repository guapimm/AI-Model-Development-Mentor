# Table de correspondance des éléments UI + table de correspondance des événements (obligatoire avant l'écriture du frontend)

> Émise par l'IA mentor avant l'écriture du code frontend, pour permettre aux utilisateurs sans aucune base de signaler précisément les problèmes.
> Archivez-la dans `docs/` et utilisez-la avec le contrat d'API (`docs/api_interface.md`).

## 1. Maquette de la page (ASCII ou Mermaid)

```
┌──────────────────────────────────────────┐
│  Barre de navigation supérieure (logo / menu / avatar) │
├───────────────┬──────────────────────────┤
│               │                          │
│   Barre       │     Contenu principal    │
│   latérale    │                          │
└───────────────┴──────────────────────────┘
```

## 2. Table de correspondance des éléments UI

| Emplacement visuel | Composant | Chemin du fichier | Classe/ID CSS | Description |
|--------------------|-----------|-------------------|---------------|-------------|
| Barre supérieure, à droite | UserAvatar | src/components/Header.tsx | .user-avatar | Avatar utilisateur et menu déroulant (déconnexion, profil) |
| | | | | |

## 3. Table de correspondance des événements du frontend

| Nom | Action (clic/glissement/saisie) | Endpoint backend appelé | Résultat attendu |
|-----|---------------------------------|-------------------------|------------------|
| Bouton de connexion | Clic | POST /api/login | Redirection vers l'accueil après validation, erreur en cas d'échec |
| | | | |

## 4. Guide d'utilisation (pour les utilisateurs sans aucune base)

1. Pour signaler un problème de page, dites simplement « **emplacement** + **ce qui se passe** », par exemple :
   > « L'avatar en haut à droite ne réagit pas au clic »
2. L'IA mentor localisera le composant et l'endpoint exacts grâce aux deux tableaux ci-dessus, sans avoir à décrire du code.
