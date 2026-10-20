---
aliases: ["Length Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:abs"
signature: "wqaqw"
start_direction: "NORTH_EAST"
input: "num|vec"
output: "number"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Number]]"]
entry: "[[Mathematics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Length Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 72.3,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="29.8,140.2 51.0,103.4 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 72.3,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaqw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num|vec` ([[Number]], [[Vector]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexcasting:abs`

## Description

Compute the absolute value or length.

Replaces a number with its absolute value, or a vector with its length.

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Complexities]]

`complex` → `num`

Pushes the Argument (the length).

### [[Complex Hex]] · 📖 [[Quaternionics]]

`quaternion` → `num`

Pushes the Argument (the length).

### [[Hexal]] · 📖 [[Motes]]

`mote` → `int`

Accepts a mote and returns the number of items in the record referenced by that mote.

### [[Hexal]] · 📖 [[Gates]]

`gate` → `num`

Consumes a gate iota, returns how many entities are marked by that gate.

### [[HexThings]] · 📖 [[Dictionary Manipulation]]

`dict` → `num`

Pushes the number of entries in the Dictionary

### [[Hexpose]] · 📖 [[Text Iota]]

`text` → `number`

Pushes the length of the [[Text Iota|text iota]].

### [[Hex Casting]] · 📖 [[Logical Operators]]

`bool` → `number`

Convert a boolean to a number; True becomes **1**, and False becomes **0**.

### [[MoreIotas]] · 📖 [[Items]]

`item stack` → `int`

Transforms the Item Stack at the top of the stack into its size.

### [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]]

`set` → `num`

Remove the set at the top of the stack, then push the number of elements in the set to the stack.

### [[Hex Casting]] · 📖 [[List Manipulation]]

`list` → `num`

Remove the list at the top of the stack, then push the number of elements in the list to the stack.

### [[MoreIotas]] · 📖 [[Strings]]

`str` → `num`

Removes a string and returns its length.

### [[Caduceus]] · 📖 [[Jump Manipulation]]

`jump` → `num`

Remove the Jump iota at the top of the stack, then push the number of frames within it to the stack.

## Related patterns (same book page)

- [[Additive Distillation]] `num|vec, num|vec → num|vec`
- [[Subtractive Distillation]] `num|vec, num|vec → num|vec`
- [[Multiplicative Distillation]] `num|vec, num|vec → num|vec`
- [[Division Distillation]] `num|vec, num|vec → num|vec`
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
- 🌐 **Official online docs:** [Length Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:abs)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:abs`
