---
aliases: ["Laser Exaltation"]
tags: [hexcasting/pattern, mod/hexpose, family/exaltation, topic/sensing]
kind: "Pattern"
family: "Exaltation"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:piercing_surface_raycast"
signature: "weeaaeqeaae"
start_direction: "EAST"
input: "vec, vec, id"
output: "vec/null"
cost: ""
iotas_in: ["[[Vector]]", "[[Identifier]]"]
iotas_out: ["[[Vector]]", "[[Null]]"]
entry: "[[Special Raycasts]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Laser Exaltation

◇ **Pattern** · [[Hexpose]] · 📖 [[Special Raycasts]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,85.0 59.4,85.0 94.4,85.0 111.8,115.3 94.4,145.5 129.3,145.5 111.8,115.3 129.3,85.0 111.8,54.7 129.3,24.5 94.4,24.5 111.8,54.7 94.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="24.5,85.0 59.4,85.0 94.4,85.0 111.8,115.3 94.4,145.5 129.3,145.5 111.8,115.3 129.3,85.0 111.8,54.7 129.3,24.5 94.4,24.5 111.8,54.7 94.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="85.0" x2="35.6" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.7,85.0 35.6,88.8 35.6,81.2" fill="#fc77be"/><circle cx="24.5" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeaaeqeaae` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec, id` ([[Vector]], [[Identifier]])
- **Leaves:** `vec/null` ([[Vector]], [[Null]])
- **ID:** `hexpose:piercing_surface_raycast`

```text
before   … │ vec │ vec │ id  ◂ top
after    … │ vec/null        ◂ top
```

## Description

Works similar to [[Architect's Distillation]] but accepts an additional identifier. It will pierce all blocks until it hits the block specified by the identifier.

## Related patterns (same book page)

- [[Dousing Distillation]] `vector, vector → vector/null`
- [[Lilypad Distillation]] `vector, vector → vector/null`
- [[Railgun Exaltation]] `vec, vec, id → vec/null`

## Sources

- 📖 **Guidebook:** [[Special Raycasts]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPiercingSurfaceRaycast` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:piercing_surface_raycast`
