---
aliases: ["Dealer's Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Lapisworks"
op_id: "lapisworks:thewhattropy"
signature: "eqqqq"
start_direction: "NORTH_WEST"
input: "[any]"
output: "any"
cost: ""
iotas_in: ["[[List]]", "[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Miscellaneous]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dealer's Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Miscellaneous]] · Lapisworks

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="114.8,140.2 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="114.8,140.2 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="114.8" y1="140.2" x2="108.0" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="103.1,120.0 112.1,126.1 104.0,130.8" fill="#fc77be"/><circle cx="114.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[any]` ([[List]], [[Any Iota]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `lapisworks:thewhattropy`

```text
before   … │ [any]  ◂ top
after    … │ any    ◂ top
```

## Description

Pops a list and pushes a random element of that list onto the [[The Stack|stack]].

## Related patterns (same book page)

- [[Hadamard Distillation]] `vec, vec → vec`
- [[Robbie's Exaltation]] `[any], int, any → [any]`
- [[Minimum Distillation]] `num, num → num`
- [[Maximum Distillation]] `num, num → num`

## Sources

- 📖 **Guidebook:** [[Miscellaneous]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `TheCoolerEntropy` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:thewhattropy`
