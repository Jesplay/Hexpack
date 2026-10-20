---
aliases: ["Alter Ballast"]
tags: [hexcasting/spell, mod/hexodus, family/spell, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Movement"]
mod: "Hexodus"
op_id: "hexodus:alter_ballast"
signature: "wwawwawwqaqwedwwwq"
start_direction: "NORTH_EAST"
input: "vector, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Ballasts]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Alter Ballast

✦ **Spell** · [[Hexodus]] · 📖 [[Ballasts]] · Items

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="115" viewBox="0 0 170.0 115.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.8" cy="20.2" r="2"/><circle cx="34.6" cy="20.2" r="2"/><circle cx="63.4" cy="20.2" r="2"/><circle cx="92.2" cy="20.2" r="2"/><circle cx="121.0" cy="20.2" r="2"/><circle cx="149.8" cy="20.2" r="2"/><circle cx="20.2" cy="45.1" r="2"/><circle cx="49.0" cy="45.1" r="2"/><circle cx="77.8" cy="45.1" r="2"/><circle cx="106.6" cy="45.1" r="2"/><circle cx="135.4" cy="45.1" r="2"/><circle cx="164.2" cy="45.1" r="2"/><circle cx="5.8" cy="70.1" r="2"/><circle cx="34.6" cy="70.1" r="2"/><circle cx="63.4" cy="70.1" r="2"/><circle cx="92.2" cy="70.1" r="2"/><circle cx="121.0" cy="70.1" r="2"/><circle cx="149.8" cy="70.1" r="2"/><circle cx="20.2" cy="95.0" r="2"/><circle cx="49.0" cy="95.0" r="2"/><circle cx="77.8" cy="95.0" r="2"/><circle cx="106.6" cy="95.0" r="2"/><circle cx="135.4" cy="95.0" r="2"/><circle cx="164.2" cy="95.0" r="2"/></g><polyline class="hexpat-base" points="77.8,95.0 92.2,70.1 106.6,45.1 121.0,20.2 92.2,20.2 63.4,20.2 34.6,20.2 49.0,45.1 63.4,70.1 77.8,95.0 106.6,95.0 92.2,70.1 63.4,70.1 34.6,70.1 20.2,45.1 49.0,45.1 77.8,45.1 106.6,45.1 135.4,45.1 149.8,20.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:547;--dur:5.3s" points="77.8,95.0 92.2,70.1 106.6,45.1 121.0,20.2 92.2,20.2 63.4,20.2 34.6,20.2 49.0,45.1 63.4,70.1 77.8,95.0 106.6,95.0 92.2,70.1 63.4,70.1 34.6,70.1 20.2,45.1 49.0,45.1 77.8,45.1 106.6,45.1 135.4,45.1 149.8,20.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="77.8" y1="95.0" x2="82.4" y2="87.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="85.7,81.3 85.2,88.6 79.7,85.5" fill="#fc77be"/><circle cx="77.8" cy="95.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwawwawwqaqwedwwwq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexodus:alter_ballast`

```text
before   … │ vector │ number  ◂ top
after    …                    ◂ top
```

## Description

Commands a **Ballast** on my person to fall in a given direction with a given strength strength. Free.

## Sources

- 📖 **Guidebook:** [[Ballasts]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexodus on Modrinth](https://modrinth.com/mod/bN6IKmdE)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpAlterBallast` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexodus:alter_ballast`
