---
aliases: ["Swizzling Purification"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:vec/swizzle"
signature: "eeeeqaawddeadawdq"
start_direction: "WEST"
input: "vec"
output: "vec"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Vector Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Swizzling Purification

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Vector Manipulation]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="72" viewBox="0 0 170.0 72.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.1" r="2"/><circle cx="27.6" cy="16.1" r="2"/><circle cx="50.5" cy="16.1" r="2"/><circle cx="73.5" cy="16.1" r="2"/><circle cx="96.5" cy="16.1" r="2"/><circle cx="119.5" cy="16.1" r="2"/><circle cx="142.4" cy="16.1" r="2"/><circle cx="165.4" cy="16.1" r="2"/><circle cx="16.1" cy="36.0" r="2"/><circle cx="39.1" cy="36.0" r="2"/><circle cx="62.0" cy="36.0" r="2"/><circle cx="85.0" cy="36.0" r="2"/><circle cx="108.0" cy="36.0" r="2"/><circle cx="130.9" cy="36.0" r="2"/><circle cx="153.9" cy="36.0" r="2"/><circle cx="4.6" cy="55.9" r="2"/><circle cx="27.6" cy="55.9" r="2"/><circle cx="50.5" cy="55.9" r="2"/><circle cx="73.5" cy="55.9" r="2"/><circle cx="96.5" cy="55.9" r="2"/><circle cx="119.5" cy="55.9" r="2"/><circle cx="142.4" cy="55.9" r="2"/><circle cx="165.4" cy="55.9" r="2"/></g><polyline class="hexpat-base" points="50.5,55.9 27.6,55.9 16.1,36.0 27.6,16.1 50.5,16.1 62.0,36.0 85.0,36.0 73.5,16.1 62.0,36.0 50.5,55.9 39.1,36.0 62.0,36.0 73.5,55.9 85.0,36.0 96.5,55.9 108.0,36.0 119.5,16.1 130.9,36.0 153.9,36.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:414;--dur:5.0s" points="50.5,55.9 27.6,55.9 16.1,36.0 27.6,16.1 50.5,16.1 62.0,36.0 85.0,36.0 73.5,16.1 62.0,36.0 50.5,55.9 39.1,36.0 62.0,36.0 73.5,55.9 85.0,36.0 96.5,55.9 108.0,36.0 119.5,16.1 130.9,36.0 153.9,36.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.5" y1="55.9" x2="43.2" y2="55.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.9,55.9 43.2,53.3 43.2,58.4" fill="#fc77be"/><circle cx="50.5" cy="55.9" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeqaawddeadawdq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `vec` ([[Vector]])
- **ID:** `hextrapats:vec/swizzle`

```text
before   … │ vec  ◂ top
after    … │ vec  ◂ top
```

## Description

A family of patterns that "swizzles" a vector's components based on the 3 sections of its tail.

A leftward point represents X, a straight line is Y, and a rightward point is Z.

The first pattern encodes ZXY;&lt;br&gt;(1, 2, 3) becomes (3, 1, 2)

The second pattern encodes YXX;&lt;br&gt;(1, 2, 3) becomes (2, 1, 1)

The third pattern encodes YYY;&lt;br&gt;(1, 2, 3) becomes (2, 2, 2)

The fourth pattern encodes ZZX;&lt;br&gt;(1, 2, 3) becomes (3, 3, 1)

## Related patterns (same book page)

- [[Coordinate Purification X]] `vec → num`
- [[Coordinate Distillation X]] `vec, num → vec`
- [[Vector Purification]] `[num] | vec → vec | [num]`

## Sources

- 📖 **Guidebook:** [[Vector Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:vec/swizzle`
