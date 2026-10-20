---
aliases: ["Conjugation Prfn."]
tags: [hexcasting/pattern, mod/complex-hex, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:conjugate"
signature: "wqqd"
start_direction: "SOUTH_WEST"
input: "complex"
output: "complex"
cost: ""
iotas_in: ["[[Complex Number]]"]
iotas_out: ["[[Complex Number]]"]
entry: "[[Complexities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjugation Prfn.

◇ **Pattern** · [[Complex Hex]] · 📖 [[Complexities]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="101" height="170" viewBox="0 0 101.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 76.9,115.3 59.4,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:175;--dur:1.6s" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 76.9,115.3 59.4,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="24.5" x2="53.8" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,41.1 50.5,32.2 57.2,36.1" fill="#fc77be"/><circle cx="59.4" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `complex` ([[Complex Number]])
- **Leaves:** `complex` ([[Complex Number]])
- **ID:** `complexhex:conjugate`

```text
before   … │ complex  ◂ top
after    … │ complex  ◂ top
```

## Description

Negates the imaginary coefficient of the topmost [[Iotas|iota]]

## Related patterns (same book page)

- [[Constant Imagination]] `— → complex`
- [[Constant Realisation]] `— → complex`
- [[Realising Complexities]] `complex → num`
- [[Imagining Complexities]] `complex → num`

## Sources

- 📖 **Guidebook:** [[Complexities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjugation Prfn.](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/complexities@complexhex:conjugate)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpConjugate` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:conjugate`
