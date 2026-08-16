# Frontend-UI-Zuordnungstabelle + Ereignis-Zuordnungstabelle (erforderlich vor dem Schreiben des Frontends)

> Wird von der Mentor-KI vor dem Schreiben des Frontend-Codes ausgegeben, damit „Anfänger ohne Vorkenntnisse" Probleme präzise melden können.
> Nach der Fertigstellung in `docs/` ablegen und zusammen mit der API-Spezifikation (`docs/api_interface.md`) verwenden.

## 1. Seiten-Wireframe (ASCII oder Mermaid)

```
┌──────────────────────────────────────────┐
│  Obere Navigationsleiste (Logo / Menü / Avatar) │
├───────────────┬──────────────────────────┤
│               │                          │
│   Seitenleiste│     Hauptinhalt          │
│               │                          │
└───────────────┴──────────────────────────┘
```

## 2. UI-Element-Zuordnungstabelle

| Visuelle Position | Komponente | Dateipfad | CSS-Klasse/ID | Beschreibung |
|-------------------|------------|-----------|---------------|--------------|
| Obere Leiste, rechts | UserAvatar | src/components/Header.tsx | .user-avatar | Benutzer-Avatar und Dropdown-Menü (Abmelden, Profil) |
| | | | | |

## 3. Frontend-Ereignis-Zuordnungstabelle

| Name | Aktion (Klick/Wischen/Eingabe) | Aufgerufener Backend-Endpunkt | Erwartetes Ergebnis |
|------|--------------------------------|-------------------------------|---------------------|
| Anmelden-Schaltfläche | Klick | POST /api/login | Nach erfolgreicher Prüfung zur Startseite weiterleiten, bei Fehler Meldung anzeigen |
| | | | |

## 4. Verwendungshinweis (für Anfänger ohne Vorkenntnisse)

1. Um ein Seitenproblem zu melden, sag einfach „**Position** + **was passiert**", z. B.:
   > „Der Avatar oben rechts reagiert nicht auf Klicks"
2. Die Mentor-KI findet mit den beiden Tabellen oben die genaue Komponente und den Endpunkt — ohne Codebeschreibung.
