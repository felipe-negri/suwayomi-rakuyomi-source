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

### Via GitHub Pages (recommended)

1. In Rakuyomi, go to **Settings → Source Lists → Add**.
2. Enter the source list URL:
   ```
   https://felipe-negri.github.io/suwayomi-rakuyomi-source/index.min.json
   ```
3. Find **Suwayomi** in the source list and install it.

### Manual (.aix file)

1. Download `package.aix` from the [latest release](../../releases/latest).
2. In Rakuyomi, go to **Settings → Sources → Add Source** and select the file.

## Configuration

After installing, go to **Rakuyomi → Sources → Suwayomi → ⚙️ Settings** and fill in:

| Setting | Description | Example |
|---------|-------------|---------|
| **Suwayomi Server URL** | Full URL to your Suwayomi server | `http://192.168.1.100:4567` |
| **Username** | Basic Auth username (optional) | — |
| **Password** | Basic Auth password (optional) | — |

> ⚠️ The server URL is **required**. The source won't work until it's configured.

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
