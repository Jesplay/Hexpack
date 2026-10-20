---
aliases: ["Substitution Exaltation"]
tags: [hexcasting/pattern, mod/complex-hex, family/exaltation, topic/math]
kind: "Pattern"
family: "Exaltation"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:substitute"
signature: "wdwddwdw"
start_direction: "SOUTH_EAST"
input: "expr, expr, expr | num"
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

# Substitution Exaltation

◇ **Pattern** · [[Complex Hex]] · 📖 [[Expressions]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,29.8 114.8,66.6 136.1,103.4 93.5,103.4 51.0,103.4 72.3,66.6 93.5,103.4 114.8,140.2 72.3,140.2 29.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="93.5,29.8 114.8,66.6 136.1,103.4 93.5,103.4 51.0,103.4 72.3,66.6 93.5,103.4 114.8,140.2 72.3,140.2 29.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="29.8" x2="100.3" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.2,50.0 96.3,43.9 104.4,39.2" fill="#fc77be"/><circle cx="93.5" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwddwdw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `expr, expr, expr | num` ([[Number]], [[Expression]])
- **Leaves:** `expr` ([[Expression]])
- **ID:** `complexhex:substitute`

## Description

Substitutes the third expression/number in place of the second expression within the first.

## Related patterns (same book page)

- [[Symbolic Purification]] `str → expression`
- [[Equality Purification]] `expr, expr | num → expr`
- [[Piecewise Exaltation]] `expr, expr|num, expr|num → expr`
- [[Derivation Purification]] `expr, expr → expr`
- [[Neo's Exaltation]] `num, num, expr → matrix`
- [[Parametric Line]] `vec, expr → entity`
- [[Parametric Surface]] `vec, expr → entity`

## Sources

- 📖 **Guidebook:** [[Expressions]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Substitution Exaltation](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions@complexhex:substitute)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSubstitute` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:substitute`
