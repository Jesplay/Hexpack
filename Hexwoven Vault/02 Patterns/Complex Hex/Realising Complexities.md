---
aliases: ["Realising Complexities"]
tags: [hexcasting/pattern, mod/complex-hex, family/other, topic/math]
kind: "Pattern"
family: "Other"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:real"
signature: "deew"
start_direction: "SOUTH_EAST"
input: "complex"
output: "num"
cost: ""
iotas_in: ["[[Complex Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Complexities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Realising Complexities

◇ **Pattern** · [[Complex Hex]] · 📖 [[Complexities]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 72.3,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="72.3,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 72.3,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="103.4" x2="79.1" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.0,123.7 75.0,117.5 83.1,112.9" fill="#fc77be"/><circle cx="72.3" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deew` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `complex` ([[Complex Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `complexhex:real`

```text
before   … │ complex  ◂ top
after    … │ num      ◂ top
```

## Description

Pushes the real coefficient.

## Related patterns (same book page)

- [[Constant Imagination]] `— → complex`
- [[Constant Realisation]] `— → complex`
- [[Imagining Complexities]] `complex → num`
- [[Conjugation Prfn]] `complex → complex`

## Sources

- 📖 **Guidebook:** [[Complexities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Realising Complexities](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/complexities@complexhex:real)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReal` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:real`
