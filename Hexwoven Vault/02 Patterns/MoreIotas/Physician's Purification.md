---
aliases: ["Physician's Purification"]
tags: [hexcasting/pattern, mod/moreiotas, family/purification, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:type/entity"
signature: "qawde"
start_direction: "SOUTH_WEST"
input: "entity"
output: "entitytype"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Entity Type]]"]
entry: "[[Types (MoreIotas)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Physician's Purification

◇ **Pattern** · [[MoreIotas]] · 📖 [[Types (MoreIotas)]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,35.0 35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="60.0,35.0 35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="35.0" x2="52.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.2,58.8 47.2,46.1 56.8,51.6" fill="#fc77be"/><circle cx="60.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qawde` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `entitytype` ([[Entity Type]])
- **ID:** `moreiotas:type/entity`

```text
before   … │ entity      ◂ top
after    … │ entitytype  ◂ top
```

## Description

Remove an entity from the [[The Stack|stack]], and adds the type of the entity at that location to the stack.

## Related patterns (same book page)

- [[Sorter's Purification]] `itemtypable → itemtype`
- [[Classifier's Purification]] `any → iotatype`
- [[Sorter's Reflection]] `— → itemtype`
- [[Entity Distillation Type]] `entitytype, pos → entity | null`
- [[Zone Exaltation Type]] `entitytype, pos, num → [entity]`
- [[Zone Exaltation Not-Type]] `entitytype, pos, num → [entity]`

## Sources

- 📖 **Guidebook:** [[Types (MoreIotas)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Physician's Purification](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/types@moreiotas:type/entity)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTypeEntity` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:type/entity`
