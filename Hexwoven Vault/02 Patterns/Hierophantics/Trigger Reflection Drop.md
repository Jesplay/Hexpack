---
aliases: ["Trigger Reflection: Drop"]
tags: [hexcasting/pattern, mod/hierophantics, family/reflection, topic/minds]
kind: "Pattern"
family: "Reflection"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:make_drop_trigger"
signature: "qqqqqawweaqaddwd"
start_direction: "SOUTH_EAST"
input: ""
output: "trigger"
cost: ""
iotas_in: []
iotas_out: ["[[Trigger]]"]
entry: "[[Hierophantic Triggers]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Trigger Reflection: Drop

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Triggers]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="90" viewBox="0 0 170.0 90.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.8" cy="20.2" r="2"/><circle cx="34.6" cy="20.2" r="2"/><circle cx="63.4" cy="20.2" r="2"/><circle cx="92.2" cy="20.2" r="2"/><circle cx="121.0" cy="20.2" r="2"/><circle cx="149.8" cy="20.2" r="2"/><circle cx="20.2" cy="45.1" r="2"/><circle cx="49.0" cy="45.1" r="2"/><circle cx="77.8" cy="45.1" r="2"/><circle cx="106.6" cy="45.1" r="2"/><circle cx="135.4" cy="45.1" r="2"/><circle cx="164.2" cy="45.1" r="2"/><circle cx="5.8" cy="70.1" r="2"/><circle cx="34.6" cy="70.1" r="2"/><circle cx="63.4" cy="70.1" r="2"/><circle cx="92.2" cy="70.1" r="2"/><circle cx="121.0" cy="70.1" r="2"/><circle cx="149.8" cy="70.1" r="2"/></g><polyline class="hexpat-base" points="20.2,45.1 34.6,70.1 63.4,70.1 77.8,45.1 63.4,20.2 34.6,20.2 20.2,45.1 49.0,45.1 77.8,45.1 106.6,45.1 121.0,70.1 135.4,45.1 121.0,20.2 106.6,45.1 92.2,20.2 121.0,20.2 149.8,20.2 135.4,45.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:490;--dur:4.8s" points="20.2,45.1 34.6,70.1 63.4,70.1 77.8,45.1 63.4,20.2 34.6,20.2 20.2,45.1 49.0,45.1 77.8,45.1 106.6,45.1 121.0,70.1 135.4,45.1 121.0,20.2 106.6,45.1 92.2,20.2 121.0,20.2 149.8,20.2 135.4,45.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.2" y1="45.1" x2="24.8" y2="53.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="28.1,58.8 22.0,54.7 27.5,51.5" fill="#fc77be"/><circle cx="20.2" cy="45.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqawweaqaddwd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `trigger` ([[Trigger]])
- **ID:** `hierophantics:make_drop_trigger`

```text
before   …            ◂ top
after    … │ trigger  ◂ top
```

## Description

Creates a trigger that activates whenever I drop an item. This does **not** include involuntary dropping, such as on death or when using [[Greater Teleport (Book Entry)|Greater Teleport]].

## Related patterns (same book page)

- [[Trigger Reflection Damage]] `— → trigger`
- [[Trigger Reflection Damage Type]] `— → trigger`
- [[Trigger Purification Health]] `num → trigger`
- [[Trigger Purification Breath]] `num → trigger`
- [[Trigger Purification Hunger]] `num → trigger`
- [[Trigger Purification Velocity]] `num → trigger`
- [[Trigger Purification Fall]] `num → trigger`
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
- 🔧 **Pattern ID:** `hierophantics:make_drop_trigger`
