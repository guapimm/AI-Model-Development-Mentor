# Projekt-Ressourcenschätzung (erforderlich in Phase 0)

> Wird zu Projektbeginn unter Anleitung der Mentor-KI ausgefüllt und dient als Grundlage für die Auswahl des Tech-Stacks und die Deployment-Planung.
> Übertrage die Tabelle nach dem Ausfüllen nach `docs/architecture.md` und halte sie in späteren Phasen aktuell.

## 1. Grundlegende Projektinformationen

| Punkt | Inhalt |
|-------|--------|
| Projektname | |
| Geschätzte Gesamtzeilen Code | (unter 500 Zeilen aktiviert den „Leichtgewicht-Modus" mit nur einer README.md) |
| Zielgruppe / Nutzerumfang | Privat / kleines Team / öffentliches Produkt |
| Spitzenwert gleichzeitiger Nutzer | |
| Datentyp | Klartext / Bilder / Audio-Video / große Dateien |

## 2. Drei-Stufen-Ressourcenschätzung

| Dimension | Minimum (Entwicklung/Demo) | Empfohlen (kleiner Launch) | Hochverfügbar (öffentliches Produkt) |
|-----------|----------------------------|----------------------------|--------------------------------------|
| Arbeitsspeicher | | | |
| Festplatte | | | |
| CPU-Kerne | | | |
| Bandbreite | | | |
| Datenbank | SQLite / In-Memory | MySQL / PostgreSQL | Cluster + Lese-/Schreibtrennung |

## 3. Abhängigkeiten von Drittanbieter-Diensten

| Dienst | Zweck | Erforderlich? | Kostenloses Kontingent ausreichend? |
|--------|-------|---------------|-------------------------------------|
| Cloud-Server | | | |
| Objektspeicher (Dateien/Bilder) | | | |
| SMS / E-Mail | | | |
| Bezahlung | | | |
| Sonstiges | | | |

## 4. Leistungs- und Ressourcenplan

- [ ] Listen-API-Aufrufe paginieren standardmäßig; keine vollständigen Tabellen-Scans
- [ ] Datenbankdesign enthält einen Index-Plan
- [ ] Operationen mit großen Dateien/Datenmengen nutzen Streaming
- [ ] Große Speicheroperationen besitzen einen expliziten Freigabemechanismus
- [ ] Externe Anfragen definieren Zeitlimits und Wiederholungsstrategien

## 5. Monatliche Kostenschätzung

| Punkt | Minimum | Empfohlen |
|-------|---------|-----------|
| Server | | |
| Speicher | | |
| Drittanbieter-Dienste | | |
| **Gesamt** | | |
