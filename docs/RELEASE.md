# Release Process

AirBattery uses semantic versions. The current workspace version is `0.1.0-alpha.1`; no public release artifact exists yet.

## Release prerequisites

A release candidate must satisfy every applicable item and the project Definition of Done:

- clean Git worktree on the intended release commit;
- matching versions in workspace, desktop package, Tauri configuration, changelog, and release notes;
- Rust format, clippy with warnings denied, tests, and release build passing;
- frontend typecheck, tests, UI tests, and production build passing;
- Linux native runtime and GNOME 50 smoke tests recorded;
- Windows native runtime smoke tests recorded for a Windows release;
- AirPods hardware status stated exactly as validated or blocked;
- dependency/license audit reviewed;
- no unfinished production markers, skipped required tests, raw addresses, or hardcoded battery values;
- `.deb`, AppImage, GNOME ZIP, and Windows `.exe` NSIS files exist for the claimed release scope;
- each artifact installs, launches, upgrades, and uninstalls on its target system;
- `SHA256SUMS` contains every uploaded artifact;
- release notes list limitations and unverified device families.

## Version preparation

1. Update `CHANGELOG.md` and all version-bearing manifests.
2. Update `docs/PROJECT_STATUS.md` with final evidence.
3. Run the complete verification matrix in `docs/TESTING.md`.
4. Create a signed or annotated semantic tag only after the release commit is reviewed:

```bash
git tag -a v0.1.0-alpha.1 -m "AirBattery v0.1.0-alpha.1"
git push origin v0.1.0-alpha.1
```

Do not create or push a tag from an unverified local state.

## CI release behavior

The tag workflow builds Linux and Windows bundles on native GitHub-hosted runners, packages the GNOME extension, validates concrete artifact names, generates SHA-256 checksums, and uploads everything to a draft GitHub Release.

The workflow does not provide signing credentials. Unsigned artifacts must be labeled accordingly. A maintainer must inspect logs, install artifacts on clean target accounts, compare checksums, and manually publish the draft.

## Required release record

Store or link:

- release commit and tag;
- CI run URL;
- exact artifact filenames and sizes;
- `SHA256SUMS`;
- Linux/GNOME/Windows smoke-test records;
- hardware validation status;
- dependency audit output;
- known limitations and rollback instructions.

## Rollback

Do not replace an existing tag. Withdraw the affected release, document the reason, fix forward with a new version, and preserve the original checksums and incident record.
