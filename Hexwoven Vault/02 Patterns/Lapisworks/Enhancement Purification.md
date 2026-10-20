---
aliases: ["Enhancement Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/enchanting]
kind: "Pattern"
family: "Purification"
topics: ["Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:check_attr"
signature: "wwwaqeeqawww"
start_direction: "NORTH_EAST"
input: "entity, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Lapisworks Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Enhancement Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Lapisworks Patterns]] · Lapisworks

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,131.1 100.7,103.8 116.5,76.6 132.2,49.3 148.0,22.0 116.5,22.0 100.7,49.3 69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 53.5,76.6 69.3,103.8 85.0,131.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:409;--dur:3.6s" points="85.0,131.1 100.7,103.8 116.5,76.6 132.2,49.3 148.0,22.0 116.5,22.0 100.7,49.3 69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 53.5,76.6 69.3,103.8 85.0,131.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="131.1" x2="90.0" y2="122.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,116.1 93.0,124.1 87.0,120.6" fill="#fc77be"/><circle cx="85.0" cy="131.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwwaqeeqawww` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, num` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:check_attr`

```text
before   … │ entity │ num  ◂ top
after    …                 ◂ top
```

## Description

Gives me the amount the given entity has been enhanced in the given area (specified by number). The patterns before were sorted such that 0 gives max health, 1 gives attack damage and so on. Negligible [[Media|media]] cost.

## Related patterns (same book page)

- [[Imbue Lapis]] `— → —`
- [[Reclaim Amethyst]] `— → —`
- [[Mold Amel]] `— → —`
- [[Enchant Skin]] `entity, num → —`
- [[Enchant Fists]] `entity, num → —`
- [[Enchant Feet]] `entity, num → —`
- [[Enchant Arms]] `entity, bool → —`

## Sources

- 📖 **Guidebook:** [[Lapisworks Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CheckAttr` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:check_attr`
