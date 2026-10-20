---
aliases: ["Zone Exaltation: Type"]
tags: [hexcasting/pattern, mod/moreiotas, family/exaltation, topic/data-structures]
kind: "Pattern"
family: "Exaltation"
topics: ["Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:zone_entity/type"
signature: "waweeeeewaw"
start_direction: "SOUTH_EAST"
input: "entitytype, pos, num"
output: "[entity]"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity Type]]"]
iotas_out: ["[[Entity]]", "[[List]]"]
entry: "[[Types (MoreIotas)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Zone Exaltation: Type

◇ **Pattern** · [[MoreIotas]] · 📖 [[Types (MoreIotas)]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,24.3 41.6,54.3 59.0,84.4 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 59.0,24.3 41.6,54.3 24.3,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:416;--dur:3.4s" points="24.3,24.3 41.6,54.3 59.0,84.4 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 59.0,24.3 41.6,54.3 24.3,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="24.3" x2="29.8" y2="33.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="33.8,40.8 26.5,35.8 33.1,32.0" fill="#fc77be"/><circle cx="24.3" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waweeeeewaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entitytype, pos, num` ([[Number]], [[Vector]], [[Entity Type]])
- **Leaves:** `[entity]` ([[Entity]], [[List]])
- **ID:** `moreiotas:zone_entity/type`

```text
before   … │ entitytype │ pos │ num  ◂ top
after    … │ [entity]                ◂ top
```

## Description

Remove an entity type, a position, and a radius from the [[The Stack|stack]], and returns a list of all entities of that type within that radius of that position.

## Related patterns (same book page)

- [[Sorter's Purification]] `itemtypable → itemtype`
- [[Classifier's Purification]] `any → iotatype`
- [[Physician's Purification]] `entity → entitytype`
- [[Sorter's Reflection]] `— → itemtype`
- [[Entity Distillation Type]] `entitytype, pos → entity | null`
- [[Zone Exaltation Not-Type]] `entitytype, pos, num → [entity]`

## Sources

- 📖 **Guidebook:** [[Types (MoreIotas)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Zone Exaltation: Type](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/types@moreiotas:zone_entity/type)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntitiesByDyn` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:zone_entity/type`
