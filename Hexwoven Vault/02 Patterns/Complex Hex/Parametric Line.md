---
aliases: ["Parametric Line"]
tags: [hexcasting/pattern, mod/complex-hex, family/other, topic/math]
kind: "Pattern"
family: "Other"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:summon/line"
signature: "wdwddwq"
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

# Parametric Line

◇ **Pattern** · [[Complex Hex]] · 📖 [[Expressions]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,29.8 93.5,66.6 114.8,103.4 72.3,103.4 29.8,103.4 51.0,66.6 72.3,103.4 93.5,140.2 136.1,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:340;--dur:2.2s" points="72.3,29.8 93.5,66.6 114.8,103.4 72.3,103.4 29.8,103.4 51.0,66.6 72.3,103.4 93.5,140.2 136.1,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="29.8" x2="79.1" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.0,50.0 75.0,43.9 83.1,39.2" fill="#fc77be"/><circle cx="72.3" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwddwq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, expr` ([[Vector]], [[Expression]])
- **Leaves:** `entity` ([[Entity]])
- **ID:** `complexhex:summon/line`

```text
before   … │ vec │ expr  ◂ top
after    … │ entity      ◂ top
```

## Description

Summons a parametric line at the given position. The expression given will have a t-value (0-1), x/y/z position, and time (w) substituted in and must resolve to a vector on the line.

## Related patterns (same book page)

- [[Symbolic Purification]] `str → expression`
- [[Substitution Exaltation]] `expr, expr, expr | num → expr`
- [[Equality Purification]] `expr, expr | num → expr`
- [[Piecewise Exaltation]] `expr, expr|num, expr|num → expr`
- [[Derivation Purification]] `expr, expr → expr`
- [[Neo's Exaltation]] `num, num, expr → matrix`
- [[Parametric Surface]] `vec, expr → entity`

## Sources

- 📖 **Guidebook:** [[Expressions]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Parametric Line](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions@complexhex:summon/line)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpawnParametricLine` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:summon/line`
