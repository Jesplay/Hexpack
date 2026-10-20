---
aliases: ["Handwriting Distillation"]
tags: [hexcasting/pattern, mod/hexical, family/distillation, topic/control-flow]
kind: "Pattern"
family: "Distillation"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexical"
op_id: "hexical:draw_pattern"
signature: "eadqqqa"
start_direction: "NORTH_EAST"
input: "pattern"
output: "list of vectors"
cost: ""
iotas_in: ["[[Pattern]]"]
iotas_out: ["[[Vector]]", "[[List]]"]
entry: "[[Pattern Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Handwriting Distillation

◇ **Pattern** · [[Hexical]] · 📖 [[Pattern Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,145.5 41.9,115.3 76.9,115.3 59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 76.9,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:280;--dur:2.2s" points="24.5,145.5 41.9,115.3 76.9,115.3 59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 76.9,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="145.5" x2="30.1" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="34.1,128.9 33.4,137.8 26.7,133.9" fill="#fc77be"/><circle cx="24.5" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eadqqqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern` ([[Pattern]])
- **Leaves:** `list of vectors` ([[Vector]], [[List]])
- **ID:** `hexical:draw_pattern`

```text
before   … │ pattern          ◂ top
after    … │ list of vectors  ◂ top
```

## Description

Takes a pattern and produces a normalized list of vectors. The z-component of these vectors is zero. I feel this may be useful for artistic **Hexes**.

## Related patterns (same book page)

- [[Glyphmaker's Distillation]] `pattern, number → pattern`
- [[Congruence Distillation]] `pattern, pattern → boolean`
- [[Similarity Distillation]] `pattern, pattern → boolean`
- [[Chirographer's Purification]] `pattern → list of numbers`
- [[Calligrapher's Purification]] `list of numbers → pattern`

## Sources

- 📖 **Guidebook:** [[Pattern Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Handwriting Distillation](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/pattern_manipulation@hexical:draw_pattern)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDrawPattern` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:draw_pattern`
