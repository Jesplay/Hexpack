---
aliases: ["Spatial Reflection"]
tags: [hexcasting/pattern, mod/oneironaut, family/reflection, topic/dimensions]
kind: "Pattern"
family: "Reflection"
topics: ["Dimensions & Worlds"]
mod: "Oneironaut"
op_id: "oneironaut:getdim1"
signature: "wqwqwqwqwqwaeqqe"
start_direction: "WEST"
input: ""
output: "Imprint"
cost: "Costs a negligible amount of media."
iotas_in: []
iotas_out: ["[[Imprint]]"]
entry: "[[Dimension Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Spatial Reflection

◇ **Pattern** · [[Oneironaut]] · 📖 [[Dimension Iotas]] · Patterns

**Topics:** [[Dimensions & Worlds (Topic)|Dimensions & Worlds]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:535;--dur:4.8s" points="116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 53.5,131.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="116.5" y1="22.0" x2="106.4" y2="22.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="99.2,22.0 106.4,18.6 106.4,25.5" fill="#fc77be"/><circle cx="116.5" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqwqwaeqqe` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `Imprint` ([[Imprint]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `oneironaut:getdim1`

```text
before   …            ◂ top
after    … │ Imprint  ◂ top
```

## Description

Returns a Spatial Imprint representing the dimension I am currently in. Costs a negligible amount of [[Media|media]].

## Related patterns (same book page)

- [[Spatial Reflection II]] `— → Imprint | NULL`
- [[Homeland Reflection]] `— → Imprint`
- [[Inferno Reflection]] `— → Imprint`
- [[Atlas' Decomposition]] `Imprint → num, num`
- [[Ratio Purification]] `Imprint → num`

## Sources

- 📖 **Guidebook:** [[Dimension Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Spatial Reflection](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#patterns/dim_iotas@oneironaut:getdim1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetDim` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:getdim1`
