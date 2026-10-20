---
aliases: ["Alidade's Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:get_entity_look"
signature: "wa"
start_direction: "EAST"
input: "entity"
output: "vector"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Alidade's Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:150;--dur:1.6s" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="51.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,78.3 51.0,83.8 51.0,72.8" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:get_entity_look`

```text
before   … │ entity  ◂ top
after    … │ vector  ◂ top
```

## Description

Transforms an entity on the [[The Stack|stack]] into the direction it's looking in, as a unit vector.

## Example

[[Mind's Reflection]] → [[Alidade's Purification]] gives the **direction you're facing** as a length-1 vector. Looking straight up gives `(0, 1, 0)`.

## Related patterns (same book page)

- [[Mind's Reflection]] `— → entity | null`
- [[Compass' Purification]] `entity → vector`
- [[Compass' Purification II]] `entity → vector`
- [[Archer's Distillation]] `vector, vector → vector | null`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Scout's Distillation]] `vector, vector → entity | null`
- [[Reveal]] `any → any`
- [[Stadiometer's Purification]] `entity → num`
- [[Pace Purification]] `entity → vector`
- [[Media Reflection (Hex Casting)]] `— → num`

## Sources

- 📖 **Guidebook:** [[Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Alidade's Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics@hexcasting:get_entity_look)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEntityLook` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:get_entity_look`
