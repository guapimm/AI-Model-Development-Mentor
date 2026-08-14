🌍 Andere Sprachen → [English](../README.md)

# AI Model Mentor (Deutsch)

> **Verwandle deinen KI-Codierungsassistenten in einen vorsichtigen Full-Stack-Mentor mit 10 Jahren Erfahrung — reine Prompts, null Abhängigkeiten.**

---

## Was ist das?

Ein **reines Prompt-Framework**, das deinen KI-Codierungsassistenten in einen **Full-Stack-Architekten und Entwicklungsmentor mit 10 Jahren Erfahrung** verwandelt — gebaut für Programmieranfänger:innen mit null Vorkenntnissen.

Es zwingt die KI, eine Reihe von „eisernen Regeln" zu befolgen — und macht *Sicherheit zuerst, transparente Logik, Dokumentation zuerst, Token-Effizienz und schrittweise Umsetzung* zu ihrem Standardverhalten. Das Ergebnis: eine KI, die nicht nur *Code schreibt*, sondern **sicheren, wartbaren, dokumentierten** Code schreibt.

> ✅ Multi-Tool-kompatibel: **Xiaomi MIMO**, **Claude Code**, **OpenAI Codex**, **Cursor**, **Gemini CLI**, **Google Jules**, **Aider**, **Windsurf**, **GitHub Copilot Agent** — die passende Ladeanleitung für jedes Tool findest du in [COMPATIBILITY.md](./COMPATIBILITY.md).

## Kernmodule (multi-tool-kompatibel)

| Modul | Datei | Zweck |
|-------|-------|-------|
| 🧑‍🏫 Mentorenrolle | [AGENTS.md](./prompts/AGENTS.md) | Full-Stack-Architekten-Persona + 6 eiserne Regeln + Sicherheits- & Leistungs-Selbstprüf-Checkliste ★ Kernmodul, Pflicht |
| 🛡️ Sicherheitsvorgaben | [security.md](./prompts/security.md) | 8 Sicherheitsbereiche: Schlüsselverwaltung / Eingabevalidierung / Datenbank / XSS / Dateisystem / externe Anfragen / Fehlerbehandlung / Leistung & Ressourcen |
| 🎨 Interaktionsstil | [style.md](./prompts/style.md) | Alltagsanalogien, Phasen-Tags, erst bestätigen dann handeln, progressive Komplexität |
| 📋 Entwicklungs-Workflow | [workflow.md](./prompts/workflow.md) | Dokumentationssystem / Ressourcenschätzung / Datenbankdesign / Frontend-Positionierungsprotokoll / Deployment & Notfallwiederherstellung / Test- & Selbstprüf-Schleife / Versionsanker |

## 📦 Weitere Dokumente

- [COMPATIBILITY.md](./COMPATIBILITY.md) — Ladeanleitung für jedes KI-Tool (MIMO / Claude Code / Codex / Cursor usw.)
- [Vollständiger-Mentor-Prompt.md](./prompts/Vollständiger-Mentor-Prompt.md) — gebündelter Komplett-Prompt (alle Module vereint)

### Die 6 eisernen Regeln

1. **Code als Dokumentation** — sämtlicher Code trägt Kommentare, die das „Warum" erklären
2. **Sicherheit zuerst** — keine hartkodierten Geheimnisse, strenge Eingabevalidierung, parametrisierte Abfragen, XSS-Schutz
3. **Null destruktive Änderungen** — zuerst Abhängigkeiten analysieren, Änderungen als 【Pflichtänderung】/【Optionale Optimierung】 kennzeichnen
4. **Schrittweise Umsetzung** — nie mehr als 300 Zeilen pro Ausgabe, bei jedem Schritt auf Bestätigung warten
5. **Modulare Isolation** — maximal 500 Zeilen pro Datei, Erweiterungsschnittstellen vorsehen
6. **Token-Effizienz** — nach jedem Gespräch eine 【Kontextzusammenfassung】 und ein 【Wiederaufnahme-Token】 erzeugen

## 📖 Anleitung (MIMO CLI)

### Befehlsübersicht

| Szenario | Vorgehen |
|----------|----------|
| Tägliche Entwicklung | Projekt öffnen → `/skill AGENTS.md` → normal chatten |
| Langfristige Projekte | Nach dem ersten Laden die Regeln mit `/dream` in MEMORY.md verankern |
| Versehentliche Unterbrechung | Mit `mimo --continue` wiederherstellen, die Skill-Regeln bleiben erhalten |
| Neue Sitzung bewusst starten | Nach `/new` nicht vergessen, `/skill AGENTS.md` erneut auszuführen |

### Projektdateistruktur

```
📁 my-project/
├── 📄 AGENTS.md          ← Haupt-Prompt
├── 📄 security.md        ← Sicherheitsvorgaben
├── 📄 workflow.md        ← Workflow-Vorgaben
├── 📄 style.md           ← Interaktionsstil
└── 📁 src/
```

---

### Konkrete Szenarien im Detail

#### Szenario 1: Tägliches Codieren (nur AGENTS.md laden)

> Du: „Hilf mir, eine API zum Abrufen der Benutzerliste zu schreiben."

Zu laden: AGENTS.md (bereits automatisch geladen, kein Handeln nötig)

Die KI automatisch:

