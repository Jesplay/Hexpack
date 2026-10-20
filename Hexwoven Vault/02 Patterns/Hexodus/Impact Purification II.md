---
aliases: ["Impact Purification II"]
tags: [hexcasting/pattern, mod/hexodus, family/purification, topic/movement]
kind: "Pattern"
family: "Purification"
topics: ["Movement"]
mod: "Hexodus"
op_id: "hexodus:get_true_strength"
signature: "weeeewedewdw"
start_direction: "NORTH_WEST"
input: "entity"
output: "number"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Gravitation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Impact Purification II

◇ **Pattern** · [[Hexodus]] · 📖 [[Gravitation]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,115.3 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 111.8,85.0 94.4,115.3 76.9,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:454;--dur:3.6s" points="59.4,115.3 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 111.8,85.0 94.4,115.3 76.9,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="115.3" x2="53.8" y2="105.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,98.6 57.2,103.7 50.5,107.5" fill="#fc77be"/><circle cx="59.4" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeewedewdw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexodus:get_true_strength`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Pushes strength of **Gravity** acting on an entity compared to baseline.

## Related patterns (same book page)

- [[Gravitate]] `entity, vector, number → —`
- [[Grasp Purification]] `entity → number`
- [[Falling Purification]] `entity → vector`
- [[Impact Purification]] `entity → number`
- [[Falling Purification II]] `entity → vector`

## Sources

- 📖 **Guidebook:** [[Gravitation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexodus on Modrinth](https://modrinth.com/mod/bN6IKmdE)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetTrueGravity` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexodus:get_true_strength`
