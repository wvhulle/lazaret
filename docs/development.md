# Development

## Build and test

```sh
nix develop path:.
cargo test
cargo clippy --all-targets
nix flake check path:.
```

`nix flake check` runs the unit tests, Clippy and formatting of the three crates, the checks of the NixOS and Home Manager modules, the screenshot and recording render and four NixOS VM tests.

It also checks the dependencies and the text:

- `dependencyAdvisories`: `cargo audit` against the RustSec advisory database pinned as the `advisory-db` flake input, which the weekly dependency update refreshes.
- `dependencyPolicy`: `cargo deny` with `deny.toml`: allowed licenses, crates.io as the only source, and no wildcard versions.
- `unusedDependencies`: `cargo machete`. Dependencies it can't see, such as a crate renamed in `[lib]`, go in `[package.metadata.cargo-machete]`.
- `spelling`: `typos` with `_typos.toml`, over the code and the English catalog. Other languages are excluded.
- `documentationLinks`: `lychee --offline` over the Markdown files, for relative links and anchors. External links are checked weekly by `.woodpecker/online-links.yaml`, which never fails the build.

The same tools are in the dev shell, so `cargo deny check`, `cargo machete`, `typos` and `lychee --offline docs` run locally. The dev shell also has `minisign`, to make the release key pair with `minisign -G -W` and to check a signed release asset. Run one VM test on its own with:

```sh
nix build path:.#checks.x86_64-linux.vm-live-tuning
```

The VM tests are `vm-detection-alerts`, `vm-live-tuning`, `vm-scheduled-scan` and `vm-signature-update`. Each one boots a desktop session with real ClamAV services and drives Lazaret through its D-Bus actions.

`src/host_verification_tests.rs` holds tests that need a real ClamAV host. They are ignored by default:

```sh
cargo test host_ -- --ignored
```

## Run

```sh
nix run path:.
```

Debug builds use the application ID `page.codeberg.wvhulle.Lazaret.Devel` and their own data directory, so they run next to an installed Lazaret without touching its quarantine or settings.

## Rules for changes

- Keep only features that help block viruses or exploits. Lazaret adds value on top of a ClamAV that is already running.
- Never bundle the scan engine. Find `clamdscan`, `clamscan` and `freshclam` with `host_services::clamav_programs::trusted_program_lookup`, which skips copies your account could replace, and run them through `host_services::desktop::flatpak_sandbox`, so the Flatpak keeps working.
- Talk to systemd through `src/host_services/systemd_bus/` and read the journal through `src/host_services/systemd_journal/`. Don't run `systemctl` or `journalctl`, and don't refresh on timers: react to bus signals and journal wakeups.
- The GUI never runs as root. `clamav_admin_service/` is the only privileged code: it authorizes every request with polkit and enforces `clamd_configuration/setting_lists/protected-clamav-settings.txt` itself, so never rely on the UI for that.
- Model scanner output as `impl Stream`. Split it on bytes and decode lossily, because file names can be arbitrary bytes. Sanitize scanner text before showing it, and keep `set_use_markup: false` on rows that show file names.
- Keep quarantine file operations no-follow and integrity-checked, in this order: copy, pending row, remove source, commit. Never abort a move in progress.
- Write each page, dialog, section and row as a Relm4 component whose model is plain data. Change it only in `update`, in answer to a message, and derive the view from it with `#[watch]`, `#[track]` and factories. Signal handlers only send messages. Return background work as a stream of command outputs and hand it to `run_stream`, keeping the guard it returns in the model so that replacing the state cancels it. Use `run_stream_to_completion` for quarantine and file moves. Wrap a blocking library such as the journal or a directory walk in an iterator and turn it into a stream with `blocking_iterator_stream`; wait for changes with journal wakeups, inotify or bus signals rather than timers. Name input messages after what happened (`StartScanRequested`, `StoredSettingChanged`) and command outputs after the finished work (`ProfilesLoaded`).
- Put a file where a user meets it in the window: a page's subpages and dialogs go inside its directory. Code without a screen goes in `host_services/`.

[Architecture](architecture.md) explains the reasons behind these rules.

## Changing the database schema

The schema of `lazaret.db` lives in the numbered files in `migrations/`:

1. Add the next numbered file, such as `migrations/0002_scan_durations.sql`, and append it to `LAZARET_MIGRATIONS` in `src/host_services/lazaret_database/embedded_migration_lists.rs`.
2. Never edit a file that a release has shipped: databases that already ran it won't run it again.
3. Test with a database at the previous version: create it with `create_migrated_test_database` and the list without your file, add rows, then open it with the store and check the rows.

## Translations

