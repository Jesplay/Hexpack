---
aliases: ["Hadamard Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Lapisworks"
op_id: "lapisworks:hadamard"
signature: "awddwde"
start_direction: "WEST"
input: "vec, vec"
output: "vec"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Miscellaneous]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hadamard Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Miscellaneous]] · Lapisworks

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,38.0 38.0,38.0 65.1,85.0 92.3,132.0 38.0,132.0 65.1,85.0 92.3,38.0 119.4,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:434;--dur:2.2s" points="92.3,38.0 38.0,38.0 65.1,85.0 92.3,132.0 38.0,132.0 65.1,85.0 92.3,38.0 119.4,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="38.0" x2="74.9" y2="38.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.4,38.0 74.9,32.0 74.9,44.0" fill="#fc77be"/><circle cx="92.3" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awddwde` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `vec` ([[Vector]])
- **ID:** `lapisworks:hadamard`

```text
before   … │ vec │ vec  ◂ top
after    … │ vec        ◂ top
```

## Description

Based off the Hadamard Product, this pattern multiplies the respective elements of two vectors.

## Related patterns (same book page)

- [[Robbie's Exaltation]] `[any], int, any → [any]`
- [[Dealer's Purification]] `[any] → any`
- [[Minimum Distillation]] `num, num → num`
- [[Maximum Distillation]] `num, num → num`

## Sources

- 📖 **Guidebook:** [[Miscellaneous]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `Hadamard` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:hadamard`
