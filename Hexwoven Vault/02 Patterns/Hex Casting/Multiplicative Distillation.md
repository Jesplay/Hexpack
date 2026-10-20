---
aliases: ["Multiplicative Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:mul"
signature: "waqaw"
start_direction: "SOUTH_EAST"
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

# Multiplicative Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,38.0 65.1,85.0 92.3,132.0 119.4,85.0 92.3,38.0 65.1,85.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="38.0,38.0 65.1,85.0 92.3,132.0 119.4,85.0 92.3,38.0 65.1,85.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="38.0" x2="46.7" y2="53.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,63.8 41.5,56.0 51.8,50.1" fill="#fc77be"/><circle cx="38.0" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num|vec, num|vec` ([[Number]], [[Vector]])
- **Leaves:** `num|vec` ([[Number]], [[Vector]])
- **ID:** `hexcasting:mul`

## Description

Perform multiplication or the dot product.

As such:
- With two numbers, combines them into their product.
- With a number and a vector, removes the number from the [[The Stack|stack]] and multiplies each component of the vector by that number.
- With two vectors, combines them into their [dot product](https://www.mathsisfun.com/algebra/vectors-dot-product.html).

## Example

- `3, 4 → 12`
- `(0, 1, 0), 3 → (0, 3, 0)`: scale a direction (e.g. make an [[Impulse]] stronger)
- two vectors → their **dot product**

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Complexities]]

`complex, num` → `complex`

Performs Multiplication

### [[Complex Hex]] · 📖 [[Longs]]

`long, long` → `long`

Perform Multiplication

### [[Complex Hex]] · 📖 [[Quaternionics]]

`quaternion, num` → `quaternion`

Performs Multiplication, Scalar if a num & quat are given, Hamiltonian if 2 quaternions are given.

### [[Hexic]] · 📖 [[Advanced Lists]]

`[a], [b]` → `[[a, b]]`

Calculates the outer product of two lists. The resulting list has an entry for every possible pairing between the input lists.

### [[HexThings]] · 📖 [[Advanced List Operators]]

`list, list` → `list`

Pushes a list containing every combination of elements between the two lists.

### [[MoreIotas]] · 📖 [[Matrices]]

`mat, mat` → `mat`

Multiplies the first n by m by with an m by p matrix to get an n by p matrix.

## Related patterns (same book page)

- [[Additive Distillation]] `num|vec, num|vec → num|vec`
- [[Subtractive Distillation]] `num|vec, num|vec → num|vec`
- [[Division Distillation]] `num|vec, num|vec → num|vec`
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
- 🌐 **Official online docs:** [Multiplicative Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:mul)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:mul`
