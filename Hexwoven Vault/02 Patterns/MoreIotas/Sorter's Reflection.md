---
aliases: ["Sorter's Reflection"]
tags: [hexcasting/pattern, mod/moreiotas, family/reflection, topic/data-structures]
kind: "Pattern"
family: "Reflection"
topics: ["Sets, Maps & Special Iotas"]
mod: "MoreIotas"
op_id: "moreiotas:type/item_held"
signature: "edeedqd"
start_direction: "SOUTH_WEST"
input: ""
output: "itemtype"
cost: ""
iotas_in: []
iotas_out: ["[[Item Type]]"]
entry: "[[Types (MoreIotas)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sorter's Reflection

◇ **Pattern** · [[MoreIotas]] · 📖 [[Types (MoreIotas)]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 85.0,78.3 60.0,121.6 35.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="110.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 85.0,78.3 60.0,121.6 35.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="35.0" x2="102.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="96.2,58.8 97.2,46.1 106.8,51.6" fill="#fc77be"/><circle cx="110.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeedqd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `itemtype` ([[Item Type]])
- **ID:** `moreiotas:type/item_held`

```text
before   …             ◂ top
after    … │ itemtype  ◂ top
```

## Description

Adds the type of the item in the caster's offhand to the [[The Stack|stack]] if there is one present, or the type of the casting hand otherwise.

## Related patterns (same book page)

- [[Sorter's Purification]] `itemtypable → itemtype`
- [[Classifier's Purification]] `any → iotatype`
- [[Physician's Purification]] `entity → entitytype`
- [[Entity Distillation Type]] `entitytype, pos → entity | null`
- [[Zone Exaltation Type]] `entitytype, pos, num → [entity]`
- [[Zone Exaltation Not-Type]] `entitytype, pos, num → [entity]`

## Sources

- 📖 **Guidebook:** [[Types (MoreIotas)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sorter's Reflection](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/types@moreiotas:type/item_held)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTypeItemHeld` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:type/item_held`
