---
aliases: ["Symbolic Purification"]
tags: [hexcasting/pattern, mod/complex-hex, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:new_symbol"
signature: "wdwdd"
start_direction: "SOUTH_EAST"
input: "str"
output: "expression"
cost: ""
iotas_in: ["[[String]]"]
iotas_out: ["[[Expression]]"]
entry: "[[Expressions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Symbolic Purification

◇ **Pattern** · [[Complex Hex]] · 📖 [[Expressions]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,35.0 110.0,78.3 135.0,121.6 85.0,121.6 35.0,121.6 60.0,78.3 85.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="85.0,35.0 110.0,78.3 135.0,121.6 85.0,121.6 35.0,121.6 60.0,78.3 85.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="35.0" x2="93.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="98.7,58.8 88.2,51.6 97.8,46.1" fill="#fc77be"/><circle cx="85.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwdd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `str` ([[String]])
- **Leaves:** `expression` ([[Expression]])
- **ID:** `complexhex:new_symbol`

```text
before   … │ str         ◂ top
after    … │ expression  ◂ top
```

## Description

Creates a new symbol with the given (single character) label

Many operations that work on numbers can also apply to expressions.
The full list is: Addition, Subtraction, Multiplication, Division, Powers, Absolute, Floor, Ceiling, Sine, Cosine, Tangent, Arcsin, Arccos, Arctan, Sinh, Cosh, Tanh, ArcSinh, ArcCosh, ArcTanh, Vector Pack, Logarithms, Modulo, Minimus, Minimus II, Maximus, Maximus II, And, Or, and Not
In the case of operators that work on booleans, 0 is treated as "false" and any non-zero value is treated as "true".

## Related patterns (same book page)

- [[Substitution Exaltation]] `expr, expr, expr | num → expr`
- [[Equality Purification]] `expr, expr | num → expr`
- [[Piecewise Exaltation]] `expr, expr|num, expr|num → expr`
- [[Derivation Purification]] `expr, expr → expr`
- [[Neo's Exaltation]] `num, num, expr → matrix`
- [[Parametric Line]] `vec, expr → entity`
- [[Parametric Surface]] `vec, expr → entity`

## Sources

- 📖 **Guidebook:** [[Expressions]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Symbolic Purification](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions@complexhex:new_symbol)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNewSymbol` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:new_symbol`
