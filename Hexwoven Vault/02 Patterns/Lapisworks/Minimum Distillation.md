---
aliases: ["Minimum Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Lapisworks"
op_id: "lapisworks:min"
signature: "waqqwqddqwqqaw"
start_direction: "WEST"
input: "num, num"
output: "num"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Miscellaneous]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Minimum Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Miscellaneous]] · Lapisworks

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="106" viewBox="0 0 170.0 106.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.6" cy="18.6" r="2"/><circle cx="45.2" cy="18.6" r="2"/><circle cx="71.7" cy="18.6" r="2"/><circle cx="98.3" cy="18.6" r="2"/><circle cx="124.8" cy="18.6" r="2"/><circle cx="151.4" cy="18.6" r="2"/><circle cx="5.3" cy="41.6" r="2"/><circle cx="31.9" cy="41.6" r="2"/><circle cx="58.4" cy="41.6" r="2"/><circle cx="85.0" cy="41.6" r="2"/><circle cx="111.6" cy="41.6" r="2"/><circle cx="138.1" cy="41.6" r="2"/><circle cx="164.7" cy="41.6" r="2"/><circle cx="18.6" cy="64.6" r="2"/><circle cx="45.2" cy="64.6" r="2"/><circle cx="71.7" cy="64.6" r="2"/><circle cx="98.3" cy="64.6" r="2"/><circle cx="124.8" cy="64.6" r="2"/><circle cx="151.4" cy="64.6" r="2"/><circle cx="5.3" cy="87.6" r="2"/><circle cx="31.9" cy="87.6" r="2"/><circle cx="58.4" cy="87.6" r="2"/><circle cx="85.0" cy="87.6" r="2"/><circle cx="111.6" cy="87.6" r="2"/><circle cx="138.1" cy="87.6" r="2"/><circle cx="164.7" cy="87.6" r="2"/></g><polyline class="hexpat-base" points="71.7,64.6 45.2,64.6 18.6,64.6 31.9,87.6 58.4,87.6 71.7,64.6 85.0,41.6 71.7,18.6 98.3,18.6 85.0,41.6 98.3,64.6 111.6,87.6 138.1,87.6 151.4,64.6 124.8,64.6 98.3,64.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:398;--dur:4.2s" points="71.7,64.6 45.2,64.6 18.6,64.6 31.9,87.6 58.4,87.6 71.7,64.6 85.0,41.6 71.7,18.6 98.3,18.6 85.0,41.6 98.3,64.6 111.6,87.6 138.1,87.6 151.4,64.6 124.8,64.6 98.3,64.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.7" y1="64.6" x2="63.2" y2="64.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.1,64.6 63.2,61.7 63.2,67.5" fill="#fc77be"/><circle cx="71.7" cy="64.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqqwqddqwqqaw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `lapisworks:min`

```text
before   … │ num │ num  ◂ top
after    … │ num        ◂ top
```

## Description

Returns the smaller of the two numbers.

## Related patterns (same book page)

- [[Hadamard Distillation]] `vec, vec → vec`
- [[Robbie's Exaltation]] `[any], int, any → [any]`
- [[Dealer's Purification]] `[any] → any`
- [[Maximum Distillation]] `num, num → num`

## Sources

- 📖 **Guidebook:** [[Miscellaneous]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `MinMax` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:min`
