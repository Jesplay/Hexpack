---
aliases: ["Pigment Exaltation"]
tags: [hexcasting/pattern, mod/hexical, family/exaltation, topic/visuals]
kind: "Pattern"
family: "Exaltation"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:sample_pigment"
signature: "edewqaqqqqqwqqq"
start_direction: "SOUTH_EAST"
input: "pigment, vec, num"
output: "vec"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Pigment]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Pigment Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Pigment Exaltation

◇ **Pattern** · [[Hexical]] · 📖 [[Pigment Manipulation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,85.0 94.4,115.3 76.9,145.5 59.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 76.9,85.0 111.8,85.0 129.3,54.7 111.8,24.5 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 76.9,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:559;--dur:4.5s" points="76.9,85.0 94.4,115.3 76.9,145.5 59.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 76.9,85.0 111.8,85.0 129.3,54.7 111.8,24.5 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 76.9,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="85.0" x2="82.5" y2="94.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.5,101.6 79.2,96.6 85.8,92.8" fill="#fc77be"/><circle cx="76.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edewqaqqqqqwqqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pigment, vec, num` ([[Number]], [[Vector]], [[Pigment]])
- **Leaves:** `vec` ([[Vector]])
- **ID:** `hexical:sample_pigment`

```text
before   … │ pigment │ vec │ num  ◂ top
after    … │ vec                  ◂ top
```

## Description

Samples the [[Pigments|pigment]] at a certain place and time as a vector, where each component is from 0 to 1 and represents the red, green, and blueness respectively.

## Related patterns (same book page)

- [[Pigment Purification]] `dye/entity → pigment`
- [[Internalize Pigment II]] `pigment → —`

## Sources

- 📖 **Guidebook:** [[Pigment Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Pigment Exaltation](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/pigments@hexical:sample_pigment)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSamplePigment` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:sample_pigment`
