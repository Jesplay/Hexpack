---
aliases: ["Congruence Distillation"]
tags: [hexcasting/pattern, mod/hexical, family/distillation, topic/control-flow]
kind: "Pattern"
family: "Distillation"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexical"
op_id: "hexical:congruent"
signature: "aaqd"
start_direction: "EAST"
input: "pattern, pattern"
output: "boolean"
cost: ""
iotas_in: ["[[Pattern]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Pattern Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Congruence Distillation

◇ **Pattern** · [[Hexical]] · 📖 [[Pattern Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,85.0 119.4,85.0 92.3,38.0 65.1,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="65.1,85.0 119.4,85.0 92.3,38.0 65.1,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="85.0" x2="82.5" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="95.0,85.0 82.5,91.0 82.5,79.0" fill="#fc77be"/><circle cx="65.1" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern, pattern` ([[Pattern]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexical:congruent`

```text
before   … │ pattern │ pattern  ◂ top
after    … │ boolean            ◂ top
```

## Description

Pushes whether two patterns have the same shape, stroke order, and orientation.

## Related patterns (same book page)

- [[Glyphmaker's Distillation]] `pattern, number → pattern`
- [[Similarity Distillation]] `pattern, pattern → boolean`
- [[Chirographer's Purification]] `pattern → list of numbers`
- [[Calligrapher's Purification]] `list of numbers → pattern`
- [[Handwriting Distillation]] `pattern → list of vectors`

## Sources

- 📖 **Guidebook:** [[Pattern Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Congruence Distillation](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/pattern_manipulation@hexical:congruent)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCongruentPattern` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:congruent`
