---
aliases: ["Ratio Purification"]
tags: [hexcasting/pattern, mod/oneironaut, family/purification, topic/dimensions]
kind: "Pattern"
family: "Purification"
topics: ["Dimensions & Worlds"]
mod: "Oneironaut"
op_id: "oneironaut:getdimscale"
signature: "wawawqwqwqwqwqw"
start_direction: "NORTH_WEST"
input: "Imprint"
output: "num"
cost: ""
iotas_in: ["[[Imprint]]"]
iotas_out: ["[[Number]]"]
entry: "[[Dimension Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Ratio Purification

◇ **Pattern** · [[Oneironaut]] · 📖 [[Dimension Iotas]] · Patterns

**Topics:** [[Dimensions & Worlds (Topic)|Dimensions & Worlds]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="116.5,131.1 100.7,103.8 85.0,76.6 69.3,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.5s" points="116.5,131.1 100.7,103.8 85.0,76.6 69.3,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="116.5" y1="131.1" x2="111.4" y2="122.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.8,116.1 114.4,120.6 108.4,124.1" fill="#fc77be"/><circle cx="116.5" cy="131.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawawqwqwqwqwqw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `Imprint` ([[Imprint]])
- **Leaves:** `num` ([[Number]])
- **ID:** `oneironaut:getdimscale`

```text
before   … │ Imprint  ◂ top
after    … │ num      ◂ top
```

## Description

Accepts a Spatial Imprint, and returns how many blocks you'd have to travel in the overworld to travel the equivalent of one block in the input dimension.

## Related patterns (same book page)

- [[Spatial Reflection]] `— → Imprint`
- [[Spatial Reflection II]] `— → Imprint | NULL`
- [[Homeland Reflection]] `— → Imprint`
- [[Inferno Reflection]] `— → Imprint`
- [[Atlas' Decomposition]] `Imprint → num, num`

## Sources

- 📖 **Guidebook:** [[Dimension Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Ratio Purification](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#patterns/dim_iotas@oneironaut:getdimscale)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDimScale` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:getdimscale`
