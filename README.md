# x86 BIOS Simulator

A narrative puzzle game set in a 1998-era PC. You discover your late grandfather's old computer in the attic and uncover secrets hidden in its BIOS, floppy disks, and hard drive sectors.

## Features

- **Authentic BIOS simulation** — PhoenixBIOS 4.0 POST, DOS 6.22 command line
- **Narrative-driven gameplay** — 4 chapters, multiple endings, bilingual (中文/English)
- **Interactive room** — Click objects to explore, collect floppy disks, trigger dialogue
- **140+ voice lines** — Bilingual audio with character-specific voices
- **Educational** — Learn real BIOS concepts: interrupts, FAT12, MBR, CHS addressing

## Controls

| Key | Action |
|-----|--------|
| Click | Power on / interact with objects |
| Type | DOS commands (dir, type, help, cls) |
| R | Enter room exploration |
| D | Demo dialogue |
| L | Toggle language (中文/English) |
| Enter/Space | Advance dialogue |
| Escape | Skip dialogue / return to DOS |

## DOS Commands

```
dir          — List files on disk
type <file>  — Display file contents (try: type README.TXT)
help         — Show all commands
cls          — Clear screen
ver          — Show DOS version
mem          — Display memory usage
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized, stripped)
cargo build --release

# Run
cargo run --release
```

## Project Structure

```
src/
├── main.rs           — Game loop and state management
├── config.rs         — Window, VGA, language settings
├── i18n.rs           — Chinese/English localization
├── core/             — BIOS emulation (CPU, memory, interrupts)
├── bios/             — Interrupt service routines
├── filesystem/       — FAT12, MBR, disk images
├── game/             — Dialogue, DOS commands, tasks, save
├── render/           — VGA, CRT shader, room scene
└── audio/            — WAV playback

assets/
├── fonts/            — MiSans CJK font, VGA bitmap font
├── shaders/          — CRT fragment shader
└── audio/            — Bilingual WAV files (9 chapters, 139 files)

audio_gen/
└── scripts/          — game_script.json (dialogue data)
```

## License

MIT
