---
aliases: ["Factorial Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hexal"
op_id: "hexal:factorial"
signature: "wawdedwaw"
start_direction: "SOUTH_EAST"
input: "num"
output: "num"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hexal Maths]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Factorial Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Hexal Maths]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,30.5 52.3,68.3 74.1,106.0 95.9,68.3 117.7,30.5 139.5,68.3 117.7,106.0 95.9,68.3 74.1,30.5 52.3,68.3 30.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:436;--dur:2.8s" points="30.5,30.5 52.3,68.3 74.1,106.0 95.9,68.3 117.7,30.5 139.5,68.3 117.7,106.0 95.9,68.3 74.1,30.5 52.3,68.3 30.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="30.5" x2="37.5" y2="42.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,51.3 33.3,45.0 41.6,40.2" fill="#fc77be"/><circle cx="30.5" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawdedwaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:factorial`

```text
before   … │ num  ◂ top
after    … │ num  ◂ top
```

## Description

Takes a number from the [[The Stack|stack]] and computes its factorial, for example inputting 4 would return 4\*3\*2\*1=24.

## Related patterns (same book page)

- [[Running Sum Purification]] `[num] → [num]`
- [[Running Product Purification]] `[num] → [num]`

## Sources

- 📖 **Guidebook:** [[Hexal Maths]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Factorial Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/hexal_maths@hexal:factorial)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpFactorial` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:factorial`
