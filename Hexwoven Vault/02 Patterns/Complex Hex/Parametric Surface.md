---
aliases: ["Parametric Surface"]
tags: [hexcasting/pattern, mod/complex-hex, family/other, topic/math]
kind: "Pattern"
family: "Other"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:summon/surface"
signature: "wdwddwqq"
start_direction: "SOUTH_EAST"
input: "vec, expr"
output: "entity"
cost: ""
iotas_in: ["[[Vector]]", "[[Expression]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Expressions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Parametric Surface

◇ **Pattern** · [[Complex Hex]] · 📖 [[Expressions]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,27.0 85.0,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 65.7,94.0 85.0,127.4 123.6,127.4 143.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="65.7,27.0 85.0,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 65.7,94.0 85.0,127.4 123.6,127.4 143.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="27.0" x2="71.9" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,45.4 68.2,39.9 75.5,35.6" fill="#fc77be"/><circle cx="65.7" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwddwqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, expr` ([[Vector]], [[Expression]])
- **Leaves:** `entity` ([[Entity]])
- **ID:** `complexhex:summon/surface`

```text
before   … │ vec │ expr  ◂ top
after    … │ entity      ◂ top
```

## Description

Summons a parametric surface at the given position. The expression given will have a u&v-value (0-1), x/y/z position, and time (w) substituted in and must resolve to a vector on the surface

Both Parametric objects can be killed by use of the [[Kill BIT|Kill Bit]] pattern

## Related patterns (same book page)

- [[Symbolic Purification]] `str → expression`
- [[Substitution Exaltation]] `expr, expr, expr | num → expr`
- [[Equality Purification]] `expr, expr | num → expr`
- [[Piecewise Exaltation]] `expr, expr|num, expr|num → expr`
- [[Derivation Purification]] `expr, expr → expr`
- [[Neo's Exaltation]] `num, num, expr → matrix`
- [[Parametric Line]] `vec, expr → entity`

## Sources

- 📖 **Guidebook:** [[Expressions]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Parametric Surface](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions@complexhex:summon/surface)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpawnParametricSurface` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:summon/surface`
