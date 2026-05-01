# suwayomi-rakuyomi-source

An [Aidoku](https://github.com/Aidoku/Aidoku) source for [Rakuyomi](https://github.com/tachibana-shin/rakuyomi) (KOReader manga plugin) that connects to your local [Suwayomi-Server](https://github.com/Suwayomi/Suwayomi-Server).

```
KOReader (Rakuyomi) ──► Aidoku Source (.aix) ──► Suwayomi Server ──► Your manga library
```

## Features

- Browse your Suwayomi library directly from Rakuyomi/KOReader
- Search manga by title
- Filter by status (Ongoing, Completed, Hiatus, Cancelled)
- Sort by title, last updated, or date added
- View recently updated chapters on the home screen
- Configurable server URL via source settings
- Optional Basic Auth support

## Requirements

- [Suwayomi-Server](https://github.com/Suwayomi/Suwayomi-Server) running and accessible on your network
- [Rakuyomi](https://github.com/tachibana-shin/rakuyomi) installed on KOReader

## Installation

### Step 1 — Add the source list to Rakuyomi's `settings.json`

On your e-reader, open the `rakuyomi/settings.json` file and add this source list URL to the `source_lists` array:

```json
{
  "$schema": "https://github.com/tachibana-shin/rakuyomi/releases/latest/download/settings.schema.json",
  "source_lists": [
    "https://aidoku-community.github.io/sources/index.min.json",
    "https://felipe-negri.github.io/suwayomi-rakuyomi-source/index.min.json"
  ],
  "languages": ["en"]
}
```

> The file is located in the `rakuyomi/` folder inside your KOReader directory. Common paths:
> - **Kobo:** `.adds/koreader/rakuyomi/settings.json`
> - **Kindle:** `koreader/rakuyomi/settings.json`
> - **Cervantes:** `/mnt/private/koreader/rakuyomi/settings.json`
> - **PocketBook:** `applications/koreader/rakuyomi/settings.json`

### Step 2 — Install the source

1. In Rakuyomi, open the menu (☰) and go to **Manage Sources**.
2. Tap ➕ to browse available sources.
3. Find **Suwayomi** and tap to install it.

### Step 3 — Configure the server URL

After installing, tap ⚙️ next to the Suwayomi source and fill in:

| Setting | Description | Example |
|---------|-------------|---------|
| **Suwayomi Server URL** | Full URL to your Suwayomi server | `http://192.168.1.100:4567` |
| **Username** | Basic Auth username (optional) | — |
| **Password** | Basic Auth password (optional) | — |

> ⚠️ The **Server URL is required**. The source won't work until it's configured.

### Manual install (.aix file)

Alternatively, download `package.aix` from the [latest release](../../releases/latest) and install it manually via **Manage Sources → ➕ → local file**.

## Building from Source

```bash
rustup target add wasm32-unknown-unknown
cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli
aidoku package .
aidoku verify package.aix
```

## Architecture

The source communicates with Suwayomi via:
- **GraphQL** (`POST /api/graphql`) — manga list, chapters, home screen
- **REST v1** (`/api/v1/manga/{id}/thumbnail`) — cover images
- **GraphQL mutation** (`fetchChapterPages`) — page URLs for reading

## License

MIT
