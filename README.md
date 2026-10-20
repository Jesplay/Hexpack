# hexpack

A desktop tool for [Hex Casting](https://modrinth.com/mod/hex-casting) players. Draw patterns like you would
with a staff, see what each one is and what your hex resolves to, and pack all patterns as tightly as possible
onto the drawing grid, so even a long hex fits on one screen.

![hexpack](docs/screenshot.png)

## Features

- **Draw** patterns with the mouse on a hex grid. They glow and shimmer like in the mod; when a line passes a
  node it already visited, it bends around it so you can follow the stroke order.
- **Recognise** every pattern from its angles, in any rotation. Numerical Reflection values and
  Bookkeeper's Gambit masks are decoded; Introspection, Retrospection and Consideration are understood.
- **Check** the hex: the stack is simulated pattern by pattern, including Thoth's Gambit loop bodies, and shown as
  typed chips. Too few iotas, wrong types, swapped arguments, division by zero, per-world great spells and
  signature clashes between addons are reported.
- **Arrange**: pack all patterns, without sharing a grid point, into the smallest 16:9 (or 16:10, 4:3, 1:1, 21:9) area.
- **Search** more than 1200 patterns from Hex Casting and its addons (Ctrl+F) with a preview of each glyph,
  and click one to add it.
- **Open in Obsidian**: right-click a pattern to open its page in the bundled vault.
- **Import / copy** hexes as plain text, one pattern per line.
- A **command line mode** to check and pack `.hex` files without opening a window.

## Quick start

**Windows:** download `hexpack-windows-x64.zip` from the
[latest release](https://github.com/Jesplay/Hexpack/releases/latest), extract it and double-click `hexpack.exe`.
The zip contains the program and the vault; keep them in the same folder.

**From source (Windows and Linux):**

```bash
git clone https://github.com/Jesplay/Hexpack.git
cd Hexpack
cargo run --release
```

## The Hexwoven Vault

[`Hexwoven Vault/`](Hexwoven%20Vault) is an [Obsidian](https://obsidian.md) vault with one page per pattern,
book entry, iota type and addon. Open it in Obsidian to browse everything with links and a graph view.

hexpack reads its pattern data from that vault (the front matter of every file in `02 Patterns/`: signature,
inputs, outputs, cost, mod). It looks for a `Hexwoven Vault` folder

1. in the `HEXPACK_VAULT` environment variable, if set,
2. next to the program or up to three folders above it (so a build in `target/release` finds the one in this repository),
3. in the current folder.

If none is found, the app asks for the folder. On the command line, use `--vault <folder>`.

## Building

You need [Rust](https://rustup.rs) (stable).

### Linux

```bash
cargo build --release
```

The binary is `target/release/hexpack`. It needs a desktop session with OpenGL (X11 or Wayland); if linking
fails, install your distribution's X11/Wayland/OpenGL development packages. "Open in Obsidian" uses `xdg-open`.

### Windows

With the default MSVC toolchain (needs the Visual Studio Build Tools with *Desktop development with C++*):

```bash
cargo build --release
```

Or, without Visual Studio, with the GNU toolchain:

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
cargo +stable-x86_64-pc-windows-gnu build --release
```

The minimal MinGW that rustup ships lacks a few Windows import libraries; `build.rs` works around that, and
`Cargo.lock` pins a few dependencies to versions that do not need `dlltool`. Keep `Cargo.lock` when updating.

## Usage

### App

Run `hexpack` without arguments (or double-click it). `hexpack ui file.hex` opens the app with a hex loaded.

| Input | Action |
| --- | --- |
| Left drag | draw a pattern (drag back to remove the last stroke) |
| Right / middle drag | move the view |
| Mouse wheel | zoom |
| Click | select a pattern |
| Right click | pattern menu: open in Obsidian, copy angles, delete |
| Delete | delete the selected pattern |
| Ctrl+Z | undo the last pattern |
| Ctrl+F | search patterns |

The panel on the right shows what the hex resolves to, the stack after every pattern (*Resolution*), the list of
patterns, and all issues and media costs.

### Command line

```bash
hexpack examples/bridge.hex              # check and pack, writes examples/bridge.svg
hexpack examples/bridge.hex --check-only
hexpack search reflection                # find patterns
hexpack number 37                        # angles for Numerical Reflection: 37
```

Options: `--vault <folder>`, `--ratio 16:9`, `--time <seconds>`, `--svg <file>`, `--seed <n>`.

### Hex file format

```text
# comment
stack: vec, num            optional: what is already on the stack
Mind's Reflection          pattern names as in the vault (case-insensitive)
num: 10                    Numerical Reflection with a value
mask: v-                   Bookkeeper's Gambit ('-' keep, 'v' remove)
sig: qaq                   raw angles
Name @ Mod                 pick a mod when a name is ambiguous
Introspection              or {
  Place Block
Retrospection              or }
Thoth's Gambit
```

See [`examples/bridge.hex`](examples/bridge.hex): it places a 10-block bridge in the direction you are looking.

## How it works

- **Recognition.** A pattern is identified only by its sequence of turns (`w` straight, `e`/`d` right,
  `q`/`a` left), so the direction you start drawing in does not matter.
- **Checking.** Inputs and outputs come from the vault's free-text `input`/`output` fields, parsed into rough
  types (num, vec, entity, list, …). Stack operations, arithmetic, Thoth's Gambit and Hermes' Gambit are modelled
  explicitly. Where the vault has no information, the stack is marked as unknown from that point instead of
  reporting false errors. This catches wrong counts, swapped arguments and obvious type errors, but cannot
  guarantee that a hex works in game.
- **Packing.** Each pattern is a fixed set of grid points that may be rotated in 6 directions (not mirrored) and
  moved; no two patterns may share a point. The score is the size of the smallest rectangle with the chosen aspect
  ratio around everything. First, patterns are placed greedily (largest first, trying every rotation and position,
  preferring spots with many neighbours) with many randomised orders. Then *ruin and recreate* repeatedly removes
  a few patterns (at least one from the border) and re-inserts them, keeping results that are not worse. It finds
  very tight layouts, but as a heuristic it cannot prove they are optimal.

## Limitations

- Type information is only as good as the vault's `input`/`output` fields.
- Great spells have per-world signatures; the vault's may differ from yours.
- Packing assumes the only grid rule is "no shared points".

## Credits

Pattern names, signatures and descriptions come from the in-game documentation of
[Hex Casting](https://github.com/FallingColors/HexMod) and its addons, collected in the Hexwoven Vault.
hexpack is a fan project and is not affiliated with the authors of Hex Casting or any addon.

## License

The hexpack source code is released under the [MIT License](LICENSE).