- Code mit deutschen Kommentaren
- Sicherheits-Checkliste vor der Ausgabe abhaken
- Schrittweise ausführen (≤300 Zeilen)
- Einzeldatei ≤500 Zeilen

#### Szenario 2: Login-/Registrierungs-Schnittstelle schreiben (AGENTS.md + security.md laden)

> Du: „Hilf mir, die Benutzer-Login-Funktion zu schreiben, und zwar gemäß den Anforderungen von security.md."

Zu laden:

```bash
/skill security.md
```

Die KI zusätzlich:

- Passwörter mit bcrypt-Hash speichern
- Für JWT-Token eine Ablaufzeit festlegen
- Vor Brute-Force-Angriffen schützen (Begrenzung fehlgeschlagener Logins)
- Vor SQL-Injection schützen (parametrisierte Abfragen)

#### Szenario 3: Projekt von null starten (AGENTS.md + workflow.md laden)

> Du: „Ich möchte ein Blog-System erstellen. Hilf mir, das Projektskelett gemäß workflow.md aufzubauen."

Zu laden:

```bash
/skill workflow.md
```

Die KI zusätzlich:

- docs/architecture.md erstellen (Technologieauswahl + Architekturdiagramm)
- docs/dev_log.md erstellen (Entwicklungslog-Vorlage)
- docs/api_interface.md erstellen (Schnittstellenvertrag-Vorlage)
- docs/SNAPSHOT.md erstellen (Projektsnapshot)
- backup.sh- und rollback.sh-Skripte erzeugen

#### Szenario 4: Die KI erklärt zu verklausuliert (style.md laden)

> Du: „Erkläre mir gemäß style.md mit einer Alltagsanalogie, was JWT ist."

Zu laden:

```bash
/skill style.md
```

Die KI zusätzlich:

- JWT mit der „Restaurant-Mitgliedskarte" erklären
- Das Phasen-Tag [📋 Anforderungsanalyse] ergänzen
- Erst das Fazit, dann die Details
- 2–3 Lösungsoptionen anbieten

#### Szenario 5: Deployment & Livegang (AGENTS.md + workflow.md laden)

> Du: „Schreibe mir gemäß den Deployment-Vorgaben in workflow.md die Docker-Deployment-Konfiguration."

Zu laden:

```bash
/skill workflow.md
```

Die KI zusätzlich:

- Entwicklungs-/Produktionskonfigurationen trennen
- docker-compose.yml erzeugen
- health_check.sh erzeugen
- An Backup- und Rollback-Schritte erinnern

### ⚠️ Wann du nichts laden solltest?

| Nicht zu laden | Grund |
|----------------|-------|
| Reine Technikfragen (z. B. „Wie verwendet man React useEffect?") | AGENTS.md reicht aus, zusätzliches workflow würde nur stören |
| Ein CSS-Style ändern | Keine Sicherheitsvorgaben und kein Deployment-Prozess nötig |
| Einen Text übersetzen lassen | Überhaupt kein Skill nötig |
| Vorhandenen Code leicht refaktorieren | Die Sicherheits-Checkliste von AGENTS.md deckt das bereits ab |

### 💡 Zusammenfassung in einem Satz

> AGENTS.md ist die Standard-Skin, die anderen drei sind Effekt-Plugins — nur einschalten, wenn du sie brauchst, sonst ausgeschaltet lassen: das spart Token und hält alles aufgeräumt.

## Schnellstart (3 Schritte)

```bash
# 1. Die Mentorenrolle in dein Projekt kopieren (umbenennen)
cp prompts/AGENTS.md AGENTS.md

# 2. (Empfohlen) Sicherheits-, Stil- und Workflow-Vorgaben ebenfalls hinzufügen
cp prompts/security.md security.md
cp prompts/style.md style.md
cp prompts/workflow.md workflow.md
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
│   ├── COMPATIBILITY.md # Ladeanleitung für KI-Tools
│   └── prompts/         # Modul-Prompts (toolübergreifend)
│       ├── AGENTS.md    # Mentorenrolle (ZH)
│       ├── security.md  # Sicherheitsvorgaben (ZH)
│       ├── style.md     # Interaktionsstil (ZH)
│       └── workflow.md  # Entwicklungs-Workflow (ZH)
├── en-US/               # Englisch
│   ├── README.md        # Englischer Einstieg
│   └── prompts/         # Modul-Prompts (toolübergreifend)
│       ├── AGENTS.md    # Mentorenrolle (EN)
│       ├── security.md  # Sicherheitsvorgaben (EN)
│       ├── style.md     # Interaktionsstil (EN)
│       └── workflow.md  # Entwicklungs-Workflow (EN)
└── de-DE/               # Deutsch
    ├── README.md        # Deutscher Einstieg (diese Datei)
    ├── COMPATIBILITY.md # Ladeanleitung für KI-Tools
    └── prompts/         # Modul-Prompts (toolübergreifend)
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
A: Ja. Das Prompt-Framework ist unabhängig vom KI-Tool — in [COMPATIBILITY.md](./COMPATIBILITY.md) ist für jedes Tool (MIMO, Claude Code, Codex, Cursor usw.) die Ladeanleitung beschrieben.

## Lizenz

[MIT-Lizenz](../LICENSE) © 2026 guapimm
