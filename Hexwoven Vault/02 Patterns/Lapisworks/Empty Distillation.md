---
aliases: ["Empty Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Lapisworks"
op_id: "lapisworks:empty_dstl"
signature: "dwewdwedwwwadwewdwedw"
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

# Empty Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Scrying]] · Lapisworks

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="63" viewBox="0 0 170.0 63.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.2" cy="14.2" r="2"/><circle cx="34.4" cy="14.2" r="2"/><circle cx="54.6" cy="14.2" r="2"/><circle cx="74.9" cy="14.2" r="2"/><circle cx="95.1" cy="14.2" r="2"/><circle cx="115.4" cy="14.2" r="2"/><circle cx="135.6" cy="14.2" r="2"/><circle cx="155.8" cy="14.2" r="2"/><circle cx="4.0" cy="31.7" r="2"/><circle cx="24.3" cy="31.7" r="2"/><circle cx="44.5" cy="31.7" r="2"/><circle cx="64.8" cy="31.7" r="2"/><circle cx="85.0" cy="31.7" r="2"/><circle cx="105.2" cy="31.7" r="2"/><circle cx="125.5" cy="31.7" r="2"/><circle cx="145.7" cy="31.7" r="2"/><circle cx="166.0" cy="31.7" r="2"/><circle cx="14.2" cy="49.2" r="2"/><circle cx="34.4" cy="49.2" r="2"/><circle cx="54.6" cy="49.2" r="2"/><circle cx="74.9" cy="49.2" r="2"/><circle cx="95.1" cy="49.2" r="2"/><circle cx="115.4" cy="49.2" r="2"/><circle cx="135.6" cy="49.2" r="2"/><circle cx="155.8" cy="49.2" r="2"/></g><polyline class="hexpat-base" points="24.3,31.7 14.2,14.2 34.4,14.2 54.6,14.2 64.8,31.7 74.9,49.2 54.6,49.2 34.4,49.2 24.3,31.7 44.5,31.7 64.8,31.7 85.0,31.7 105.2,31.7 95.1,14.2 115.4,14.2 135.6,14.2 145.7,31.7 155.8,49.2 135.6,49.2 115.4,49.2 105.2,31.7 125.5,31.7 145.7,31.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:445;--dur:6.0s" points="24.3,31.7 14.2,14.2 34.4,14.2 54.6,14.2 64.8,31.7 74.9,49.2 54.6,49.2 34.4,49.2 24.3,31.7 44.5,31.7 64.8,31.7 85.0,31.7 105.2,31.7 95.1,14.2 115.4,14.2 135.6,14.2 145.7,31.7 155.8,49.2 135.6,49.2 115.4,49.2 105.2,31.7 125.5,31.7 145.7,31.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="31.7" x2="21.0" y2="26.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="18.7,22.1 23.0,25.0 19.1,27.2" fill="#fc77be"/><circle cx="24.3" cy="31.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwewdwedwwwadwewdwedw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:empty_dstl`

```text
before   … │ vec │ vec  ◂ top
after    … │ bool       ◂ top
```

## Description

Negligible cost. Answers the question, "are there no opaque blocks between these points?". Both vectors must be in my ambit.

## Related patterns (same book page)

- [[Empty Purification]] `vec → bool`
- [[Visible Distillation]] `entity, vec → bool`
- [[Block Equivalency Distillation]] `vec, vec → bool`
- [[Block Equality Distillation]] `vec, vec → bool`
- [[Cleaner's Purification]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Scrying]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `EmptyDstl` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:empty_dstl`
