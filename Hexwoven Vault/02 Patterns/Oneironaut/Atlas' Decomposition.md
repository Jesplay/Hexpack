---
aliases: ["Atlas' Decomposition"]
tags: [hexcasting/pattern, mod/oneironaut, family/decomposition, topic/dimensions]
kind: "Pattern"
family: "Decomposition"
topics: ["Dimensions & Worlds"]
mod: "Oneironaut"
op_id: "oneironaut:getdimheight"
signature: "awqqqwqwqwqwqwq"
start_direction: "NORTH_EAST"
input: "Imprint"
output: "num, num"
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

# Atlas' Decomposition

◇ **Pattern** · [[Oneironaut]] · 📖 [[Dimension Iotas]] · Patterns

**Topics:** [[Dimensions & Worlds (Topic)|Dimensions & Worlds]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="100.7,103.8 116.5,76.6 85.0,76.6 53.5,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.5s" points="100.7,103.8 116.5,76.6 85.0,76.6 53.5,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="100.7" y1="103.8" x2="105.8" y2="95.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="109.4,88.8 108.8,96.8 102.8,93.4" fill="#fc77be"/><circle cx="100.7" cy="103.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awqqqwqwqwqwqwq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `Imprint` ([[Imprint]])
- **Leaves:** `num, num` ([[Number]])
- **ID:** `oneironaut:getdimheight`

```text
before   … │ Imprint    ◂ top
after    … │ num │ num  ◂ top
```

## Description

Accepts a Spatial Imprint, and returns the lowest and highest altitudes at which blocks can exist in the corresponding dimension.

## Related patterns (same book page)

- [[Spatial Reflection]] `— → Imprint`
- [[Spatial Reflection II]] `— → Imprint | NULL`
- [[Homeland Reflection]] `— → Imprint`
- [[Inferno Reflection]] `— → Imprint`
- [[Ratio Purification]] `Imprint → num`

## Sources

- 📖 **Guidebook:** [[Dimension Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Atlas' Decomposition](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#patterns/dim_iotas@oneironaut:getdimheight)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDimHeight` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:getdimheight`
