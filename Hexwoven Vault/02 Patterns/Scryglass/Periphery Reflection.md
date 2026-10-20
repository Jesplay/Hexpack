---
aliases: ["Periphery Reflection"]
tags: [hexcasting/pattern, mod/scryglass, family/reflection, topic/visuals]
kind: "Pattern"
family: "Reflection"
topics: ["Visuals, Sound & UI"]
mod: "Scryglass"
op_id: "scryglass:get_window_size"
signature: "aawawaa"
start_direction: "NORTH_EAST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Viewforming]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Periphery Reflection

◇ **Pattern** · [[Scryglass]] · 📖 [[Viewforming]] · Patterns

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,78.3 85.0,35.0 35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="60.0,78.3 85.0,35.0 35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="78.3" x2="68.0" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="73.7,54.5 72.8,67.2 63.2,61.7" fill="#fc77be"/><circle cx="60.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawawaa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `scryglass:get_window_size`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Pushes a vector whose respective components are the width and height of my vision, and the breadth of my vision in radians.

## Related patterns (same book page)

- [[Icons Reflection]] `— → list of indices`
- [[Clear Icon]] `index → —`
- [[Draw Text]] `index, vector, number, any → —`
- [[Rotate Icon]] `index, number → —`
- [[Scale Icon]] `index, number → —`
- [[Draw Rectangle]] `index, vector, vector, vector → —`
- [[Draw Line]] `index, vector, vector, vector → —`

## Sources

- 📖 **Guidebook:** [[Viewforming]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Periphery Reflection](https://scryglass.hexxy.media/v/latest/main/en_us/#scryglass@scryglass:get_window_size)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetWindowSize` is not a `SpellAction`
- 🔧 **Pattern ID:** `scryglass:get_window_size`
