---
aliases: ["Dousing Distillation"]
tags: [hexcasting/pattern, mod/hexpose, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:fluid_raycast"
signature: "wqqaqwede"
start_direction: "EAST"
input: "vector, vector"
output: "vector/null"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]", "[[Null]]"]
entry: "[[Special Raycasts]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dousing Distillation

◇ **Pattern** · [[Hexpose]] · 📖 [[Special Raycasts]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,85.0 59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 76.9,54.7 94.4,85.0 111.8,115.3 94.4,145.5 76.9,115.3 94.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="24.5,85.0 59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 76.9,54.7 94.4,85.0 111.8,115.3 94.4,145.5 76.9,115.3 94.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="85.0" x2="35.6" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.7,85.0 35.6,88.8 35.6,81.2" fill="#fc77be"/><circle cx="24.5" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqaqwede` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `vector/null` ([[Vector]], [[Null]])
- **ID:** `hexpose:fluid_raycast`

```text
before   … │ vector │ vector  ◂ top
after    … │ vector/null      ◂ top
```

## Description

Works similar to [[Archer's Distillation]] but targets only liquids, piercing through all other blocks.

## Related patterns (same book page)

- [[Lilypad Distillation]] `vector, vector → vector/null`
- [[Railgun Exaltation]] `vec, vec, id → vec/null`
- [[Laser Exaltation]] `vec, vec, id → vec/null`

## Sources

- 📖 **Guidebook:** [[Special Raycasts]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpFluidRaycast` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:fluid_raycast`
