---
aliases: ["Empty Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Lapisworks"
op_id: "lapisworks:empty_prfn"
signature: "qwawqwaqwweqqqq"
start_direction: "NORTH_WEST"
input: "vec"
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

# Empty Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Scrying]] · Lapisworks

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="77" viewBox="0 0 170.0 77.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.9" cy="17.2" r="2"/><circle cx="29.6" cy="17.2" r="2"/><circle cx="54.2" cy="17.2" r="2"/><circle cx="78.8" cy="17.2" r="2"/><circle cx="103.5" cy="17.2" r="2"/><circle cx="128.1" cy="17.2" r="2"/><circle cx="152.8" cy="17.2" r="2"/><circle cx="17.2" cy="38.6" r="2"/><circle cx="41.9" cy="38.6" r="2"/><circle cx="66.5" cy="38.6" r="2"/><circle cx="91.2" cy="38.6" r="2"/><circle cx="115.8" cy="38.6" r="2"/><circle cx="140.4" cy="38.6" r="2"/><circle cx="165.1" cy="38.6" r="2"/><circle cx="4.9" cy="59.9" r="2"/><circle cx="29.6" cy="59.9" r="2"/><circle cx="54.2" cy="59.9" r="2"/><circle cx="78.8" cy="59.9" r="2"/><circle cx="103.5" cy="59.9" r="2"/><circle cx="128.1" cy="59.9" r="2"/><circle cx="152.8" cy="59.9" r="2"/></g><polyline class="hexpat-base" points="140.4,38.6 128.1,17.2 103.5,17.2 78.8,17.2 91.2,38.6 103.5,59.9 128.1,59.9 152.8,59.9 140.4,38.6 115.8,38.6 91.2,38.6 66.5,38.6 54.2,17.2 29.6,17.2 17.2,38.6 29.6,59.9 54.2,59.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:394;--dur:4.5s" points="140.4,38.6 128.1,17.2 103.5,17.2 78.8,17.2 91.2,38.6 103.5,59.9 128.1,59.9 152.8,59.9 140.4,38.6 115.8,38.6 91.2,38.6 66.5,38.6 54.2,17.2 29.6,17.2 17.2,38.6 29.6,59.9 54.2,59.9" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="140.4" y1="38.6" x2="136.5" y2="31.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="133.7,26.8 138.8,30.4 134.1,33.1" fill="#fc77be"/><circle cx="140.4" cy="38.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwawqwaqwweqqqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:empty_prfn`

```text
before   … │ vec   ◂ top
after    … │ bool  ◂ top
```

## Description

Answers the question, "is there only air there?" for negligible [[Media|media]] cost. Vector must be in my ambit.

## Related patterns (same book page)

- [[Empty Distillation]] `vec, vec → bool`
- [[Visible Distillation]] `entity, vec → bool`
- [[Block Equivalency Distillation]] `vec, vec → bool`
- [[Block Equality Distillation]] `vec, vec → bool`
- [[Cleaner's Purification]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Scrying]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `EmptyPrfn` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:empty_prfn`
