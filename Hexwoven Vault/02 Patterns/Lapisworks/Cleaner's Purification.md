---
aliases: ["Cleaner's Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Lapisworks"
op_id: "lapisworks:is_conjureable"
signature: "qqaaeqqqq"
start_direction: "NORTH_EAST"
input: "vec, vec"
output: "bool"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Scrying]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cleaner's Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Scrying]] · Lapisworks

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,145.5 76.9,115.3 59.4,85.0 24.5,85.0 41.9,115.3 59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 41.9,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="59.4,145.5 76.9,115.3 59.4,85.0 24.5,85.0 41.9,115.3 59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 41.9,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="145.5" x2="65.0" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,128.9 68.3,137.8 61.7,133.9" fill="#fc77be"/><circle cx="59.4" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqaaeqqqq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:is_conjureable`

```text
before   … │ vec │ vec  ◂ top
after    … │ bool       ◂ top
```

## Description

Returns whether or not the block at the given coordinates within range is cheap to break with **Break Block** or not, for negligible cost.

## Related patterns (same book page)

- [[Empty Purification]] `vec → bool`
- [[Empty Distillation]] `vec, vec → bool`
- [[Visible Distillation]] `entity, vec → bool`
- [[Block Equivalency Distillation]] `vec, vec → bool`
- [[Block Equality Distillation]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Scrying]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `IsConjureable` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:is_conjureable`
