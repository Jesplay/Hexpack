---
aliases: ["Vacant Reflection: Number Set"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/reflection, topic/data-structures]
kind: "Pattern"
family: "Reflection"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hextra Patterns"
op_id: "hextrapats:empty_set/num"
signature: "eedqddeeaqaa"
start_direction: "NORTH_WEST"
input: ""
output: "num set"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]", "[[Set]]"]
entry: "[[Hash Set Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vacant Reflection: Number Set

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 65.7,60.5 85.0,94.0 46.4,94.0 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 65.7,60.5 85.0,94.0 46.4,94.0 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="40.2" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,75.6 43.9,81.1 36.5,85.4" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eedqddeeaqaa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num set` ([[Number]], [[Set]])
- **ID:** `hextrapats:empty_set/num`

```text
before   …            ◂ top
after    … │ num set  ◂ top
```

## Description

Push an empty set that can hold numbers to the top of the [[The Stack|stack]].

## Related patterns (same book page)

- [[Vacant Reflection Vector Set]] `— → vec set`
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
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEmptyNumSet` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:empty_set/num`
