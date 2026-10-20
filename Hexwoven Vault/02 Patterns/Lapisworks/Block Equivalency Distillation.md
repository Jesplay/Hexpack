---
aliases: ["Block Equivalency Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Lapisworks"
op_id: "lapisworks:equiv_block"
signature: "qqqqqeqeeeee"
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

# Block Equivalency Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Scrying]] · Lapisworks

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:345;--dur:3.6s" points="71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.7" y1="41.6" x2="67.5" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="64.4,28.9 70.0,32.8 64.9,35.7" fill="#fc77be"/><circle cx="71.7" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqeqeeeee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:equiv_block`

```text
before   … │ vec │ vec  ◂ top
after    … │ bool       ◂ top
```

## Description

Returns whether the two blocks are equivalent/roughly equal (the same kind of block), so wheat seeds equals a full wheat crop. Both blocks must be within my ambit.

## Related patterns (same book page)

- [[Empty Purification]] `vec → bool`
- [[Empty Distillation]] `vec, vec → bool`
- [[Visible Distillation]] `entity, vec → bool`
- [[Block Equality Distillation]] `vec, vec → bool`
- [[Cleaner's Purification]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Scrying]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `EquivBlock` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:equiv_block`
