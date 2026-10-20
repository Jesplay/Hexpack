---
aliases: ["Vacant Reflection: Vector Set"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/reflection, topic/data-structures]
kind: "Pattern"
family: "Reflection"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hextra Patterns"
op_id: "hextrapats:empty_set/vec"
signature: "eedqddeeqqqqq"
start_direction: "NORTH_WEST"
input: ""
output: "vec set"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]", "[[Set]]"]
entry: "[[Hash Set Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vacant Reflection: Vector Set

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="37.8,76.6 22.0,49.3 37.8,22.0 69.3,22.0 53.5,49.3 69.3,76.6 37.8,76.6 53.5,49.3 85.0,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:441;--dur:3.9s" points="37.8,76.6 22.0,49.3 37.8,22.0 69.3,22.0 53.5,49.3 69.3,76.6 37.8,76.6 53.5,49.3 85.0,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="76.6" x2="32.7" y2="67.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="29.1,61.6 35.7,66.1 29.7,69.6" fill="#fc77be"/><circle cx="37.8" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eedqddeeqqqqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vec set` ([[Vector]], [[Set]])
- **ID:** `hextrapats:empty_set/vec`

```text
before   …            ◂ top
after    … │ vec set  ◂ top
```

## Description

Push an empty set that can hold vectors to the top of the [[The Stack|stack]].

## Related patterns (same book page)

- [[Vacant Reflection Number Set]] `— → num set`
- [[Vacant Reflection Entity Set]] `— → entity set`
- [[Insertion Gambit]] `set, (num|vec|entity) → set, bool`
- [[Removal Gambit]] `set, (num|vec|entity) → set, bool`
- [[Ordering Purification]] `set → list`
- [[Disordering Purification]] `[num]|[vec]|[entity] → set`
- [[Bucketing Gambit]] `many, num → set`
- [[Spilling Disintegration]] `set → many`

## Sources

- 📖 **Guidebook:** [[Hash Set Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEmptyVecSet` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:empty_set/vec`
