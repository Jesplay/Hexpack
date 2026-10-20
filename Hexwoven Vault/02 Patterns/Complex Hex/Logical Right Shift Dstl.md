---
aliases: ["Logical Right Shift Dstl."]
tags: [hexcasting/pattern, mod/complex-hex, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:shift/right/logical"
signature: "eaqqdee"
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

# Logical Right Shift Dstl.

◇ **Pattern** · [[Complex Hex]] · 📖 [[Longs]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="30.5" x2="37.5" y2="42.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,51.3 33.3,45.0 41.6,40.2" fill="#fc77be"/><circle cx="30.5" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqqdee` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `long, num` ([[Number]], [[Long]])
- **Leaves:** `long` ([[Long]])
- **ID:** `complexhex:shift/right/logical`

```text
before   … │ long │ num  ◂ top
after    … │ long        ◂ top
```

## Description

Shifts all the bits to the right num times

## Related patterns (same book page)

- [[Long Reflection]] `— → long`
- [[Example 43L]] `— → —`
- [[Long Purification]] `num/long → long/num`
- [[Left Shift Distillation]] `long, num → long`
- [[Arithmetic R. Shift Dstl]] `long, num → long`

## Sources

- 📖 **Guidebook:** [[Longs]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Logical Right Shift Dstl.](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/longs@complexhex:shift/right/logical)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpLogicalRightBitshift` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:shift/right/logical`
