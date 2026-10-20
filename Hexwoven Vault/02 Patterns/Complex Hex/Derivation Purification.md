---
aliases: ["Derivation Purification"]
tags: [hexcasting/pattern, mod/complex-hex, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:derivative"
signature: "wdwddwewddwdw"
start_direction: "SOUTH_EAST"
input: "expr, expr"
output: "expr"
cost: ""
iotas_in: ["[[Expression]]"]
iotas_out: ["[[Expression]]"]
entry: "[[Expressions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Derivation Purification

◇ **Pattern** · [[Complex Hex]] · 📖 [[Expressions]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="88" height="170" viewBox="0 0 87.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="43.8,18.0 56.7,40.4 69.6,62.7 43.8,62.7 18.0,62.7 30.9,40.4 43.8,62.7 56.7,85.0 43.8,107.3 30.9,129.6 18.0,107.3 43.8,107.3 69.6,107.3 56.7,129.6 43.8,152.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:361;--dur:3.9s" points="43.8,18.0 56.7,40.4 69.6,62.7 43.8,62.7 18.0,62.7 30.9,40.4 43.8,62.7 56.7,85.0 43.8,107.3 30.9,129.6 18.0,107.3 43.8,107.3 69.6,107.3 56.7,129.6 43.8,152.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="43.8" y1="18.0" x2="47.9" y2="25.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.9,30.3 45.5,26.6 50.4,23.8" fill="#fc77be"/><circle cx="43.8" cy="18.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwddwewddwdw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `expr, expr` ([[Expression]])
- **Leaves:** `expr` ([[Expression]])
- **ID:** `complexhex:derivative`

```text
before   … │ expr │ expr  ◂ top
after    … │ expr         ◂ top
```

## Description

Takes in an expression and a lone symbol and returns the partial derivative of the expression with respect to the symbol.

## Related patterns (same book page)

- [[Symbolic Purification]] `str → expression`
- [[Substitution Exaltation]] `expr, expr, expr | num → expr`
- [[Equality Purification]] `expr, expr | num → expr`
- [[Piecewise Exaltation]] `expr, expr|num, expr|num → expr`
- [[Neo's Exaltation]] `num, num, expr → matrix`
- [[Parametric Line]] `vec, expr → entity`
- [[Parametric Surface]] `vec, expr → entity`

## Sources

- 📖 **Guidebook:** [[Expressions]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Derivation Purification](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions@complexhex:derivative)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDerivative` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:derivative`
