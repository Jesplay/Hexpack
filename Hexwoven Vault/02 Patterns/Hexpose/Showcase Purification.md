---
aliases: ["Showcase Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:item_frame_rotation"
signature: "ewdwewdea"
start_direction: "NORTH_EAST"
input: "entity"
output: "number"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Miscellaneous (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Showcase Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Miscellaneous (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 65.7,27.0 104.3,27.0 143.0,27.0 123.6,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 85.0,60.5 65.7,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="46.4,60.5 65.7,27.0 104.3,27.0 143.0,27.0 123.6,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 85.0,60.5 65.7,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="52.5" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.0,42.1 56.2,51.9 48.9,47.7" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewdwewdea` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:item_frame_rotation`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Pushes the rotation of an **Item Frame** as an integer 0 to 7.

## Related patterns (same book page)

- [[Feline Purification]] `entity → identifier`
- [[Anger Purification]] `entity → number`
- [[Artistic Purification]] `entity → identifier`

## Sources

- 📖 **Guidebook:** [[Miscellaneous (Hexpose)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `HexposeActions$init$77` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:item_frame_rotation`
