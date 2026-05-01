# suwayomi-aidoku-source

An [Aidoku](https://github.com/Aidoku/Aidoku) source that bridges [Rakuyomi](https://github.com/tachibana-shin/rakuyomi) (the KOReader manga plugin) with your local [Suwayomi-Server](https://github.com/Suwayomi/Suwayomi-Server).

```
KOReader (Rakuyomi) ──► Aidoku Source (.aix) ──► Suwayomi Server ──► Your manga library
```

## Features

- Browse your Suwayomi library directly from Rakuyomi/KOReader
- Search manga by title
- Filter by status (Ongoing, Completed, Hiatus, Cancelled)
- Sort by title, last updated, or date added
- View recently updated chapters on the home screen
- Configurable server URL
- Optional Basic Auth support

## Requirements

- [Suwayomi-Server](https://github.com/Suwayomi/Suwayomi-Server) running and accessible
- [Rakuyomi](https://github.com/tachibana-shin/rakuyomi) installed on KOReader
- KOReader with Aidoku SDK 0.7+ support

## Installation

### From GitHub Release

1. Download `package.aix` from the [latest release](../../releases/latest).
2. In Rakuyomi, go to **Settings → Sources → Add Source**.
3. Install `package.aix` from your local file or by URL.

### From Docker Container

If you run the source server via Docker (see below), install the source directly:

1. In Rakuyomi, add source URL: `http://<your-host>:4667/package.aix`

## Configuration

After installing, configure the source in **Rakuyomi → Sources → Suwayomi → Settings**:

| Setting | Description | Default |
|---------|-------------|---------|
| **Suwayomi Server URL** | Full URL to your Suwayomi server | `http://localhost:4567` |
| **Username** | Basic Auth username (leave blank if not needed) | — |
| **Password** | Basic Auth password (leave blank if not needed) | — |

## Docker Deployment

### Source server only (serves `package.aix`)

```bash
docker build -t suwayomi-source .
docker run -p 4667:80 suwayomi-source
```

The `.aix` file will be available at `http://localhost:4667/package.aix`.

### Full stack (Suwayomi + source server)

```bash
docker compose up -d
```

This starts:
- **Suwayomi Server** at `http://localhost:4567`
- **Source server** at `http://localhost:4667` (serves `package.aix`)

Install the source in Rakuyomi from `http://<host>:4667/package.aix`, then configure the server URL to point to `http://<host>:4567`.

## Building from Source

### Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install --git https://github.com/Aidoku/aidoku-rs aidoku-cli
```

### Build

```bash
aidoku package .
# Output: package.aix
```

### Verify

```bash
aidoku verify package.aix
```

## Architecture

The source communicates with Suwayomi exclusively via:
- **GraphQL** (`POST /api/graphql`) — for all metadata (manga list, chapters, home)
- **REST v1** (`/api/v1/manga/{id}/thumbnail`) — for cover images
- **GraphQL mutation** (`fetchChapterPages`) — to get the correct page URLs

## License

MIT
