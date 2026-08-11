# Sicherheitshandbuch im Detail

## 1. Schlüssel- & Konfigurationsverwaltung

- Es ist verboten, Schlüssel, Passwörter oder API-Tokens im Code hartzucodieren
- Einheitlich Umgebungsvariablen verwenden; im Code nur auf Variablennamen verweisen
- Alle Konfigurationsoptionen in `.env.example` auslagern (ohne echte Werte, nur Variablennamen)
- Die `.env`-Datei der Produktionsumgebung in `.gitignore` aufnehmen

## 2. Validierung von Benutzereingaben

- Alle Benutzereingaben müssen einer Typprüfung unterzogen werden (z. B. lehnen numerische Felder Strings ab)
- Angemessene Längenbegrenzungen setzen (z. B. Benutzername 2–50 Zeichen)
- Injektion von Sonderzeichen ablehnen (SQL-Schlüsselwörter, HTML-Tags usw.)
- Bei Datei-Uploads Typ und Größe begrenzen, MIME-Typ validieren

## 3. Datenbanksicherheit

- Zwingend parametrisierte Abfragen oder vorkompilierte ORM-Anweisungen verwenden
- SQL per String-Konkatenation ist verboten
- Sensible Felder (Passwörter) müssen gehasht gespeichert werden (bcrypt/argon2)
- Passwörter in Datenbank-Verbindungsstrings über Umgebungsvariablen beziehen

## 4. XSS-Schutz im Frontend

- Sämtlicher dynamisch gerenderter Inhalt muss HTML-escaped werden
- Integrierte Escaping-Mechanismen der Frameworks nutzen (z. B. `{}` in React, `{{}}` in Vue)
- Es ist verboten, Benutzereingaben direkt per `innerHTML` oder `v-html` zu rendern
- Für Cookies die Flags `HttpOnly` und `Secure` setzen

## 5. Dateisystem-Sicherheit

- Alle Dateipfad-Operationen validieren, um Directory-Traversal (`../`) zu verhindern
- Zugängliche Verzeichnisse über eine Whitelist einschränken
- Hochgeladene Dateien in zufällige UUIDs umbenennen, Originaldateinamen nicht behalten

## 6. Sicherheit externer Anfragen

- Alle HTTP-Anfragen mit einem Timeout versehen (empfohlen: 5–10 Sekunden)
- Wiederholungsstrategie implementieren (max. 3 Versuche mit exponentiellem Backoff)
- SSL-Zertifikate verifizieren; das Überspringen der Zertifikatsprüfung ist verboten

## 7. Fehlerbehandlung

- Alle Ausnahmen müssen mit try-catch abgefangen werden
- In Produktionsumgebungen keine Roh-Stack-Traces an Clients zurückgeben
- Fehler-Logs erfassen (Zeitstempel, Request-ID, Fehlertyp)
- Sensible Vorgänge (fehlgeschlagene Logins, fehlende Berechtigungen) auditieren

## 8. Leistung & Sicherheit

- Leistungsengpässe von Schnittstellen bedenken, bei Bedarf Cache (Redis) ergänzen
- Langsame Abfragen optimieren, Datenbankindizes hinzufügen
- Große Datei-Uploads in Chunks oder per Streaming verarbeiten
- Ressourcen-Erschöpfungsangriffe verhindern (Rate-Limiting für Anfragen)
