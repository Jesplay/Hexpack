---
aliases: ["Trigger Purification: Breath"]
tags: [hexcasting/pattern, mod/hierophantics, family/purification, topic/minds]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:make_breath_trigger"
signature: "qqqqqawweqqqqqaa"
start_direction: "SOUTH_EAST"
input: "num"
output: "trigger"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Trigger]]"]
entry: "[[Hierophantic Triggers]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Trigger Purification: Breath

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Triggers]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 45.2,41.6 71.7,41.6 98.3,41.6 111.6,64.6 138.1,64.6 151.4,41.6 138.1,18.6 111.6,18.6 98.3,41.6 124.8,41.6 111.6,18.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:452;--dur:4.8s" points="18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 45.2,41.6 71.7,41.6 98.3,41.6 111.6,64.6 138.1,64.6 151.4,41.6 138.1,18.6 111.6,18.6 98.3,41.6 124.8,41.6 111.6,18.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.6" y1="41.6" x2="22.8" y2="49.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="25.9,54.2 20.3,50.4 25.4,47.5" fill="#fc77be"/><circle cx="18.6" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqawweqqqqqaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `trigger` ([[Trigger]])
- **ID:** `hierophantics:make_breath_trigger`

```text
before   … │ num      ◂ top
after    … │ trigger  ◂ top
```

## Description

Creates a trigger that activates when my breath drops below the specified value. A full bar of bubbles has a value of 10.

## Related patterns (same book page)

- [[Trigger Reflection Damage]] `— → trigger`
- [[Trigger Reflection Damage Type]] `— → trigger`
- [[Trigger Purification Health]] `num → trigger`
- [[Trigger Purification Hunger]] `num → trigger`
- [[Trigger Purification Velocity]] `num → trigger`
- [[Trigger Purification Fall]] `num → trigger`
- [[Trigger Reflection Drop]] `— → trigger`
- [[Trigger Reflection Attack]] `— → trigger`
- [[Trigger Reflection Break]] `— → trigger`
- [[Trigger Reflection Jump]] `— → trigger`
- [[Trigger Reflection Teleport]] `— → trigger`
- [[Inversion Purification]] `trigger → trigger`

## Sources

- 📖 **Guidebook:** [[Hierophantic Triggers]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hierophantics:make_breath_trigger`
