---
aliases: ["Subtractive Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:sub"
signature: "wddw"
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

# Subtractive Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,132.0 65.1,85.0 38.0,38.0 92.3,38.0 65.1,85.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="92.3,132.0 65.1,85.0 38.0,38.0 92.3,38.0 65.1,85.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="132.0" x2="83.6" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="77.3,106.2 88.8,114.0 78.4,119.9" fill="#fc77be"/><circle cx="92.3" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wddw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num|vec, num|vec` ([[Number]], [[Vector]])
- **Leaves:** `num|vec` ([[Number]], [[Vector]])
- **ID:** `hexcasting:sub`

## Description

Perform subtraction.

As such:
- With two numbers at the top of the [[The Stack|stack]], combines them into their difference.
- With a number and a vector, removes the number from the stack and subtracts it from each element of the vector.
- With two vectors, combines them by subtracting each component.

In all cases, the top of the stack or its components are subtracted from the second-from-the-top.

## Example

`10, 3 → 7`. With vectors, `target − origin` gives the vector **pointing from origin to target**.

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Complexities]]

`complex, complex/num` → `complex`

Performs Subtraction, num will be treated as num + 0i.

### [[Complex Hex]] · 📖 [[Longs]]

`long, long` → `long`

Perform Subtraction

### [[Complex Hex]] · 📖 [[Quaternionics]]

`qrtn, qtrn` → `quaternion`

Performs Subtraction

### [[PhLib]] · 📖 [[Maps]]

`map<k, v>, map<t, *>` → `map<k, v>`

Removes every entry in the first map that is found in the second map. The values in the second map are ignored.

### [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]]

`set, set` → `set`

Remove the set at the top of the stack, then remove all its elements from the set at the top of the stack. The two sets must be of matching type.

## Related patterns (same book page)

- [[Additive Distillation]] `num|vec, num|vec → num|vec`
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
- 🌐 **Official online docs:** [Subtractive Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:sub)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:sub`
