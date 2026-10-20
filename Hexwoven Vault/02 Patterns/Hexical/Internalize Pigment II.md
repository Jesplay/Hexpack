---
aliases: ["Internalize Pigment II"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:take_on_pigment"
signature: "weeeweeqeeeewqaqweeee"
start_direction: "EAST"
input: "pigment"
output: ""
cost: ""
iotas_in: ["[[Pigment]]"]
iotas_out: []
entry: "[[Pigment Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Internalize Pigment II

◇ **Pattern** · [[Hexical]] · 📖 [[Pigment Manipulation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,85.0 76.9,85.0 111.8,85.0 129.3,115.3 111.8,145.5 76.9,145.5 41.9,145.5 24.5,115.3 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 76.9,85.0 59.4,54.7 76.9,24.5 111.8,24.5 129.3,54.7 111.8,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:769;--dur:6.0s" points="41.9,85.0 76.9,85.0 111.8,85.0 129.3,115.3 111.8,145.5 76.9,145.5 41.9,145.5 24.5,115.3 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 76.9,85.0 59.4,54.7 76.9,24.5 111.8,24.5 129.3,54.7 111.8,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="85.0" x2="53.1" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="61.2,85.0 53.1,88.8 53.1,81.2" fill="#fc77be"/><circle cx="41.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeweeqeeeewqaqweeee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pigment` ([[Pigment]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:take_on_pigment`

```text
before   … │ pigment  ◂ top
after    …            ◂ top
```

## Description

Internalizes a [[Pigments|pigment]] [[Iotas|iota]].

## Related patterns (same book page)

- [[Pigment Purification]] `dye/entity → pigment`
- [[Pigment Exaltation]] `pigment, vec, num → vec`

## Sources

- 📖 **Guidebook:** [[Pigment Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Internalize Pigment II](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/pigments@hexical:take_on_pigment)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTakeOnPigment` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:take_on_pigment`
