---
aliases: ["Tachygrapher's Gambit"]
tags: [hexcasting/spell, mod/ioticblocks, family/gambit, topic/storage, topic/blocks]
kind: "Spell"
family: "Gambit"
topics: ["Reading & Writing", "World: Blocks, Terrain & Weather"]
mod: "IoticBlocks"
op_id: "ioticblocks:write/index"
signature: "deeeeeqawqwaw"
start_direction: "EAST"
input: "entity | vec, num, any"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[ReadingWriting Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Tachygrapher's Gambit

✦ **Spell** · [[IoticBlocks]] · 📖 [[ReadingWriting Blocks]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 104.3,94.0 85.0,127.4 46.4,127.4 27.0,94.0 46.4,60.5 85.0,60.5 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 65.7,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="65.7,94.0 104.3,94.0 85.0,127.4 46.4,127.4 27.0,94.0 46.4,60.5 85.0,60.5 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 65.7,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="78.0" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.9,94.0 78.0,98.2 78.0,89.7" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeeeqawqwaw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity | vec, num, any` ([[Number]], [[Vector]], [[Entity]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `ioticblocks:write/index`

## Description

Read a list [[Iotas|iota]] out of an entity or block, then set the element at the specified index of that list to the given iota, then write the resulting list back to the entity or block.

## Related patterns (same book page)

- [[Tachygrapher's Distillation]] `entity | vec, num → any`

## Sources

- 📖 **Guidebook:** [[ReadingWriting Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Tachygrapher's Gambit](https://ioticblocks.hexxy.media/v/1.0.2+1.20.1/1.1/en_us/#patterns/readwrite_blocks@ioticblocks:write/index)
- 🧩 **Mod page:** [IoticBlocks on Modrinth](https://modrinth.com/mod/u3PPsXzt)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpWriteIndex` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `ioticblocks:write/index`
