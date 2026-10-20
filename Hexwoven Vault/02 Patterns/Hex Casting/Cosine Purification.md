---
aliases: ["Cosine Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:cos"
signature: "qqqqqad"
start_direction: "SOUTH_EAST"
input: "num"
output: "num"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Advanced Mathematics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cosine Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Advanced Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 85.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 85.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="43.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,102.1 38.2,94.9 47.8,89.4" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqad` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:cos`

```text
before   … │ num  ◂ top
after    … │ num  ◂ top
```

## Description

Takes the cosine of an angle in radians, yielding the horizontal component of that angle drawn on a unit circle. Related to the values of [[Arc's Reflection|π]] and [[Circle's Reflection|τ]].

## Related patterns (same book page)

- [[Sine Purification]] `num → num`
- [[Tangent Purification]] `num → num`
- [[Inverse Sine Purification]] `num → num`
- [[Inverse Cosine Purification]] `num → num`
- [[Inverse Tangent Purification]] `num → num`
- [[Inverse Tangent Distillation]] `num, num → num`
- [[Logarithmic Distillation]] `num, num → num`

## Sources

- 📖 **Guidebook:** [[Advanced Mathematics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Cosine Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/advanced_math@hexcasting:cos)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:cos`
