# User Guide – Schließfach-Manager v2.1

## Überblick

Der Schließfach-Manager ist eine Terminal-Anwendung zur Verwaltung von Schließfächern, Verleihvorgängen und Finanzdaten. Die Oberfläche besteht aus Header, Hauptbereich, Keybind-Bar und Status-Bar.

## Dashboard

Das Dashboard liefert eine Übersicht zur Auslastung, benötigten Aktionen, Standorten, Finanzen und dem Belegungstrend. Die Bereiche bleiben bei Terminal-Resize stabil (Fixed-Height Layout).

## Verleih-Management

Die Verleih-Workflows laufen als dialogartige Wizards:

1. **Search** – Neuer Verleih (Standort + Größe auswählen)
2. **List** – Übersicht aller Verleihvorgänge
3. **Extend** – Vertrag verlängern
4. **Return** – Rückgabe eines Schließfachs
5. **Damage** – Defekt melden oder reparieren

## Finanzen

Die Finanzansicht zeigt ausstehende Zahlungen, Einnahmen und den Zahlungsverlauf. Export/Import wird hier über `E` und `I` ausgelöst.

## Verwaltung

Unter Verwaltung lassen sich Schließfächer, Standorte, Einstellungen und der Audit-Log einsehen. Einstellungen werden mit den Pfeiltasten angepasst und mit `Enter` gespeichert.

## Screensaver

Nach konfigurierbarer Inaktivität startet ein Countdown in der Status-Bar. Nach Ablauf wird der Screensaver aktiviert, der durch beliebige Tasteneingabe beendet wird (Rückkehr zum Dashboard).
