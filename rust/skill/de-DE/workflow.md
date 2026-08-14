# Vorgaben für den Entwicklungs-Workflow

## 1. Projektinitialisierung & Dokumentationssystem

### Leichtgewicht-Modus (Codeumfang < 500 Zeilen)
Nur `README.md` erforderlich, mit: Projektübersicht, Technologie-Stack, zentralen Tabellenstrukturen, Schnittstellenliste, Deploymentschritten, Übersichtstabelle zur Ressourcenschätzung.

### Standardmodus (Codeumfang ≥ 500 Zeilen)
Beim Projektstart die folgende Dokumentstruktur anlegen:

```
📁 /docs/
├── architecture.md      # Begründung der Technologie-Auswahl (mit Alltagsanalogien), Systemarchitekturdiagramm (Mermaid), Verzeichnisstruktur
├── resource_estimate.md # Projekt-Ressourcenschätzungstabelle (drei Stufen: Arbeitsspeicher / Speicherplatz / Konfiguration, Skalierungsschwellen)
├── dev_log.md           # Entwicklungslog: Zeitpunkt, Änderungen, Testergebnisse, bekannte Probleme und Lösungen
├── api_interface.md     # Frontend-Backend-Schnittstellenvertrag (URL, Parameter, Rückgabewerte, Ausnahmeszenarien)
└── SNAPSHOT.md          # Kern-Snapshot (≤200 Zeilen): Technologie-Stack-Versionen, Tabellenliste, API-Pfade, Geschäftsprozessdiagramm
```

### Pflichtausgabe in Phase 0: Projekt-Ressourcenschätzungstabelle
Nach Bestätigung der Anforderungen und vor dem Codieren verbindlich ausgeben, im Standardformat:

| Konfigurationsstufe | Speicherspitzenwert | Anfängliche Speicherbelegung | Geschätztes jährliches Speicherwachstum | Mindest-CPU | Anwendungsszenario |
|---------------------|---------------------|------------------------------|----------------------------------------|-------------|---------------------|
| Mindeststufe | XX MB | XX MB | XX MB | 1 Kern | Einzelentwicklung, geringe Zugriffszahl |
| Empfohlene Stufe | XX MB | XX MB | XX MB | 2 Kerne | Tägliche Nutzung, Zugriff von bis zu 100 Personen |
| Hohe Stufe | XX MB | XX GB | XX GB | 4 Kerne | Paralleler Zugriff, Produktionsumgebung |

- Skalierungsauslöser: angeben, ab welcher Nutzer-/Datenmengen-Schwelle eine Konfigurationserhöhung erforderlich ist
- Token-Verbrauchsschätzung: voraussichtliche Token-Verbrauchsspanne über den gesamten Projektablauf

Token-Optimierung: Am Ende jedes Gesprächs eine 【Kontextzusammenfassung】 erzeugen (Fortschritt, Variablennamen, offene Aufgaben, Wiederaufnahme-Token), jeweils ≤100 Zeichen.

## 2. Verbindliche Vorgaben für das Datenbankdesign

- Das Tabellendesign liefert synchron einen Index-Designplan; zentrale Abfragefelder müssen einen Index erhalten
- Die voraussichtliche Datenmenge pro Tabelle schätzen; bei über 100.000 Zeilen rechtzeitig eine Tabellenaufteilung/Optimierungslösung vorschlagen
- Feldlängen und -typen bedarfsgerecht festlegen, um unnötige Speicherbelegung zu vermeiden
- Zwingend eine Obergrenze für den Datenbank-Verbindungspool konfigurieren, um einen Dienstausfall durch erschöpfte Verbindungen zu vermeiden

## 3. Frontend-Visualisierungspositionierungsprotokoll

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

## 4. Deployment- & Disaster-Recovery-Mechanismen

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

## 5. Anforderungserweiterung & Vorschläge

Nach Abschluss der vom Nutzer gewünschten Funktion eine 《Vorschlagskarte für Funktionserweiterungen》 ausgeben:

- ✅ **Zusammenfassung der fertigen Funktionen** — klar erklären, welche Funktionen aktuell verfügbar sind
- 🔮 **Warnung vor potenziellen Risiken** — paralleler Zugriff, Datenkonsistenz, Drittabhängigkeiten usw.
- 🚀 **Empfohlene Erweiterungsfunktionen** — Priorität P0/P1/P2 kennzeichnen, Implementierungsschwierigkeit als ⭐-Stufen, erwarteter Nutzen
- ⚡ **Leistungsoptimierungsvorschläge** — Priorität kennzeichnen, z. B. P0: zentrale Felder indizieren, P1: Cache für Hot Data ergänzen
- ⚠️ **Anfänger-Leitfaden zur Fehlervermeidung** — häufige Missverständnisse, Hinweise zur Bedienung

## 6. Test- & Selbstprüf-Schleife

### Minimal verifizierbare Testfälle
Dem Nutzer manuell ausführbare Verifikationsschritte anbieten, z. B.:
> „Klicken Sie auf den Login-Button, geben Sie korrekte Zugangsdaten ein und prüfen Sie, ob die Weiterleitung zur Startseite erfolgreich ist"

### Erklärung der logischen Konsistenz
Nach der Codeausgabe muss erklärt werden:
> „Ich habe geprüft: ① Variablengeltungsbereiche korrekt ② asynchrone Verarbeitung vollständig ③ Ausnahmebehandlung vollständig abgedeckt ④ keine sensiblen Informationen ausgeleakt ⑤ keine offensichtlichen Leistungsengpässe ⑥ Speichernutzung unter Kontrolle"

## 7. Versionsanker

Nach jedem abgeschlossenen Meilenstein eine standardisierte Git-Commit-Nachricht ausgeben:
```
feat: Benutzer-Login-Modul abgeschlossen
JWT-Token-Authentifizierung implementiert
Passwort-Hash-Speicherung ergänzt
Validierung des Frontend-Login-Formulars
Author: AI Assistant
Date: 2026-08-08
```
