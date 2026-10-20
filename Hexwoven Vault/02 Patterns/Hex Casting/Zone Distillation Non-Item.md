---
aliases: ["Zone Distillation: Non-Item"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:zone_entity/not_item"
signature: "eeeeewaqaaww"
start_direction: "NORTH_EAST"
input: "vector, number"
output: "list"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[List]]"]
entry: "[[Entities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Zone Distillation: Non-Item

◇ **Pattern** · [[Hex Casting]] · 📖 [[Entities]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6 85.0,18.6 71.7,41.6 85.0,64.6 98.3,41.6 71.7,41.6 45.2,41.6 18.6,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:345;--dur:3.6s" points="98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6 85.0,18.6 71.7,41.6 85.0,64.6 98.3,41.6 71.7,41.6 45.2,41.6 18.6,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="98.3" y1="41.6" x2="102.5" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.6,28.9 105.1,35.7 100.0,32.8" fill="#fc77be"/><circle cx="98.3" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeewaqaaww` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:zone_entity/not_item`

```text
before   … │ vector │ number  ◂ top
after    … │ list             ◂ top
```

## Description

Take a position and maximum distance on the [[The Stack|stack]], and combine them into a list of non-dropped-item entities near the position.

## Related patterns (same book page)

- [[Entity Purification]] `vector → entity or null`
- [[Entity Purification Animal]] `vector → entity or null`
- [[Entity Purification Monster]] `vector → entity or null`
- [[Entity Purification Item]] `vector → entity or null`
- [[Entity Purification Player]] `vector → entity or null`
- [[Entity Purification Living]] `vector → entity or null`
- [[Zone Distillation Animal]] `vector, number → list`
- [[Zone Distillation Non-Animal]] `vector, number → list`
- [[Zone Distillation Monster]] `vector, number → list`
- [[Zone Distillation Non-Monster]] `vector, number → list`
- [[Zone Distillation Item]] `vector, number → list`
- [[Zone Distillation Player]] `vector, number → list`
- [[Zone Distillation Non-Player]] `vector, number → list`
- [[Zone Distillation Living]] `vector, number → list`
- [[Zone Distillation Non-Living]] `vector, number → list`
- [[Zone Distillation Any]] `vector, number → list`

## Sources

- 📖 **Guidebook:** [[Entities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Zone Distillation: Non-Item](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/entities@hexcasting:zone_entity/not_item)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntitiesBy` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:zone_entity/not_item`
