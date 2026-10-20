---
aliases: ["Division Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:div"
signature: "wdedw"
start_direction: "NORTH_EAST"
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

# Division Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,132.0 65.1,85.0 92.3,38.0 119.4,85.0 92.3,132.0 65.1,85.0 38.0,38.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="38.0,132.0 65.1,85.0 92.3,38.0 119.4,85.0 92.3,132.0 65.1,85.0 38.0,38.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="132.0" x2="46.7" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,106.2 51.8,119.9 41.5,114.0" fill="#fc77be"/><circle cx="38.0" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdedw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num|vec, num|vec` ([[Number]], [[Vector]])
- **Leaves:** `num|vec` ([[Number]], [[Vector]])
- **ID:** `hexcasting:div`

## Description

Perform division or the cross product.

As such:
- With two numbers, combines them into their quotient.
- With a number and a vector, removes the number and divides each element of the vector by it.
- With two vectors, combines them into their [cross product](https://www.mathsisfun.com/algebra/vectors-cross-product.html).

In the first and second cases, the second-from-the-top of the [[The Stack|stack]] is divided by the top of the stack.

WARNING: Never divide by zero!

## Example

`12, 4 → 3`. Two vectors give their **cross product**.

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Complexities]]

`complex, num` → `complex`

Performs Scalar Division.

### [[Complex Hex]] · 📖 [[Longs]]

`long, long` → `long`

Perform Floored Division

### [[Complex Hex]] · 📖 [[Quaternionics]]

`quaternion, num` → `quaternion`

Performs Scalar Division.

### [[Hexic]] · 📖 [[Advanced Lists]]

`[a], [b]` → `[[a, b]]`

Calculates the inner product of two lists. If the two lists were arranged side-by-side, the resulting list has one entry for every row (unpaired elements are skipped).

### [[HexThings]] · 📖 [[Advanced List Operators]]

`list, list` → `list`

Pushes a list containing pairs of values corresponding to the values at that index in each list.

### [[MoreIotas]] · 📖 [[Matrices]]

`mat, mat` → `mat`

Divides the first n by m matrix with an m by m matrix to get an n by m matrix.

## Related patterns (same book page)

- [[Additive Distillation]] `num|vec, num|vec → num|vec`
- [[Subtractive Distillation]] `num|vec, num|vec → num|vec`
- [[Multiplicative Distillation]] `num|vec, num|vec → num|vec`
- [[Length Purification]] `num|vec → number`
- [[Power Distillation]] `num|vec, num|vec → num|vec`
- [[Floor Purification]] `num|vec → num|vec`
- [[Ceiling Purification]] `num|vec → num|vec`
- [[Vector Exaltation]] `num, num, num → vector`
- [[Vector Disintegration]] `vector → num, num, num`
- [[Modulus Distillation]] `num|vec, num|vec → num|vec`
- [[Axial Purification]] `vec|num → vec|num`
- [[Entropy Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Mathematics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Division Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:div)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:div`
