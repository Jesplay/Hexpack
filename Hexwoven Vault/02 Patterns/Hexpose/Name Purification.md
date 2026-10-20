---
aliases: ["Name Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing, topic/text, hexcasting/not-in-book]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "Text, Strings & Chat"]
mod: "Hexpose"
op_id: "hexpose:entity_name"
signature: "edeweedw"
start_direction: "SOUTH_WEST"
input: "entity"
output: "display"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Undocumented Hexpose Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: false
---

# Name Purification

> [!caution] Not in the guidebook
> This pattern exists in the mod's code, but the in-game book doesn't describe it. The description below comes from the official online docs.

◇ **Pattern** · [[Hexpose]] · 📖 [[Undocumented Hexpose Patterns]] · Not in the Book

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 65.7,60.5 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="85.0,27.0 65.7,60.5 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="78.8" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,45.4 75.1,35.6 82.5,39.9" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeweedw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `display` (—)
- **ID:** `hexpose:entity_name`

```text
before   … │ entity   ◂ top
after    … │ display  ◂ top
```

## Description

Pushes a display [[Iotas|iota]] with the formatted name of the entity.

\*(Description from the official online docs; the in-game book doesn't include it.)\*

## Sources

- 📖 **Guidebook:** [[Undocumented Hexpose Patterns]] *(not in the in-game book)*
- 🌐 **Official online docs:** [Name Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/entities@hexpose:entity_name)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:entity_name`
