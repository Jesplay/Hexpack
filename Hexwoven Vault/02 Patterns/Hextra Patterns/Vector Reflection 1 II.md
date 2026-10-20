---
aliases: ["Vector Reflection 1 II"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:vec/1"
signature: "qeaqaa"
start_direction: "SOUTH_EAST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Vector Literals]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Vector Reflection 1 II

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Vector Literals]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,30.5 52.3,68.3 95.9,68.3 117.7,106.0 139.5,68.3 117.7,30.5 95.9,68.3 139.5,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="30.5,30.5 52.3,68.3 95.9,68.3 117.7,106.0 139.5,68.3 117.7,30.5 95.9,68.3 139.5,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="30.5" x2="37.5" y2="42.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,51.3 33.3,45.0 41.6,40.2" fill="#fc77be"/><circle cx="30.5" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeaqaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hextrapats:vec/1`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

A head that slightly bends to the left will output a vector where [[Vector Reflection +1|all 3 components are the same]].

First, I draw one of the eight shapes shown on the prior pages. Next, the angles following will modify a running count starting at 0.
- Forward: Add 1
- Left: Add 5
- Right: Add 10
- Sharp Left: Multiply by 2
- Sharp Right: Divide by 2.

The clockwise version of the pattern, on the right of the other pages, will negate the value at the very end. (The left-hand counter-clockwise version keeps the number positive).

Once I finish drawing, a vector's pushed to the top of the [[The Stack|stack]].

## Related patterns (same book page)

- [[Vector Reflection X II]] `— → vector`
- [[Vector Reflection Y II]] `— → vector`
- [[Vector Reflection Z II]] `— → vector`
- [[Example 1]] `— → —`
- [[Example 2]] `— → —`
- [[Example 3]] `— → —`
- [[Example 4]] `— → —`

## Sources

- 📖 **Guidebook:** [[Vector Literals]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:vec/1`
