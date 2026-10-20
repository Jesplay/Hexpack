---
aliases: ["Tangent Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:tan"
signature: "wqqqqqadq"
start_direction: "SOUTH_WEST"
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

# Tangent Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Advanced Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,29.8 72.3,66.6 51.0,103.4 72.3,140.2 114.8,140.2 136.1,103.4 114.8,66.6 72.3,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="93.5,29.8 72.3,66.6 51.0,103.4 72.3,140.2 114.8,140.2 136.1,103.4 114.8,66.6 72.3,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="29.8" x2="86.7" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="81.9,50.0 82.7,39.2 90.8,43.9" fill="#fc77be"/><circle cx="93.5" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqqqadq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:tan`

```text
before   … │ num  ◂ top
after    … │ num  ◂ top
```

## Description

Takes the tangent of an angle in radians, yielding the slope of that angle drawn on a circle. Related to the values of [[Arc's Reflection|π]] and [[Circle's Reflection|τ]].

## Related patterns (same book page)

- [[Sine Purification]] `num → num`
- [[Cosine Purification]] `num → num`
- [[Inverse Sine Purification]] `num → num`
- [[Inverse Cosine Purification]] `num → num`
- [[Inverse Tangent Purification]] `num → num`
- [[Inverse Tangent Distillation]] `num, num → num`
- [[Logarithmic Distillation]] `num, num → num`

## Sources

- 📖 **Guidebook:** [[Advanced Mathematics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Tangent Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/advanced_math@hexcasting:tan)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:tan`
