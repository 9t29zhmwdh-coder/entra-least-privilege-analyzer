<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>Entra Least-Privilege Analyzer</h1>
</div>

> 🇬🇧 [English Version](README.md)

**Read-only Rust CLI zur Analyse von Entra ID Berechtigungskonfigurationen, Erkennung überprivilegierter Accounts, Rollen-Overlap und PIM-Lücken.**

Der Entra Least-Privilege Analyzer verbindet sich per Anwendungsberechtigungen mit der Microsoft Graph API und erstellt einen strukturierten Berechtigungsbericht. Vollständig read-only, keine Daten verlassen das lokale Gerät.

![Rust](https://img.shields.io/badge/Rust-1.78+-orange?logo=rust)
![Microsoft Entra ID](https://img.shields.io/badge/Microsoft%20Entra%20ID-blue?logo=microsoftazure)
![Plattform](https://img.shields.io/badge/Plattform-Windows%20%7C%20Linux-lightgrey?logo=windows)
![Lizenz](https://img.shields.io/badge/Lizenz-MIT-green)
[![Azure Ready](https://img.shields.io/badge/Azure-Graph%20API%20%7C%20PIM-blue?logo=microsoftazure)](docs/graph_api_setup.md)
[![CI](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer/actions/workflows/ci.yml)

---

## Funktionen

| Funktion | Beschreibung |
|---|---|
| Privilege Scoring | Gewichtete Punktzahl pro Account basierend auf gehaltenen Rollen |
| Erkennung überprivilegierter Accounts | Flaggt Accounts über konfigurierbare Score-Schwellenwerte |
| Rollen-Overlap-Analyse | Erkennt redundante oder konfliktbehaftete Rollenzuweisungen |
| PIM-Lückenerkennung | Erkennt permanente Hochprivileg-Zuweisungen und schwache PIM-Einstellungen |
| PIM-Einstellungsaudit | Prüft MFA-Anforderung, Begründungspflicht, maximale Aktivierungsdauer |
| JSON / Markdown Export | Strukturierte Ausgabe für Tickets, Audits und Dokumentation |
| SARIF-Stub | Vorbereitet für GitHub Advanced Security Integration (v0.2) |

---

## Benötigte Graph API Berechtigungen

Registriere eine Anwendung in Entra ID mit folgenden **Anwendungsberechtigungen** (nicht delegiert):

| Berechtigung | Zweck |
|---|---|
| `Directory.Read.All` | Benutzer und Gruppenmitgliedschaften lesen |
| `RoleManagement.Read.All` | Rollendefinitionen und Zuweisungen lesen |
| `PrivilegedAccess.Read.AzureAD` | PIM Eligible und Active Assignments lesen |
| `Policy.Read.All` | Rollenverwaltungsrichtlinien und PIM-Einstellungen lesen |

Alle Berechtigungen sind **read-only**. Es werden keine Schreibberechtigungen benötigt oder verwendet.

---

## App-Registrierung einrichten

1. Im [Azure Portal](https://portal.azure.com) zu **Entra ID > App-Registrierungen > Neue Registrierung** navigieren
2. Anwendung benennen (z.B. `elpa-analyzer`) und registrieren
3. Unter **API-Berechtigungen** die vier oben genannten Berechtigungen hinzufügen
4. Administratorzustimmung für den Mandanten erteilen
5. Unter **Zertifikate & Geheimnisse > Neuer geheimer Clientschlüssel** den Wert kopieren
6. **Mandanten-ID**, **Client-ID** und **Geheimer Clientschlüssel** notieren

---

## Schnellstart

```bash
git clone https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer
cd entra-least-privilege-analyzer

# .env erstellen und Zugangsdaten eintragen
cp .env.example .env

cargo build --release

# Vollständige Analyse
./target/release/elpa analyze

# Nur PIM-Analyse
./target/release/elpa pim

# Export als Markdown
./target/release/elpa export --format md --output bericht.md

# Export als JSON
./target/release/elpa export --format json --output bericht.json
```

---

## Konfiguration

`.env` Datei im Projektverzeichnis anlegen:

```env
ENTRA_TENANT_ID=deine-mandanten-id
ENTRA_CLIENT_ID=deine-client-id
ENTRA_CLIENT_SECRET=dein-clientschluessel
```

Die `.env`-Datei ist in `.gitignore` aufgeführt. Zugangsdaten werden nie committet.

---

## Schweregrade der Befunde

| Stufe | Bedeutung | Beispiele |
|---|---|---|
| Critical | Sofortiger Handlungsbedarf | Permanenter Global Admin ohne PIM |
| High | Im nächsten Sprint beheben | PIM ohne MFA, überprivilegierter Account |
| Medium | Innerhalb von 30 Tagen beheben | Rollen-Overlap, langer PIM-Aktivierungszeitraum |
| Low | Best Practice Verbesserung | Fehlende Begründungspflicht |

---

## Voraussetzungen

- Rust 1.78+
- Entra ID Mandant mit App-Registrierung
- Netzwerkzugang zu `login.microsoftonline.com` und `graph.microsoft.com`

---

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Early Release · **Zuletzt aktualisiert:** Juni 2026
