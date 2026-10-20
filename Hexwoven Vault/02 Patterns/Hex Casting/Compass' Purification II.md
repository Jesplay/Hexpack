---
aliases: ["Compass' Purification II"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:entity_pos/foot"
signature: "dd"
start_direction: "NORTH_EAST"
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

# Compass' Purification II

◇ **Pattern** · [[Hex Casting]] · 📖 [[Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="148" viewBox="0 0 156.9 148.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.1" cy="45.8" r="2"/><circle cx="78.5" cy="45.8" r="2"/><circle cx="143.8" cy="45.8" r="2"/><circle cx="45.8" cy="102.4" r="2"/><circle cx="111.2" cy="102.4" r="2"/></g><polyline class="hexpat-base" points="45.8,102.4 78.5,45.8 111.2,102.4 45.8,102.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:196;--dur:1.6s" points="45.8,102.4 78.5,45.8 111.2,102.4 45.8,102.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.8" y1="102.4" x2="56.2" y2="84.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="63.8,71.3 62.5,87.9 50.0,80.7" fill="#fc77be"/><circle cx="45.8" cy="102.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:entity_pos/foot`

```text
before   … │ entity  ◂ top
after    … │ vector  ◂ top
```

## Description

Transforms an entity on the [[The Stack|stack]] into the position it is standing. I should probably use this on other entities.

## Related patterns (same book page)

- [[Mind's Reflection]] `— → entity | null`
- [[Compass' Purification]] `entity → vector`
- [[Alidade's Purification]] `entity → vector`
- [[Archer's Distillation]] `vector, vector → vector | null`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Scout's Distillation]] `vector, vector → entity | null`
- [[Reveal]] `any → any`
- [[Stadiometer's Purification]] `entity → num`
- [[Pace Purification]] `entity → vector`
- [[Media Reflection (Hex Casting)]] `— → num`

## Sources

- 📖 **Guidebook:** [[Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Compass' Purification II](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics@hexcasting:entity_pos/foot)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEntityPos` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:entity_pos/foot`
