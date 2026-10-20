---
aliases: ["Entity Purification: Animal"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:get_entity/animal"
signature: "qqqqqdaqaawa"
start_direction: "SOUTH_EAST"
input: "vector"
output: "entity or null"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]", "[[Null]]"]
entry: "[[Entities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Entity Purification: Animal

◇ **Pattern** · [[Hex Casting]] · 📖 [[Entities]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3 69.3,22.0 53.5,49.3 69.3,76.6 85.0,49.3 53.5,49.3 22.0,49.3 37.8,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:409;--dur:3.6s" points="85.0,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3 69.3,22.0 53.5,49.3 69.3,76.6 85.0,49.3 53.5,49.3 22.0,49.3 37.8,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="90.0" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,64.3 87.0,59.8 93.0,56.3" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqdaqaawa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `entity or null` ([[Entity]], [[Null]])
- **ID:** `hexcasting:get_entity/animal`

## Description

Transform the position on the [[The Stack|stack]] into the animal at that location (or **Null** if there isn't one).

## Related patterns (same book page)

- [[Entity Purification]] `vector → entity or null`
- [[Entity Purification Monster]] `vector → entity or null`
- [[Entity Purification Item]] `vector → entity or null`
- [[Entity Purification Player]] `vector → entity or null`
- [[Entity Purification Living]] `vector → entity or null`
- [[Zone Distillation Animal]] `vector, number → list`
- [[Zone Distillation Non-Animal]] `vector, number → list`
- [[Zone Distillation Monster]] `vector, number → list`
- [[Zone Distillation Non-Monster]] `vector, number → list`
- [[Zone Distillation Item]] `vector, number → list`
- [[Zone Distillation Non-Item]] `vector, number → list`
- [[Zone Distillation Player]] `vector, number → list`
- [[Zone Distillation Non-Player]] `vector, number → list`
- [[Zone Distillation Living]] `vector, number → list`
- [[Zone Distillation Non-Living]] `vector, number → list`
- [[Zone Distillation Any]] `vector, number → list`

## Sources

- 📖 **Guidebook:** [[Entities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Entity Purification: Animal](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/entities@hexcasting:get_entity/animal)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityAt` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:get_entity/animal`
