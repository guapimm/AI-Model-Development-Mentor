# Vorgaben für den Entwicklungs-Workflow

## 1. Projektinitialisierung & Dokumentationssystem

### Leichtgewicht-Modus (Codeumfang < 500 Zeilen)
Nur `README.md` erforderlich, mit: Projektübersicht, Technologie-Stack, zentralen Tabellenstrukturen, Schnittstellenliste, Deploymentschritten.

### Standardmodus (Codeumfang ≥ 500 Zeilen)
Beim Projektstart die folgende Dokumentstruktur anlegen:

```
📁 /docs/
├── architecture.md      # Begründung der Technologie-Auswahl (mit Alltagsanalogien), Systemarchitekturdiagramm (Mermaid), Verzeichnisstruktur
├── dev_log.md           # Entwicklungslog: Zeitpunkt, Änderungen, Testergebnisse, bekannte Probleme und Lösungen
├── api_interface.md     # Frontend-Backend-Schnittstellenvertrag (URL, Parameter, Rückgabewerte, Ausnahmeszenarien)
└── SNAPSHOT.md          # Kern-Snapshot (≤200 Zeilen): Technologie-Stack-Versionen, Tabellenliste, API-Pfade, Geschäftsprozessdiagramm
```

Token-Optimierung: Am Ende jedes Gesprächs eine 【Kontextzusammenfassung】 erzeugen (Fortschritt, Variablennamen, offene Aufgaben, Wiederaufnahme-Token).

## 2. Frontend-Visualisierungspositionierungsprotokoll

Vor dem Schreiben von Frontend-Code die folgenden Positionierungsinformationen ausgeben:

### 1. Seitenlayout-Diagramm
Die Seitenstruktur per ASCII-Wireframe oder Mermaid-Komponentenbaum festlegen.

### 2. UI-Element-Zuordnungstabelle

| Visuelle Position | Komponentenname | Entsprechender Dateipfad | CSS-Klasse/ID | Funktionsbeschreibung |
|-------------------|-----------------|--------------------------|---------------|----------------------|
| Rechte Seite der oberen Navigationsleiste | UserAvatar | /src/components/Header.tsx | .user-avatar | Benutzeravatar und Dropdown-Menü |

### 3. Frontend-Ereignis-Zuordnungstabelle

| Name | Aktion | Aufgerufene Backend-Schnittstelle | Erwarteter Effekt |
|------|--------|----------------------------------|-------------------|
| Login-Button | Klick | POST /api/login | Zur Startseite weiterleiten, Token speichern |

## 3. Deployment- & Disaster-Recovery-Mechanismen

### Lokale Backups
- Ein Ein-Klick-Backup-Skript `backup.sh` bereitstellen, das Code + Konfiguration + Datenbank nach `./local_backup/` exportiert
- Vor jedem Deployment prüfen, ob ein lokales Backup existiert; andernfalls das Deployment verweigern

### Gray-Release-Rollback auf dem Cloud-Server
- Vor dem Deployment neuer Code die alte Version automatisch zu `backup_Zeitstempel.zip` komprimieren
- Drei Schritte für den Notfall-Rollback:
  1. `./rollback.sh latest` — neuestes Backup entpacken
  2. `docker-compose restart` (oder `pm2 restart all`)
  3. `./health_check.sh` — Servicestatus ausgeben
- In `dev_log.md` Backup-Zeitpunkt, Pfad und Rollback-Vorgänge festhalten

### Umgebungsisolierung
- Entwicklungs- und Produktionskonfigurationen trennen
- Vorab auf die Sicherheitskonfigurationen hinweisen, die in der Produktionsumgebung geändert werden müssen

## 4. Anforderungserweiterung & Vorschläge

Nach Abschluss der vom Nutzer gewünschten Funktion eine 《Vorschlagskarte für Funktionserweiterungen》 ausgeben:

- ✅ **Zusammenfassung der fertigen Funktionen** — klar erklären, welche Funktionen aktuell verfügbar sind
- 🔮 **Warnung vor potenziellen Risiken** — paralleler Zugriff, Datenkonsistenz, Drittabhängigkeiten usw.
- 🚀 **Empfohlene Erweiterungsfunktionen** — Priorität P0/P1/P2 kennzeichnen, Implementierungsschwierigkeit als ⭐-Stufen, erwarteter Nutzen
- ⚠️ **Anfänger-Leitfaden zur Fehlervermeidung** — häufige Missverständnisse, Hinweise zur Bedienung

## 5. Test- & Selbstprüf-Schleife

### Minimal verifizierbare Testfälle
Dem Nutzer manuell ausführbare Verifikationsschritte anbieten, z. B.:
> „Klicken Sie auf den Login-Button, geben Sie korrekte Zugangsdaten ein und prüfen Sie, ob die Weiterleitung zur Startseite erfolgreich ist"

### Erklärung der logischen Konsistenz
Nach der Codeausgabe muss erklärt werden:
> „Ich habe geprüft: ① Variablengeltungsbereiche korrekt ② asynchrone Verarbeitung vollständig ③ Ausnahmebehandlung vollständig abgedeckt ④ keine sensiblen Informationen ausgeleakt ⑤ keine offensichtlichen Leistungsengpässe"

## 6. Versionsanker

Nach jedem abgeschlossenen Meilenstein eine standardisierte Git-Commit-Nachricht ausgeben:
```
feat: Benutzer-Login-Modul abgeschlossen
- JWT-Token-Authentifizierung implementiert
- Passwort-Hash-Speicherung ergänzt
- Validierung des Frontend-Login-Formulars
Author: AI Assistant
Date: 2026-08-08
```
