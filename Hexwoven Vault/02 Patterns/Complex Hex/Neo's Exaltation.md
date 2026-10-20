---
aliases: ["Neo's Exaltation"]
tags: [hexcasting/pattern, mod/complex-hex, family/exaltation, topic/math]
kind: "Pattern"
family: "Exaltation"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:matrix"
signature: "wdwddweawwaeaw"
start_direction: "SOUTH_EAST"
input: "num, num, expr"
output: "matrix"
cost: ""
iotas_in: ["[[Number]]", "[[Expression]]"]
iotas_out: ["[[Matrix]]"]
entry: "[[Expressions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Neo's Exaltation

◇ **Pattern** · [[Complex Hex]] · 📖 [[Expressions]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="53.5,22.0 69.3,49.3 85.0,76.6 53.5,76.6 22.0,76.6 37.8,49.3 53.5,76.6 69.3,103.8 53.5,131.1 85.0,131.1 116.5,131.1 148.0,131.1 132.2,103.8 148.0,76.6 116.5,76.6 85.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="53.5,22.0 69.3,49.3 85.0,76.6 53.5,76.6 22.0,76.6 37.8,49.3 53.5,76.6 69.3,103.8 53.5,131.1 85.0,131.1 116.5,131.1 148.0,131.1 132.2,103.8 148.0,76.6 116.5,76.6 85.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="22.0" x2="58.6" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.2,37.0 55.6,32.5 61.6,29.0" fill="#fc77be"/><circle cx="53.5" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwddweawwaeaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num, expr` ([[Number]], [[Expression]])
- **Leaves:** `matrix` ([[Matrix]])
- **ID:** `complexhex:matrix`

```text
before   … │ num │ num │ expr  ◂ top
after    … │ matrix            ◂ top
```

## Description

Takes in a width, height and expression and creates a matrix by substituting i & j-values into the expression corresponding to a position in the matrix and putting the result in the matrix.

## Related patterns (same book page)

- [[Symbolic Purification]] `str → expression`
- [[Substitution Exaltation]] `expr, expr, expr | num → expr`
- [[Equality Purification]] `expr, expr | num → expr`
- [[Piecewise Exaltation]] `expr, expr|num, expr|num → expr`
- [[Derivation Purification]] `expr, expr → expr`
- [[Parametric Line]] `vec, expr → entity`
- [[Parametric Surface]] `vec, expr → entity`

## Sources

- 📖 **Guidebook:** [[Expressions]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Neo's Exaltation](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions@complexhex:matrix)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMakeMatrix` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:matrix`
