---
aliases: ["Running Product Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hexal"
op_id: "hexal:running/mul"
signature: "qaawaaq"
start_direction: "NORTH_EAST"
input: "[num]"
output: "[num]"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[Number]]", "[[List]]"]
entry: "[[Hexal Maths]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Running Product Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Hexal Maths]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,121.6 85.0,78.3 60.0,35.0 35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="60.0,121.6 85.0,78.3 60.0,35.0 35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="121.6" x2="68.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="73.8,97.8 72.8,110.5 63.2,105.0" fill="#fc77be"/><circle cx="60.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaawaaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[num]` ([[Number]], [[List]])
- **Leaves:** `[num]` ([[Number]], [[List]])
- **ID:** `hexal:running/mul`

```text
before   … │ [num]  ◂ top
after    … │ [num]  ◂ top
```

## Description

Takes a list from the [[The Stack|stack]] and computes its running product, for example inputting \[1,2,5\] would return \[1,2,10\].

## Related patterns (same book page)

- [[Factorial Purification]] `num → num`
- [[Running Sum Purification]] `[num] → [num]`

## Sources

- 📖 **Guidebook:** [[Hexal Maths]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Running Product Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/hexal_maths@hexal:running/mul)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRunningOp` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:running/mul`
