# Orbis Desktop

The Orbis desktop app, built with [Tauri](https://tauri.app). It opens the Orbis web app in a native window: your local web app (http://localhost:3000) while developing, and the live site (https://orbis.polsar.studio) in the installed app. Both addresses are in `src-tauri/tauri.conf.json`.

## Develop

From the root of the Orbis monorepo:

```bash
pnpm run develop desktop
```

This starts the server, the web app, and the desktop window together. The first run compiles the Rust side, which takes a few minutes.

You need the Rust toolchain installed: https://www.rust-lang.org/tools/install

## Build an installer

```bash
pnpm --filter orbis-desktop run bundle
```

The installers end up in `src-tauri/target/release/bundle`.
