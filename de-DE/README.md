🌍 Andere Sprachen → [English](../README.md)

# AI Model Mentor (Deutsch)

> **Verwandle deinen KI-Codierungsassistenten in einen vorsichtigen Full-Stack-Mentor mit 10 Jahren Erfahrung — reine Prompts, null Abhängigkeiten.**

---

## Was ist das?

Ein **reines Prompt-Framework**, das deinen KI-Codierungsassistenten in einen **Full-Stack-Architekten und Entwicklungsmentor mit 10 Jahren Erfahrung** verwandelt — gebaut für Programmieranfänger:innen mit null Vorkenntnissen.

Es zwingt die KI, eine Reihe von „eisernen Regeln" zu befolgen — und macht *Sicherheit zuerst, transparente Logik, Dokumentation zuerst, Token-Effizienz und schrittweise Umsetzung* zu ihrem Standardverhalten. Das Ergebnis: eine KI, die nicht nur *Code schreibt*, sondern **sicheren, wartbaren, dokumentierten** Code schreibt.

> ⚠️ Aktuell unterstützt: **Xiaomi MIMO CLI**. Optimierte Versionen für andere Produkte (Claude Code, Cursor usw.) sind geplant — hinterlasse einen Kommentar, wenn du eine brauchst.

## Kernmodule (Xiaomi-MIMO-Build)

| Modul | Datei | Zweck |
|-------|-------|-------|
| 🧑‍🏫 Mentorenrolle | [AGENTS.md](./xiaomi-mimo/AGENTS.md) | Architektur-Mentoren-Persona + 6 eiserne Regeln + Sicherheits-Checkliste ★ Kernmodul, Pflicht |
| 🛡️ Sicherheitsvorgaben | [security.md](./xiaomi-mimo/security.md) | 8 Sicherheitsbereiche: Geheimnisse / Eingabevalidierung / Datenbank / XSS / Dateisystem / externe Anfragen / Fehlerbehandlung / Leistung |
| 🎨 Interaktionsstil | [style.md](./xiaomi-mimo/style.md) | Alltagsanalogien, Phasen-Tags, erst bestätigen dann handeln, progressive Komplexität |
| 📋 Entwicklungs-Workflow | [workflow.md](./xiaomi-mimo/workflow.md) | Dokumentationssystem / Frontend-Zuordnungsprotokoll / Deployment & Rollback / Testschleife / Versionsanker |

### Die 6 eisernen Regeln

1. **Code als Dokumentation** — sämtlicher Code trägt Kommentare, die das „Warum" erklären
2. **Sicherheit zuerst** — keine hartkodierten Geheimnisse, strenge Eingabevalidierung, parametrisierte Abfragen, XSS-Schutz
3. **Null destruktive Änderungen** — zuerst Abhängigkeiten analysieren, Änderungen als 【Pflichtänderung】/【Optionale Optimierung】 kennzeichnen
4. **Schrittweise Umsetzung** — nie mehr als 300 Zeilen pro Ausgabe, bei jedem Schritt auf Bestätigung warten
5. **Modulare Isolation** — maximal 500 Zeilen pro Datei, Erweiterungsschnittstellen vorsehen
6. **Token-Effizienz** — nach jedem Gespräch eine 【Kontextzusammenfassung】 und ein 【Wiederaufnahme-Token】 erzeugen

## Schnellstart (3 Schritte)

```bash
# 1. Die Mentorenrolle in dein Projekt kopieren (umbenennen)
cp xiaomi-mimo/AGENTS.md AGENTS.md

# 2. (Empfohlen) Sicherheits-, Stil- und Workflow-Vorgaben ebenfalls hinzufügen
cp xiaomi-mimo/security.md security.md
cp xiaomi-mimo/style.md style.md
cp xiaomi-mimo/workflow.md workflow.md
```

3. Xiaomi MIMO starten und sagen:

> „Ich bin ein kompletter Anfänger. Hier ist meine Projektanforderungsspezifikation: Projektname ____, Kernziele ____, Benutzerrollen ____, Kernarbeitsabläufe ____, zu speichernde Daten ____. Beginne mit Phase 0: Umgebungseinrichtung & Technologie-Stack-Auswahl und führe mich Schritt für Schritt."

Die KI arbeitet sich durch „Design → Kernlogik → Oberfläche → Tests" vor und wartet in jeder Phase auf deine Bestätigung.

## Dateistruktur

```
AI_Model_Development_Mentor/
├── README.md            # Mehrsprachige Einstiegsseite
├── LICENSE              # MIT-Lizenz
├── zh-CN/               # Chinesisch
│   ├── README.md        # Chinesischer Einstieg
│   └── xiaomi-mimo/     # Xiaomi-MIMO-Build
│       ├── AGENTS.md    # Mentorenrolle (ZH)
│       ├── security.md  # Sicherheitsvorgaben (ZH)
│       ├── style.md     # Interaktionsstil (ZH)
│       └── workflow.md  # Entwicklungs-Workflow (ZH)
├── en-US/               # Englisch
│   ├── README.md        # Englischer Einstieg
│   └── xiaomi-mimo/     # Xiaomi-MIMO-Build
│       ├── AGENTS.md    # Mentorenrolle (EN)
│       ├── security.md  # Sicherheitsvorgaben (EN)
│       ├── style.md     # Interaktionsstil (EN)
│       └── workflow.md  # Entwicklungs-Workflow (EN)
└── de-DE/               # Deutsch
    ├── README.md        # Deutscher Einstieg (diese Datei)
    └── xiaomi-mimo/     # Xiaomi-MIMO-Build
        ├── AGENTS.md    # Mentorenrolle (DE)
        ├── security.md  # Sicherheitsvorgaben (DE)
        ├── style.md     # Interaktionsstil (DE)
        └── workflow.md  # Entwicklungs-Workflow (DE)
```

> 📦 Weitere Produkt-Builds werden als Schwestverzeichnisse unter jedem Sprachordner ergänzt, z. B. `zh-CN/claude-code/`, `en-US/cursor/`.

## Häufige Fragen

**F: Muss ich alle 4 Module verwenden?**
A: Nein. Nur `AGENTS.md` ist Pflicht. Füge `security.md` für stärkere Schutzvorkehrungen hinzu und `style.md` für eine freundlichere Gesprächserfahrung.

**F: Funktioniert das auch mit anderen KI-Produkten?**
A: Derzeit wird nur Xiaomi MIMO unterstützt. Optimierte Versionen für andere Produkte sind in Arbeit — hinterlasse einen Kommentar, um uns mitzuteilen, was du brauchst.

## Lizenz

[MIT-Lizenz](../LICENSE) © 2026 guapimm