User-facing text lives in [Fluent](https://projectfluent.org) catalogs, one per language: `locales/<language>/lazaret.ftl`, with English in `locales/en/lazaret.ftl` as the source. `build.rs` turns every English message into a Rust function. The part of the id before the first dash names the module, so:

```ftl
scans-scanning-name = Scanning { $name }
scans-count-target =
    { $count ->
        [one] { $count } target
       *[other] { $count } targets
    }
```

becomes `translated_messages::scans::scanning_name(target_name)` and `translated_messages::scans::count_target(target_count)`. A variable used as a selector takes a number; any other variable takes anything that implements `Display`.

Give translators data, not finished text, so every language can build a grammatical sentence:

- A variable selected with plural categories, or written as `{ NUMBER($count) }`, takes a number. Use `NUMBER` for a second count in the same message (`{ NUMBER($count) } of { $total } engines`), so a translation may select on it.
- A variable selected with other keys is a choice and becomes an enum in the generated module: `{ $credential -> [purchase-receipt] … *[customer-identifier] … }` takes `translated_messages::signatures::CredentialChoice`. Translations may only use those keys, and one variable name has the same choices across a module.
- Never insert one translated message into another. Write a separate message for each case (`host-listed-malicious-unnamed-source`), or pass a choice. A reason inserted after another sentence (`… is unknown. { $reason }`) is itself a complete sentence.
- Name variables after what they hold, such as `$relative-time` for "2 days ago" and `$local-time` for a formatted date and time.

Name a message after where it appears, not after its English text, so rewording keeps the id: a module prefix (`startup`, `window`, `scans`, `findings`, `signatures`, `preferences`, `widgets`, `host`, `headless`), two to four words, and the widget role when there is one (`-title`, `-tooltip`, `-button`, `-heading`, `-body`). Give each place its own message even when the English is the same, because other languages may need different words. Write the application name as `{ -brand-name }`; every catalog defines that term.

To add or change text, edit the English catalog and call the function. The build fails when:

- a catalog has a syntax error;
- a message id has no module prefix, or doesn't convert to a Rust name;
- a translation uses a variable the English message doesn't pass, selects on one that English passes as text, or uses a choice key English doesn't have;
- a translation keeps a message that English no longer has, or uses a term its catalog doesn't define.

An English message that no code calls shows up as a dead-code warning. Untranslated messages fall back to English, and the build prints how many each language is missing.

To add a language, create `locales/<tag>/lazaret.ftl` with a BCP 47 tag such as `pt-BR`, starting with `-brand-name = Lazaret`. The language is chosen from `LANGUAGE`, `LC_ALL`, `LC_MESSAGES` and `LANG`, so `LANGUAGE=de cargo run` shows the German text.

## Screenshots and recordings

```sh
nix build path:.#screenshots
nix run path:.#render-screenshots -- dist/screenshots
```

Recordings and screenshots are rendered in a headless sway session with fixture ClamAV programs. Every scenario records an animated webp named after it, with the pointer shown before each click; the README and the guides embed these recordings. The PNG screenshots taken along the way are for the AppStream metainfo. `xtask/src/page_screenshots.rs` lists each scenario as typed steps that find widgets by accessibility role and name, such as `WidgetTarget::button("Start Scan")`, so layout changes don't break them. To see the names on a page, run `nix build path:.#render-screenshots` and then `result/bin/list-widgets -- --page=manual-scan`.

On every push to `main`, CI renders the recordings and screenshots and pushes them, as wvhulle-bot, to the `screenshots` branch. The README, the guides and the metainfo link to that branch. Never commit images to `main`.

## Packaging

The Debian package is built inside a Debian container with the static xtask:

```sh
nix run path:.#prepare-ci-tools
podman run --rm -v $PWD:/src -w /src debian:trixie .ci-tools/xtask ci-build-debian-package
podman run --rm -v $PWD:/src -w /src ubuntu:26.04 .ci-tools/xtask check-debian-package-install
```

The install check installs the package with its recommended ClamAV packages and checks the shared libraries, the compiled settings schema, the admin service's polkit, D-Bus and systemd files, and that merging without runtime changes leaves `clamd.conf` unchanged.

The Flatpak bundle is built from `page.codeberg.wvhulle.Lazaret.json`:

```sh
nix run path:.#build-flatpak-bundle -- dist
flatpak install --user dist/lazaret-x86_64.flatpak
```

## Releases

Releases are cut by [release-plz](https://release-plz.dev/) on Codeberg, configured in `release-plz.toml`:

1. On every push to `main`, `.woodpecker/release.yaml` opens or updates a release pull request. It bumps the version from the conventional commits since the last tag and writes the changelog.
2. Merging that pull request tags `v<version>` and creates a Codeberg release.
3. The release starts `.woodpecker/debian-package.yaml`, which attaches `lazaret_<version>_amd64.deb`, and `.forgejo/workflows/flatpak.yaml`, which attaches `lazaret-x86_64.flatpak`. Each file gets a `.sha256` checksum and a `.minisig` signature next to it, made with the minisign key in the `minisign_secret_key` secret (Woodpecker) and `MINISIGN_SECRET_KEY` (Forgejo). Without that secret the upload fails rather than publishing unsigned files. The matching public key is `minisign.pub` in the repository root.

A weekly cron job, `.woodpecker/dependency-update-pr.yaml`, updates `Cargo.lock` and `flake.lock` on the `bot/dependency-updates` branch and opens a pull request. Everything automatic that pushes, opens pull requests or creates releases acts as `wvhulle-bot`, with the `bot_token` secret.
