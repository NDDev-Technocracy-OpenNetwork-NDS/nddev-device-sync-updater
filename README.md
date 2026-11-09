# NDDev OpenNetwork · updater

AGPL-3.0-only NDS adapter. [NDDev OpenNetwork](https://nddev.ai).

Read the native policy plan and last actual update report. Planning performs no update or network discovery.

`contracts/module.json` is the canonical compiled-in runtime descriptor;
`module.yaml` identifies its repository package. Caller-owned local/process
capability handles share a bounded agent admission budget. Native responses,
private paths and content are never Debug or telemetry payloads.

`just check` validates formatting and locked dependency licenses, sources and use without
compiling or starting an updater. On an execution host or runner, run `just test`, then
`just provider-check` with the explicit `NDS_TEST_*` provider paths used by CI.
Acceptance exercises the real pinned native owner
in isolated temporary state. Linux provider acceptance and portable source
checks are distinct; an absent provider is unavailable, not a successful stub.
There is no backup/recovery implementation or private estate configuration.

Release journals live in [GitHub Releases](https://github.com/NDDev-Technocracy-OpenNetwork-NDS/nddev-device-sync-updater/releases).
