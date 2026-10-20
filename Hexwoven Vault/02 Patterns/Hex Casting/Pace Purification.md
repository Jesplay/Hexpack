---
aliases: ["Pace Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:get_entity_velocity"
signature: "wq"
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

# Pace Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/></g><polyline class="hexpat-base" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:131;--dur:1.6s" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="68.3" x2="44.5" y2="68.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="54.5,68.3 44.5,73.1 44.5,63.5" fill="#fc77be"/><circle cx="30.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:get_entity_velocity`

```text
before   … │ entity  ◂ top
after    … │ vector  ◂ top
```

## Description

Transforms an entity on the [[The Stack|stack]] into the direction in which it's moving, with the speed of that movement as that direction's magnitude.

## Related patterns (same book page)

- [[Mind's Reflection]] `— → entity | null`
- [[Compass' Purification]] `entity → vector`
- [[Compass' Purification II]] `entity → vector`
- [[Alidade's Purification]] `entity → vector`
- [[Archer's Distillation]] `vector, vector → vector | null`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Scout's Distillation]] `vector, vector → entity | null`
- [[Reveal]] `any → any`
- [[Stadiometer's Purification]] `entity → num`
- [[Media Reflection (Hex Casting)]] `— → num`

## Sources

- 📖 **Guidebook:** [[Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Pace Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics@hexcasting:get_entity_velocity)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEntityVelocity` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:get_entity_velocity`
