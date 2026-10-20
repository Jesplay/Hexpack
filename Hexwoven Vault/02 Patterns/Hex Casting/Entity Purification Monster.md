---
aliases: ["Entity Purification: Monster"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:get_entity/monster"
signature: "qqqqqdaqaawq"
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

# Entity Purification: Monster

◇ **Pattern** · [[Hex Casting]] · 📖 [[Entities]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="90" viewBox="0 0 170.0 90.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.2" cy="20.2" r="2"/><circle cx="49.0" cy="20.2" r="2"/><circle cx="77.8" cy="20.2" r="2"/><circle cx="106.6" cy="20.2" r="2"/><circle cx="135.4" cy="20.2" r="2"/><circle cx="164.2" cy="20.2" r="2"/><circle cx="5.8" cy="45.1" r="2"/><circle cx="34.6" cy="45.1" r="2"/><circle cx="63.4" cy="45.1" r="2"/><circle cx="92.2" cy="45.1" r="2"/><circle cx="121.0" cy="45.1" r="2"/><circle cx="149.8" cy="45.1" r="2"/><circle cx="20.2" cy="70.1" r="2"/><circle cx="49.0" cy="70.1" r="2"/><circle cx="77.8" cy="70.1" r="2"/><circle cx="106.6" cy="70.1" r="2"/><circle cx="135.4" cy="70.1" r="2"/><circle cx="164.2" cy="70.1" r="2"/></g><polyline class="hexpat-base" points="92.2,45.1 106.6,70.1 135.4,70.1 149.8,45.1 135.4,20.2 106.6,20.2 92.2,45.1 77.8,20.2 63.4,45.1 77.8,70.1 92.2,45.1 63.4,45.1 34.6,45.1 20.2,70.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:375;--dur:3.6s" points="92.2,45.1 106.6,70.1 135.4,70.1 149.8,45.1 135.4,20.2 106.6,20.2 92.2,45.1 77.8,20.2 63.4,45.1 77.8,70.1 92.2,45.1 63.4,45.1 34.6,45.1 20.2,70.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.2" y1="45.1" x2="96.8" y2="53.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="100.1,58.8 94.1,54.7 99.6,51.5" fill="#fc77be"/><circle cx="92.2" cy="45.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqdaqaawq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `entity or null` ([[Entity]], [[Null]])
- **ID:** `hexcasting:get_entity/monster`

## Description

Transform the position on the [[The Stack|stack]] into the monster at that location (or **Null** if there isn't one).

## Related patterns (same book page)

- [[Entity Purification]] `vector → entity or null`
- [[Entity Purification Animal]] `vector → entity or null`
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
- 🌐 **Official online docs:** [Entity Purification: Monster](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/entities@hexcasting:get_entity/monster)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityAt` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:get_entity/monster`
