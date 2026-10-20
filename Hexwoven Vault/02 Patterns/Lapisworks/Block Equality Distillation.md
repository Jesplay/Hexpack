---
aliases: ["Block Equality Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Lapisworks"
op_id: "lapisworks:equal_block"
signature: "qwawqwadadwewdwe"
start_direction: "NORTH_WEST"
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

# Block Equality Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Scrying]] · Lapisworks

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="72" viewBox="0 0 170.0 72.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.1" cy="16.1" r="2"/><circle cx="39.1" cy="16.1" r="2"/><circle cx="62.0" cy="16.1" r="2"/><circle cx="85.0" cy="16.1" r="2"/><circle cx="108.0" cy="16.1" r="2"/><circle cx="130.9" cy="16.1" r="2"/><circle cx="153.9" cy="16.1" r="2"/><circle cx="4.6" cy="36.0" r="2"/><circle cx="27.6" cy="36.0" r="2"/><circle cx="50.5" cy="36.0" r="2"/><circle cx="73.5" cy="36.0" r="2"/><circle cx="96.5" cy="36.0" r="2"/><circle cx="119.5" cy="36.0" r="2"/><circle cx="142.4" cy="36.0" r="2"/><circle cx="165.4" cy="36.0" r="2"/><circle cx="16.1" cy="55.9" r="2"/><circle cx="39.1" cy="55.9" r="2"/><circle cx="62.0" cy="55.9" r="2"/><circle cx="85.0" cy="55.9" r="2"/><circle cx="108.0" cy="55.9" r="2"/><circle cx="130.9" cy="55.9" r="2"/><circle cx="153.9" cy="55.9" r="2"/></g><polyline class="hexpat-base" points="73.5,36.0 62.0,16.1 39.1,16.1 16.1,16.1 27.6,36.0 39.1,55.9 62.0,55.9 85.0,55.9 73.5,36.0 96.5,36.0 85.0,16.1 108.0,16.1 130.9,16.1 142.4,36.0 153.9,55.9 130.9,55.9 108.0,55.9 96.5,36.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:391;--dur:4.8s" points="73.5,36.0 62.0,16.1 39.1,16.1 16.1,16.1 27.6,36.0 39.1,55.9 62.0,55.9 85.0,55.9 73.5,36.0 96.5,36.0 85.0,16.1 108.0,16.1 130.9,16.1 142.4,36.0 153.9,55.9 130.9,55.9 108.0,55.9 96.5,36.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.5" y1="36.0" x2="69.8" y2="29.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.2,25.0 72.0,28.3 67.6,30.9" fill="#fc77be"/><circle cx="73.5" cy="36.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwawqwadadwewdwe` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:equal_block`

```text
before   … │ vec │ vec  ◂ top
after    … │ bool       ◂ top
```

## Description

Returns whether the two blocks are *exactly* equal, so wheat seeds do NOT equal a full wheat crop. Blocks must be within my ambit.

## Related patterns (same book page)

- [[Empty Purification]] `vec → bool`
- [[Empty Distillation]] `vec, vec → bool`
- [[Visible Distillation]] `entity, vec → bool`
- [[Block Equivalency Distillation]] `vec, vec → bool`
- [[Cleaner's Purification]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Scrying]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `EqualBlock` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:equal_block`
