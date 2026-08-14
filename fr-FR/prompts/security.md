# Manuel détaillé des normes de sécurité

## 1. Gestion des clés et de la configuration

- Interdiction de coder en dur une clé, un mot de passe ou un API Token dans le code
- Utiliser systématiquement des variables d'environnement ; le code ne référence que les noms de variables
- Tous les éléments de configuration sont extraits dans le fichier `.env.example` (sans valeur réelle, uniquement les noms de variables)
- Le fichier `.env` de production est ajouté au `.gitignore`

## 2. Validation des entrées utilisateur

- Toute entrée utilisateur doit passer une validation de type (par ex. refuser les chaînes pour les champs numériques)
- Définir des limites de longueur raisonnables (par ex. nom d'utilisateur : 2 à 50 caractères)
- Refuser l'injection de caractères spéciaux (mots-clés SQL, balises HTML, etc.)
- Limiter les types et la taille des fichiers téléversés, valider le type MIME

## 3. Sécurité de la base de données

- Utiliser obligatoirement des requêtes paramétrées ou des instructions ORM précompilées
- Interdiction de concaténer des chaînes pour construire du SQL
- Les champs sensibles (mots de passe) doivent être stockés sous forme de hachage (bcrypt/argon2)
- Le mot de passe de la chaîne de connexion à la base de données passe par une variable d'environnement
- Configurer obligatoirement une limite maximale pour le pool de connexions de la base de données, afin d'éviter que l'épuisement des connexions ne fasse planter le service

## 4. Protection XSS côté front-end

- Tout contenu rendu dynamiquement doit passer un échappement HTML
- Utiliser les mécanismes d'échappement intégrés du framework (par ex. `{}` de React, `{{}}` de Vue)
- Interdiction de rendre directement une entrée utilisateur avec `innerHTML` ou `v-html`
- Les cookies doivent porter les drapeaux `HttpOnly` et `Secure`

## 5. Sécurité du système de fichiers

- Toute opération sur les chemins de fichiers doit être validée pour empêcher les traversées de répertoire (`../`)
- Restreindre l'accès aux répertoires avec une liste blanche
- Renommer les fichiers téléversés avec un UUID aléatoire, sans conserver le nom de fichier d'origine
- Définir une limite maximale stricte pour la taille d'un fichier ; imposer le téléversement par fragments (chunked) pour les fichiers très volumineux

## 6. Sécurité des requêtes externes

- Toutes les requêtes HTTP doivent définir un délai d'attente (recommandé : 5 à 10 secondes)
- Mettre en place une stratégie de nouvelle tentative (3 maximum, avec backoff exponentiel)
- Vérifier les certificats SSL ; interdiction de sauter la validation des certificats

## 7. Gestion des exceptions

- Toutes les exceptions doivent être capturées avec try-catch
- Ne pas renvoyer la trace de pile brute au client en environnement de production
- Consigner les journaux d'erreurs (horodatage, ID de requête, type d'erreur)
- Journaliser les opérations sensibles (échecs de connexion, permissions insuffisantes) dans les journaux d'audit

## 8. Sécurité des performances et des ressources

- Toutes les interfaces de listes activent la pagination par défaut, avec un nombre maximal de lignes par page (100 par défaut) ; interdiction des requêtes en masse
- Configurer la limitation de débit des interfaces selon la concurrence estimée (au niveau IP et au niveau utilisateur), afin de prévenir les attaques par épuisement des ressources
- Traiter les gros fichiers / gros volumes de données par lecture-écriture en flux (streaming), afin d'éviter un chargement complet en mémoire qui provoquerait un débordement
- Les champs de requête essentiels doivent être indexés ; interdiction des balayages complets de table sans index
- Nettoyer périodiquement les journaux expirés et les fichiers temporaires, afin d'éviter une croissance illimitée de l'occupation disque
