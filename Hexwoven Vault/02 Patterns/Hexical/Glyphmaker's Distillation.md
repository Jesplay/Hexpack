---
aliases: ["Glyphmaker's Distillation"]
tags: [hexcasting/pattern, mod/hexical, family/distillation, topic/control-flow]
kind: "Pattern"
family: "Distillation"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexical"
op_id: "hexical:shuffle_pattern"
signature: "aqqqdae"
start_direction: "NORTH_EAST"
input: "pattern, number"
output: "pattern"
cost: ""
iotas_in: ["[[Number]]", "[[Pattern]]"]
iotas_out: ["[[Pattern]]"]
entry: "[[Pattern Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Glyphmaker's Distillation

◇ **Pattern** · [[Hexical]] · 📖 [[Pattern Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 76.9,85.0 59.4,115.3 94.4,115.3 111.8,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:280;--dur:2.2s" points="59.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 76.9,85.0 59.4,115.3 94.4,115.3 111.8,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="54.7" x2="65.0" y2="45.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,38.1 68.3,47.0 61.7,43.1" fill="#fc77be"/><circle cx="59.4" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqqqdae` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern, number` ([[Number]], [[Pattern]])
- **Leaves:** `pattern` ([[Pattern]])
- **ID:** `hexical:shuffle_pattern`

```text
before   … │ pattern │ number  ◂ top
after    … │ pattern           ◂ top
```

## Description

Uses the number to shuffle the pattern into one of the same shape, but different stroke order. My notes hint that ancient **Hexcasters** used this for some *grand library*.

## Related patterns (same book page)

- [[Congruence Distillation]] `pattern, pattern → boolean`
- [[Similarity Distillation]] `pattern, pattern → boolean`
- [[Chirographer's Purification]] `pattern → list of numbers`
- [[Calligrapher's Purification]] `list of numbers → pattern`
- [[Handwriting Distillation]] `pattern → list of vectors`

## Sources

- 📖 **Guidebook:** [[Pattern Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Glyphmaker's Distillation](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/pattern_manipulation@hexical:shuffle_pattern)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpShufflePattern` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:shuffle_pattern`
