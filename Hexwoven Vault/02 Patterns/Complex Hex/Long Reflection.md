---
aliases: ["Long Reflection"]
tags: [hexcasting/pattern, mod/complex-hex, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:long"
signature: "awdedwaaw"
start_direction: "SOUTH_EAST"
input: ""
output: "long"
cost: ""
iotas_in: []
iotas_out: ["[[Long]]"]
entry: "[[Longs]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Long Reflection

◇ **Pattern** · [[Complex Hex]] · 📖 [[Longs]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3 60.0,35.0 35.0,78.3 85.0,78.3 135.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:500;--dur:2.8s" points="35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3 60.0,35.0 35.0,78.3 85.0,78.3 135.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="43.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,102.1 38.2,94.9 47.8,89.4" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awdedwaaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `long` ([[Long]])
- **ID:** `complexhex:long`

```text
before   …         ◂ top
after    … │ long  ◂ top
```

## Description

Just like with normal numbers, Nature is not so generous as to make this easy for us.
(Details on next page)

Thankfully, what each angle does to the count is very similar to the norm, the only differences being:
- a sharp left will shift the bits in the count to the left (effectively doubling them)
- a sharp right will shift the bits in the count to the right (effectively halving them\*)

\* Due to a long's inability to contain a decimal, the value will always be rounded down.

## Related patterns (same book page)

- [[Example 43L]] `— → —`
- [[Long Purification]] `num/long → long/num`
- [[Left Shift Distillation]] `long, num → long`
- [[Logical Right Shift Dstl]] `long, num → long`
- [[Arithmetic R. Shift Dstl]] `long, num → long`

## Sources

- 📖 **Guidebook:** [[Longs]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Long Reflection](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/longs@complexhex:long)
- ✏️ **Strokes:** from the book page, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `complexhex:long`
