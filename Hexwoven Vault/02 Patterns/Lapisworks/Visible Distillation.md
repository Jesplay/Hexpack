---
aliases: ["Visible Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Lapisworks"
op_id: "lapisworks:visible_dstl"
signature: "edeewadwewdwe"
start_direction: "SOUTH_EAST"
input: "entity, vec"
output: "bool"
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Scrying]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Visible Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Scrying]] · Lapisworks

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="106" viewBox="0 0 170.0 106.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.6" cy="18.6" r="2"/><circle cx="45.2" cy="18.6" r="2"/><circle cx="71.7" cy="18.6" r="2"/><circle cx="98.3" cy="18.6" r="2"/><circle cx="124.8" cy="18.6" r="2"/><circle cx="151.4" cy="18.6" r="2"/><circle cx="5.3" cy="41.6" r="2"/><circle cx="31.9" cy="41.6" r="2"/><circle cx="58.4" cy="41.6" r="2"/><circle cx="85.0" cy="41.6" r="2"/><circle cx="111.6" cy="41.6" r="2"/><circle cx="138.1" cy="41.6" r="2"/><circle cx="164.7" cy="41.6" r="2"/><circle cx="18.6" cy="64.6" r="2"/><circle cx="45.2" cy="64.6" r="2"/><circle cx="71.7" cy="64.6" r="2"/><circle cx="98.3" cy="64.6" r="2"/><circle cx="124.8" cy="64.6" r="2"/><circle cx="151.4" cy="64.6" r="2"/><circle cx="5.3" cy="87.6" r="2"/><circle cx="31.9" cy="87.6" r="2"/><circle cx="58.4" cy="87.6" r="2"/><circle cx="85.0" cy="87.6" r="2"/><circle cx="111.6" cy="87.6" r="2"/><circle cx="138.1" cy="87.6" r="2"/><circle cx="164.7" cy="87.6" r="2"/></g><polyline class="hexpat-base" points="31.9,41.6 45.2,64.6 31.9,87.6 18.6,64.6 31.9,41.6 58.4,41.6 85.0,41.6 71.7,18.6 98.3,18.6 124.8,18.6 138.1,41.6 151.4,64.6 124.8,64.6 98.3,64.6 85.0,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:372;--dur:3.9s" points="31.9,41.6 45.2,64.6 31.9,87.6 18.6,64.6 31.9,41.6 58.4,41.6 85.0,41.6 71.7,18.6 98.3,18.6 124.8,18.6 138.1,41.6 151.4,64.6 124.8,64.6 98.3,64.6 85.0,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.9" y1="41.6" x2="36.1" y2="49.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.2,54.2 33.6,50.4 38.7,47.5" fill="#fc77be"/><circle cx="31.9" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeewadwewdwe` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, vec` ([[Vector]], [[Entity]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:visible_dstl`

```text
before   … │ entity │ vec  ◂ top
after    … │ bool          ◂ top
```

## Description

Negligible cost. Pushes whether or not the specified entity (within ambit) is able to see the specified block (also within ambit).

## Related patterns (same book page)

- [[Empty Purification]] `vec → bool`
- [[Empty Distillation]] `vec, vec → bool`
- [[Block Equivalency Distillation]] `vec, vec → bool`
- [[Block Equality Distillation]] `vec, vec → bool`
- [[Cleaner's Purification]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Scrying]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `VisibleDstl` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:visible_dstl`
