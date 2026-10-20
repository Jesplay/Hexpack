---
aliases: ["Power Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:pow"
signature: "wedew"
start_direction: "NORTH_WEST"
input: "num|vec, num|vec"
output: "num|vec"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Number]]", "[[Vector]]"]
entry: "[[Mathematics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Power Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,140.2 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="72.3,140.2 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="140.2" x2="65.5" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,120.0 69.5,126.1 61.4,130.8" fill="#fc77be"/><circle cx="72.3" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wedew` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num|vec, num|vec` ([[Number]], [[Vector]])
- **Leaves:** `num|vec` ([[Number]], [[Vector]])
- **ID:** `hexcasting:pow`

## Description

Perform exponentiation or vector projection.

With two numbers, combines them by raising the first to the power of the second.
- With a number and a vector, removes the number and raises each component of the vector to the number's power.
- With two vectors, combines them into the [vector projection](https://en.wikipedia.org/wiki/Vector_projection) of the second-from-the-top onto the top of the [[The Stack|stack]].

In the first and second cases, the first argument or its components are the base, and the second argument or its components are the exponent.

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Complexities]]

`num, complex` → `complex`

Performs Exponentiation.

### [[MoreIotas]] · 📖 [[Matrices]]

`mat, int` → `mat`

Accepts a square matrix and an integer, and raises the matrix to the power of the integer.

## Related patterns (same book page)

- [[Additive Distillation]] `num|vec, num|vec → num|vec`
- [[Subtractive Distillation]] `num|vec, num|vec → num|vec`
- [[Multiplicative Distillation]] `num|vec, num|vec → num|vec`
- [[Division Distillation]] `num|vec, num|vec → num|vec`
- [[Length Purification]] `num|vec → number`
- [[Floor Purification]] `num|vec → num|vec`
- [[Ceiling Purification]] `num|vec → num|vec`
- [[Vector Exaltation]] `num, num, num → vector`
- [[Vector Disintegration]] `vector → num, num, num`
- [[Modulus Distillation]] `num|vec, num|vec → num|vec`
- [[Axial Purification]] `vec|num → vec|num`
- [[Entropy Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Mathematics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Power Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:pow)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:pow`
