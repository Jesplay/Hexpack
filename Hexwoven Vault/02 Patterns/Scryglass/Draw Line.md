---
aliases: ["Draw Line"]
tags: [hexcasting/pattern, mod/scryglass, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Scryglass"
op_id: "scryglass:draw_line"
signature: "aaqdwdwdeww"
start_direction: "NORTH_EAST"
input: "index, vector, vector, vector"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Viewforming]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Draw Line

◇ **Pattern** · [[Scryglass]] · 📖 [[Viewforming]] · Patterns

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,54.7 76.9,24.5 41.9,24.5 59.4,54.7 94.4,54.7 76.9,85.0 59.4,115.3 41.9,85.0 24.5,54.7 59.4,54.7 76.9,85.0 94.4,115.3 111.8,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="59.4,54.7 76.9,24.5 41.9,24.5 59.4,54.7 94.4,54.7 76.9,85.0 59.4,115.3 41.9,85.0 24.5,54.7 59.4,54.7 76.9,85.0 94.4,115.3 111.8,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="54.7" x2="65.0" y2="45.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,38.1 68.3,47.0 61.7,43.1" fill="#fc77be"/><circle cx="59.4" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqdwdwdeww` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `index, vector, vector, vector` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `scryglass:draw_line`

```text
before   … │ index │ vector │ vector │ vector  ◂ top
after    …                                     ◂ top
```

## Description

Takes an index, a position, a size, and a color. Conjures an **Icon** that joins between the two with a thin line.

## Related patterns (same book page)

- [[Periphery Reflection]] `— → vector`
- [[Icons Reflection]] `— → list of indices`
- [[Clear Icon]] `index → —`
- [[Draw Text]] `index, vector, number, any → —`
- [[Rotate Icon]] `index, number → —`
- [[Scale Icon]] `index, number → —`
- [[Draw Rectangle]] `index, vector, vector, vector → —`

## Sources

- 📖 **Guidebook:** [[Viewforming]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Draw Line](https://scryglass.hexxy.media/v/latest/main/en_us/#scryglass@scryglass:draw_line)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDrawLine` is not a `SpellAction`
- 🔧 **Pattern ID:** `scryglass:draw_line`
