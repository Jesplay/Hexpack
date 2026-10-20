---
aliases: ["Akasha's Gambit"]
tags: [hexcasting/spell, mod/hex-casting, family/gambit, topic/storage]
kind: "Spell"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:akashic/write"
signature: "eeeweeeeede"
start_direction: "EAST"
input: "vector, pattern, any"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Pattern]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Akashic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Akasha's Gambit

✦ **Spell** · [[Hex Casting]] · 📖 [[Akashic Patterns]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="97.4" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="106.2,27.0 97.4,31.3 97.4,22.8" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeweeeeede` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, pattern, any` ([[Vector]], [[Pattern]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexcasting:akashic/write`

```text
before   … │ vector │ pattern │ any  ◂ top
after    …                           ◂ top
```

## Description

Associate the [[Iotas|iota]] with the given pattern in the [[Akashic Libraries|Akashic Library]] with its [[Akashic Libraries|Record]] at the given position. This does have a range limit. Costs about one [[Amethyst|Amethyst Dust]].

## Example

Stores an iota in an [[Akashic Libraries|akashic library]] under a *pattern* used as the key: `library-pos, key-pattern, value`. Read it back with [[Akasha's Distillation]]. A shared, permanent database for [[Spell Circles|spell circles]] and teams.

## Related patterns (same book page)

- [[Akasha's Distillation]] `vector, pattern → any`

## Sources

- 📖 **Guidebook:** [[Akashic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Akasha's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/akashic_patterns@hexcasting:akashic/write)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpAkashicWrite` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:akashic/write`
