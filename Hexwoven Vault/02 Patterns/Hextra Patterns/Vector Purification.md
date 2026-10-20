---
aliases: ["Vector Purification"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:swap_vec_list"
signature: "qqqqqdeee"
start_direction: "WEST"
input: "[num] | vec"
output: "vec | [num]"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[List]]"]
iotas_out: ["[[Number]]", "[[Vector]]", "[[List]]"]
entry: "[[Vector Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vector Purification

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Vector Manipulation]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="72.6" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="63.8,27.0 72.6,22.8 72.6,31.3" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqdeee` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[num] | vec` ([[Number]], [[Vector]], [[List]])
- **Leaves:** `vec | [num]` ([[Number]], [[Vector]], [[List]])
- **ID:** `hextrapats:swap_vec_list`

## Description

Converts a list of 3 numbers to a vector, or a vector into a list of 3 numbers.

## Related patterns (same book page)

- [[Coordinate Purification X]] `vec → num`
- [[Coordinate Distillation X]] `vec, num → vec`
- [[Swizzling Purification]] `vec → vec`

## Sources

- 📖 **Guidebook:** [[Vector Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpVecList` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:swap_vec_list`
