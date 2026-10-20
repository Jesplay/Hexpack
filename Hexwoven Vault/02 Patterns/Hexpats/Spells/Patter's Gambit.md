---
aliases: ["Patter's Gambit"]
tags: [hexcasting/spell, mod/hexpats, family/gambit, topic/misc]
kind: "Spell"
family: "Gambit"
topics: ["Miscellaneous"]
mod: "Hexpats"
op_id: "hexpats:headpat"
signature: "weweeeee"
start_direction: "EAST"
input: "entity"
output: ""
cost: "Costs nothing if the target isn't on fire, otherwise costs an Amethyst Dust."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Comfort Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Patter's Gambit

✦ **Spell** · [[Hexpats]] · 📖 [[Comfort Patterns]] · Patterns

**Topics:** [[Miscellaneous (Topic)|Miscellaneous]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,27.0 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 123.6,127.4 85.0,127.4 65.7,94.0 85.0,60.5 123.6,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="27.0,27.0 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 123.6,127.4 85.0,127.4 65.7,94.0 85.0,60.5 123.6,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="27.0" x2="39.4" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,27.0 39.4,31.3 39.4,22.8" fill="#fc77be"/><circle cx="27.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weweeeee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs nothing if the target isn't on fire, otherwise costs an Amethyst Dust.
- **ID:** `hexpats:headpat`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

This pattern appears to conjure a projection of the casters hand about the head of some entity. Costs nothing if the target isn't on fire, otherwise costs an Amethyst Dust.

## Related patterns (same book page)

- [[Patter's Purification]] `entity → bool`

## Sources

- 📖 **Guidebook:** [[Comfort Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpats on Modrinth](https://modrinth.com/mod/dmOz7d0b)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `Headpat` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexpats:headpat`
