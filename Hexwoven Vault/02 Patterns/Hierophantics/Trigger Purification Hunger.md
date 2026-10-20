---
aliases: ["Trigger Purification: Hunger"]
tags: [hexcasting/pattern, mod/hierophantics, family/purification, topic/minds]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:make_hunger_trigger"
signature: "qqqqqawwaedwda"
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

# Trigger Purification: Hunger

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Triggers]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="22.0,76.6 37.8,103.8 69.3,103.8 85.0,76.6 69.3,49.3 37.8,49.3 22.0,76.6 53.5,76.6 85.0,76.6 116.5,76.6 100.7,49.3 116.5,22.0 132.2,49.3 148.0,76.6 116.5,76.6 132.2,103.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="22.0,76.6 37.8,103.8 69.3,103.8 85.0,76.6 69.3,49.3 37.8,49.3 22.0,76.6 53.5,76.6 85.0,76.6 116.5,76.6 100.7,49.3 116.5,22.0 132.2,49.3 148.0,76.6 116.5,76.6 132.2,103.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="76.6" x2="27.1" y2="85.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,91.6 24.1,87.0 30.1,83.6" fill="#fc77be"/><circle cx="22.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqawwaedwda` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `trigger` ([[Trigger]])
- **ID:** `hierophantics:make_hunger_trigger`

```text
before   … │ num      ◂ top
after    … │ trigger  ◂ top
```

## Description

Creates a trigger that activates when my hunger drops below the specified value. A full hunger bar has a value of 20.

## Related patterns (same book page)

- [[Trigger Reflection Damage]] `— → trigger`
- [[Trigger Reflection Damage Type]] `— → trigger`
- [[Trigger Purification Health]] `num → trigger`
- [[Trigger Purification Breath]] `num → trigger`
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
- 🔧 **Pattern ID:** `hierophantics:make_hunger_trigger`
