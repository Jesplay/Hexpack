---
aliases: ["Arithmetic R. Shift Dstl."]
tags: [hexcasting/pattern, mod/complex-hex, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:shift/right/arithmetic"
signature: "eaqq"
start_direction: "SOUTH_EAST"
input: "long, num"
output: "long"
cost: ""
iotas_in: ["[[Number]]", "[[Long]]"]
iotas_out: ["[[Long]]"]
entry: "[[Longs]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Arithmetic R. Shift Dstl.

◇ **Pattern** · [[Complex Hex]] · 📖 [[Longs]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0 119.4,85.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0 119.4,85.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="38.0" x2="46.7" y2="53.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,63.8 41.5,56.0 51.8,50.1" fill="#fc77be"/><circle cx="38.0" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `long, num` ([[Number]], [[Long]])
- **Leaves:** `long` ([[Long]])
- **ID:** `complexhex:shift/right/arithmetic`

```text
before   … │ long │ num  ◂ top
after    … │ long        ◂ top
```

## Description

Shifts all the bits to the right num times while preserving the sign

## Related patterns (same book page)

- [[Long Reflection]] `— → long`
- [[Example 43L]] `— → —`
- [[Long Purification]] `num/long → long/num`
- [[Left Shift Distillation]] `long, num → long`
- [[Logical Right Shift Dstl]] `long, num → long`

## Sources

- 📖 **Guidebook:** [[Longs]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Arithmetic R. Shift Dstl.](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/longs@complexhex:shift/right/arithmetic)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpArithmeticRightBitshift` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:shift/right/arithmetic`
