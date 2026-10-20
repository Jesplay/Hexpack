---
aliases: ["Additive Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:add"
signature: "waaw"
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

# Additive Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="38.0,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="132.0" x2="46.7" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,106.2 51.8,119.9 41.5,114.0" fill="#fc77be"/><circle cx="38.0" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num|vec, num|vec` ([[Number]], [[Vector]])
- **Leaves:** `num|vec` ([[Number]], [[Vector]])
- **ID:** `hexcasting:add`

## Description

Perform addition.

As such:
- With two numbers at the top of the [[The Stack|stack]], combines them into their sum.
- With a number and a vector, removes the number from the stack and adds it to each element of the vector.
- With two vectors, combines them by summing corresponding components into a new vector (i.e. (1, 2, 3) + (0, 4, -1) = (1, 6, 2)).

## Example

- `2, 3 → 5`
- `(1, 2, 3), (1, 0, 0) → (2, 2, 3)`: add two vectors (e.g. a position plus an offset)
- `(1, 2, 3), 1 → (2, 3, 4)`: a number added to a vector is added to every component

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Complexities]]

`complex, complex/num` → `complex`

Performs Addition, num will be treated as num + 0i.

### [[Complex Hex]] · 📖 [[Longs]]

`long, long` → `long`

Perform Addition

### [[Complex Hex]] · 📖 [[Quaternionics]]

`qtrn, qtrn` → `quaternion`

Performs Addition

### [[Hexal]] · 📖 [[Motes]]

`mote, mote` → `mote`

Combines two motes into one, merging the mote records they point to. Any pointers to the second mote will become null, any pointers to the first will remain. This can make motes holding many more items than I could carry in a stack naturally!

### [[PhLib]] · 📖 [[Maps]]

`map<k, v>, map<k, v>` → `map<k, v>`

Merges two maps together. Maps cannot have duplicate items: any entry in the latter map will override that in the first map.

### [[Hexpose]] · 📖 [[Text Iota]]

`text, text` → `text`

Pushes a [[Text Iota|text iota]] of the first followed immediately by the second.

### [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]]

`set, set` → `set`

Remove the set at the top of the stack, then add all its elements to the set at the top of the stack. The two sets must be of matching type.

### [[Hex Casting]] · 📖 [[List Manipulation]]

`list, list` → `list`

Remove the list at the top of the stack, then add all its elements to the end of the list at the top of the stack.

### [[MoreIotas]] · 📖 [[Matrices]]

`mat, mat` → `mat`

Adds two matrices together; they must have the same number of columns and rows.

### [[MoreIotas]] · 📖 [[Strings]]

`str, str` → `str`

Removes the top two strings from the stack and combines them together.

### [[Caduceus]] · 📖 [[Jump Manipulation]]

`jump, jump` → `jump`

Remove the Jump iota at the top of the stack, then add all its frames to the bottom of the Jump iota at the top of the stack.

## Related patterns (same book page)

- [[Subtractive Distillation]] `num|vec, num|vec → num|vec`
- [[Multiplicative Distillation]] `num|vec, num|vec → num|vec`
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
- 🌐 **Official online docs:** [Additive Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:add)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:add`
