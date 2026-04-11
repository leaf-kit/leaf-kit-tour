# leaf-kit-tour

<p align="center">
  <img src="images/logo.png" alt="leaf-kit-tour" width="600" />
</p>

> [한국어](README.md)

> An interactive installer to easily set up leaf-kit Markdown CLI tools via Homebrew.

## Installation

```bash
brew tap leaf-kit/leaf-kit-tour
brew install leaf-kit-tour
```

## Usage

```bash
# Interactive language selection (default)
leaf-kit-tour

# English directly
leaf-kit-tour --lang en

# Korean directly
leaf-kit-tour --lang ko
```

## Features

- **Auto-update** — automatically checks for and applies updates on launch
- **Install status** — shows real-time install state of each tool via `brew list`
- **Install / Reinstall / Upgrade / Uninstall** — all from an interactive menu
- **Multilingual** — Korean and English with interactive language selection at startup

## Terminal Screenshots

### Language Selection (no arguments)

```
══════════════════════════════════════════════════════════════
          leaf-kit-tour  —  CLI Toolkit Installer
══════════════════════════════════════════════════════════════

[언어 선택 / Select Language]
  1  한국어 (기본값)
  2  English

선택/select (1)> 2
```

### Start Screen (English)

```
══════════════════════════════════════════════════════════════
          leaf-kit-tour  —  CLI Toolkit Installer
                        v0.4.0
══════════════════════════════════════════════════════════════

Install leaf-kit Markdown CLI tools easily via Homebrew.

[OK] Homebrew detected

[*] Checking for leaf-kit-tour updates...
[OK] Already up to date. (v0.4.0)
```

### Tool List & Install Status

```
══════════════════════════════════════════════════════════════════════════
  #  Status  Tool          Description                            Install
══════════════════════════════════════════════════════════════════════════
  1  [latest] v1.2.0  stylemd       All-in-one Markdown toolkit   SRC/BIN
                                    $ stylemd lint README.md
                                    $ stylemd format --fix docs/
                                    https://github.com/leaf-kit/style.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  2  [not installed]   playgraph    Animated Markdown viewer       SRC/BIN
                                    https://github.com/leaf-kit/playgraph.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  3  [upgrade] v0.8.0→v0.9.1  lsmd Markdown-aware dir listing    SRC/BIN
                                    $ lsmd --tree docs/
                                    https://github.com/leaf-kit/ls.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  4  [not installed]   gmd          Grep Markdown — fast search    SRC/BIN
                                    $ gmd search "API" docs/
                                    https://github.com/leaf-kit/g.md
  - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
  5  [latest] v1.0.0  bark         Terminal Markdown viewer        SRC/BIN
                                    $ bark README.md
                                    https://github.com/leaf-kit/bark.md
══════════════════════════════════════════════════════════════════════════
  i SRC = Source (Formula) build  BIN = Pre-built binary (Bottle)
```

> **Status indicators**
> - `[latest] vX.Y.Z` — installed, up to date
> - `[upgrade] vX.Y.Z → vA.B.C` — installed, newer version available
> - `[not installed]` — not yet installed
>
> **Install type**
> - `SRC` — build from source via Formula (green)
> - `BIN` — pre-built binary via Bottle (cyan)

### Menu Options

```
[Options]
  a  Install all CLI tools
  N  Select by number (e.g. 1,3,5 or 1-3)
  r  Reinstall mode
  u  Upgrade all installed tools
  d  Uninstall mode (by number or all)
  p  Show supported platforms
  s  Refresh install status
  q  Quit

select>
```

## Available CLI Tools

| # | Tool | Description | Install Type | Install Command |
|---|------|-------------|-------------|-----------------|
| 1 | **stylemd** | All-in-one Markdown toolkit (lint, format, fix, analyze) | 🟢 Source &nbsp; 🔵 Binary | `brew install leaf-kit/stylemd/stylemd` |
| 2 | **playgraph** | Animated Markdown viewer (graphs & diagrams) | 🟢 Source &nbsp; 🔵 Binary | `brew install leaf-kit/playgraph/playgraph` |
| 3 | **lsmd** | Markdown-aware directory listing tool | 🟢 Source &nbsp; 🔵 Binary | `brew install leaf-kit/lsmd/lsmd` |
| 4 | **gmd** | Grep Markdown — structure-aware fast search | 🟢 Source &nbsp; 🔵 Binary | `brew install leaf-kit/gmd/gmd` |
| 5 | **bark** | Terminal Markdown viewer (Browse And Render marKdown) | 🟢 Source &nbsp; 🔵 Binary | `brew install leaf-kit/bark/bark` |

> **Install type legend**
> - 🟢 **Source (Formula)** — builds from source via `brew install` (requires Rust toolchain)
> - 🔵 **Binary (Cask/Bottle)** — installs a pre-built binary for instant setup

### Pre-built Binary (Bottle) Platform Support

| Platform | Architecture | Support |
|----------|-------------|---------|
| macOS Sonoma 14+ | Apple Silicon (arm64) | ✅ Supported |
| macOS Sonoma 14+ | Intel (x86_64) | ✅ Supported |
| macOS Ventura 13 | Apple Silicon (arm64) | ✅ Supported |
| macOS Ventura 13 | Intel (x86_64) | ✅ Supported |
| Linux (glibc 2.17+) | x86_64 | ✅ Supported |
| Linux (glibc 2.17+) | aarch64 | ✅ Supported |
| Windows (WSL2) | x86_64 | ⚠️ Via WSL2 + Homebrew |

> **Note**: Even on platforms without a pre-built Bottle, you can build from source via the Formula. Requires Rust 1.70+ toolchain.

### Command Examples

```bash
# stylemd — Markdown style lint & fix
stylemd lint README.md
stylemd format --fix docs/

# playgraph — Animated Markdown viewer (UI application)
# Runs as a UI app, not a terminal CLI

# lsmd — Markdown directory listing
lsmd
lsmd --tree docs/

# gmd — Markdown structure search
gmd search "API" docs/
gmd headings README.md

# bark — Terminal Markdown rendering
bark README.md
bark --theme dark guide.md
```

### Related Repositories

- [style.md](https://github.com/leaf-kit/style.md)
- [playgraph.md](https://github.com/leaf-kit/playgraph.md)
- [ls.md](https://github.com/leaf-kit/ls.md)
- [g.md](https://github.com/leaf-kit/g.md)
- [bark.md](https://github.com/leaf-kit/bark.md)

## Options

| Flag | Description |
|------|-------------|
| (none) | Interactive language selection (default: Korean) |
| `--lang ko` | Run with Korean interface |
| `--lang en` | Run with English interface |
| `--version` | Show version |
| `--help` | Show help |

## Platform Support

| OS | Architecture | Homebrew Install | Status |
|----|-------------|-----------------|--------|
| macOS 13+ | Apple Silicon (arm64) | `brew install` | 🟢 Primary |
| macOS 13+ | Intel (x86_64) | `brew install` | 🟢 Primary |
| Linux | x86_64 / aarch64 | `brew install` (Linuxbrew) | 🟡 Supported |
| Windows | x86_64 | WSL2 + `brew install` | 🟡 Supported |

## License

MIT
