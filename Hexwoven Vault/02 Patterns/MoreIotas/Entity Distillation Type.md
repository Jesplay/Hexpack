---
aliases: ["Entity Distillation: Type"]
tags: [hexcasting/pattern, mod/moreiotas, family/distillation, topic/data-structures]
kind: "Pattern"
family: "Distillation"
topics: ["Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:get_entity/type"
signature: "dadqqqqqdad"
start_direction: "NORTH_EAST"
input: "entitytype, pos"
output: "entity | null"
cost: ""
iotas_in: ["[[Vector]]", "[[Entity Type]]"]
iotas_out: ["[[Entity]]", "[[Null]]"]
entry: "[[Types (MoreIotas)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Entity Distillation: Type

◇ **Pattern** · [[MoreIotas]] · 📖 [[Types (MoreIotas)]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,84.4 41.6,54.3 59.0,84.4 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 59.0,24.3 41.6,54.3 24.3,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:416;--dur:3.4s" points="24.3,84.4 41.6,54.3 59.0,84.4 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 59.0,24.3 41.6,54.3 24.3,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="84.4" x2="29.8" y2="74.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="33.8,67.9 33.1,76.7 26.5,72.9" fill="#fc77be"/><circle cx="24.3" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dadqqqqqdad` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entitytype, pos` ([[Vector]], [[Entity Type]])
- **Leaves:** `entity | null` ([[Entity]], [[Null]])
- **ID:** `moreiotas:get_entity/type`

## Description

Remove an entity type and a position from the [[The Stack|stack]], and returns an entity of that type at that position if one exists.

## Related patterns (same book page)

- [[Sorter's Purification]] `itemtypable → itemtype`
- [[Classifier's Purification]] `any → iotatype`
- [[Physician's Purification]] `entity → entitytype`
- [[Sorter's Reflection]] `— → itemtype`
- [[Zone Exaltation Type]] `entitytype, pos, num → [entity]`
- [[Zone Exaltation Not-Type]] `entitytype, pos, num → [entity]`

## Sources

- 📖 **Guidebook:** [[Types (MoreIotas)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Entity Distillation: Type](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/types@moreiotas:get_entity/type)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityAtDyn` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:get_entity/type`
