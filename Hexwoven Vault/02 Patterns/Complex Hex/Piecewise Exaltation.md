---
aliases: ["Piecewise Exaltation"]
tags: [hexcasting/pattern, mod/complex-hex, family/exaltation, topic/math]
kind: "Pattern"
family: "Exaltation"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:piecewise"
signature: "wdwddwawdd"
start_direction: "SOUTH_EAST"
input: "expr, expr|num, expr|num"
output: "expr"
cost: ""
iotas_in: ["[[Number]]", "[[Expression]]"]
iotas_out: ["[[Expression]]"]
entry: "[[Expressions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Piecewise Exaltation

◇ **Pattern** · [[Complex Hex]] · 📖 [[Expressions]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,27.0 85.0,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 65.7,94.0 85.0,127.4 104.3,94.0 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="65.7,27.0 85.0,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 65.7,94.0 85.0,127.4 104.3,94.0 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="27.0" x2="71.9" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,45.4 68.2,39.9 75.5,35.6" fill="#fc77be"/><circle cx="65.7" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwddwawdd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `expr, expr|num, expr|num` ([[Number]], [[Expression]])
- **Leaves:** `expr` ([[Expression]])
- **ID:** `complexhex:piecewise`

## Description

Creates a piecewise expression that simplifies to the second argument if the first simplifies to 1.0, otherwise simplifies to the third argument.

## Related patterns (same book page)

- [[Symbolic Purification]] `str → expression`
- [[Substitution Exaltation]] `expr, expr, expr | num → expr`
- [[Equality Purification]] `expr, expr | num → expr`
- [[Derivation Purification]] `expr, expr → expr`
- [[Neo's Exaltation]] `num, num, expr → matrix`
- [[Parametric Line]] `vec, expr → entity`
- [[Parametric Surface]] `vec, expr → entity`

## Sources

- 📖 **Guidebook:** [[Expressions]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Piecewise Exaltation](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions@complexhex:piecewise)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPiecewise` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:piecewise`
