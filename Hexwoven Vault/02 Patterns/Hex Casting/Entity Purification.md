---
aliases: ["Entity Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:get_entity"
signature: "qqqqqdaqa"
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

# Entity Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Entities]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="71.9" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,78.9 68.2,73.3 75.5,69.1" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqdaqa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `entity or null` ([[Entity]], [[Null]])
- **ID:** `hexcasting:get_entity`

## Description

Transform the position on the [[The Stack|stack]] into the entity at that location (or **Null** if there isn't one).

## Example

Takes a position and gives the entity at that spot (or [[Null]]). The filtered variants, like [[Entity Purification Item]] for dropped items, are usually more practical: they ignore everything else that might be standing there.

## Related patterns (same book page)

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
- [[Zone Distillation Non-Item]] `vector, number → list`
- [[Zone Distillation Player]] `vector, number → list`
- [[Zone Distillation Non-Player]] `vector, number → list`
- [[Zone Distillation Living]] `vector, number → list`
- [[Zone Distillation Non-Living]] `vector, number → list`
- [[Zone Distillation Any]] `vector, number → list`

## Sources

- 📖 **Guidebook:** [[Entities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Entity Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/entities@hexcasting:get_entity)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityAt` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:get_entity`
