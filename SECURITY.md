# Security Policy

## Supported versions

AirBattery has not published a stable release. Security fixes currently target the latest development branch only.

## Reporting a vulnerability

Use a GitHub Security Advisory through the repository’s private vulnerability-reporting interface when it is available. Do not open a public issue with vulnerability details. Do not publish exploit details, raw Bluetooth captures, user identifiers, or secrets in a public issue.

A useful report includes:

- affected commit or version;
- operating system and desktop environment;
- impact and realistic attack path;
- minimal reproduction steps;
- whether raw device identifiers or local files are exposed;
- suggested mitigation, when known.

If private reporting is not enabled, open a public issue containing only a request for a private contact channel and no vulnerability details.

## Security boundaries

AirBattery is designed around these assumptions:

- Bluetooth advertisements are unauthenticated input.
- The desktop process, not the webview or GNOME extension, owns Bluetooth parsing and filesystem access.
- Frontend commands validate inputs and Tauri capabilities remain narrowly allow-listed.
- Raw Bluetooth addresses must not cross diagnostic or presentation boundaries.
- No telemetry, account, or cloud service is required for local monitoring.
- Update and release signing must remain disabled until maintainers provide and protect real signing credentials.

## Disclosure and remediation

Maintainers should acknowledge a private report, reproduce it, assess affected versions, develop a regression test, and coordinate a release before public disclosure. No response-time guarantee is claimed during alpha development.
