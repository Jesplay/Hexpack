---
aliases: ["Homeland Reflection"]
tags: [hexcasting/pattern, mod/oneironaut, family/reflection, topic/dimensions]
kind: "Pattern"
family: "Reflection"
topics: ["Dimensions & Worlds"]
mod: "Oneironaut"
op_id: "oneironaut:getdim/overworld"
signature: "wqwqwqwqwqwawedewdwedew"
start_direction: "NORTH_EAST"
input: ""
output: "Imprint"
cost: "Costs one-tenth of an amethyst dust."
iotas_in: []
iotas_out: ["[[Imprint]]"]
entry: "[[Dimension Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Homeland Reflection

◇ **Pattern** · [[Oneironaut]] · 📖 [[Dimension Iotas]] · Patterns

**Topics:** [[Dimensions & Worlds (Topic)|Dimensions & Worlds]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 100.7,103.8 85.0,76.6 100.7,49.3 116.5,76.6 100.7,103.8 85.0,131.1 69.3,103.8 53.5,76.6 69.3,49.3 85.0,76.6 69.3,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:756;--dur:6.0s" points="116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 100.7,103.8 85.0,76.6 100.7,49.3 116.5,76.6 100.7,103.8 85.0,131.1 69.3,103.8 53.5,76.6 69.3,49.3 85.0,76.6 69.3,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="116.5" y1="131.1" x2="121.5" y2="122.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="125.1,116.1 124.5,124.1 118.5,120.6" fill="#fc77be"/><circle cx="116.5" cy="131.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqwqwawedewdwedew` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `Imprint` ([[Imprint]])
- **Cost:** Costs one-tenth of an amethyst dust.
- **ID:** `oneironaut:getdim/overworld`

```text
before   …            ◂ top
after    … │ Imprint  ◂ top
```

## Description

Returns a Spatial Imprint representing the overworld. Costs one-tenth of an amethyst dust.

## Related patterns (same book page)

- [[Spatial Reflection]] `— → Imprint`
- [[Spatial Reflection II]] `— → Imprint | NULL`
- [[Inferno Reflection]] `— → Imprint`
- [[Atlas' Decomposition]] `Imprint → num, num`
- [[Ratio Purification]] `Imprint → num`

## Sources

- 📖 **Guidebook:** [[Dimension Iotas]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpecificDim` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:getdim/overworld`
