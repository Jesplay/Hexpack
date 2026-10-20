---
aliases: ["Running Sum Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hexal"
op_id: "hexal:running/sum"
signature: "aea"
start_direction: "WEST"
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

# Running Sum Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Hexal Maths]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="38.0" x2="74.9" y2="38.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.4,38.0 74.9,32.0 74.9,44.0" fill="#fc77be"/><circle cx="92.3" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aea` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[num]` ([[Number]], [[List]])
- **Leaves:** `[num]` ([[Number]], [[List]])
- **ID:** `hexal:running/sum`

```text
before   … │ [num]  ◂ top
after    … │ [num]  ◂ top
```

## Description

Takes a list from the [[The Stack|stack]] and computes its running sum, for example inputting \[1,2,5\] would return \[1,3,8\].

## Related patterns (same book page)

- [[Factorial Purification]] `num → num`
- [[Running Product Purification]] `[num] → [num]`

## Sources

- 📖 **Guidebook:** [[Hexal Maths]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Running Sum Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/hexal_maths@hexal:running/sum)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRunningOp` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:running/sum`
