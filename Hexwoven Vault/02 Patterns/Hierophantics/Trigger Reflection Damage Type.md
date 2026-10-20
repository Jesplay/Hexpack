---
aliases: ["Trigger Reflection: Damage Type"]
tags: [hexcasting/pattern, mod/hierophantics, family/reflection, topic/minds]
kind: "Pattern"
family: "Reflection"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:make_damage_type_trigger"
signature: "qqqqqawwqdwaqqwawqwa"
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

# Trigger Reflection: Damage Type

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Triggers]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="77" viewBox="0 0 170.0 77.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.9" cy="17.2" r="2"/><circle cx="29.6" cy="17.2" r="2"/><circle cx="54.2" cy="17.2" r="2"/><circle cx="78.8" cy="17.2" r="2"/><circle cx="103.5" cy="17.2" r="2"/><circle cx="128.1" cy="17.2" r="2"/><circle cx="152.8" cy="17.2" r="2"/><circle cx="17.2" cy="38.6" r="2"/><circle cx="41.9" cy="38.6" r="2"/><circle cx="66.5" cy="38.6" r="2"/><circle cx="91.2" cy="38.6" r="2"/><circle cx="115.8" cy="38.6" r="2"/><circle cx="140.4" cy="38.6" r="2"/><circle cx="165.1" cy="38.6" r="2"/><circle cx="4.9" cy="59.9" r="2"/><circle cx="29.6" cy="59.9" r="2"/><circle cx="54.2" cy="59.9" r="2"/><circle cx="78.8" cy="59.9" r="2"/><circle cx="103.5" cy="59.9" r="2"/><circle cx="128.1" cy="59.9" r="2"/><circle cx="152.8" cy="59.9" r="2"/></g><polyline class="hexpat-base" points="17.2,38.6 29.6,59.9 54.2,59.9 66.5,38.6 54.2,17.2 29.6,17.2 17.2,38.6 41.9,38.6 66.5,38.6 91.2,38.6 103.5,17.2 115.8,38.6 128.1,59.9 140.4,38.6 128.1,17.2 103.5,17.2 78.8,17.2 91.2,38.6 103.5,59.9 128.1,59.9 152.8,59.9 140.4,38.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:517;--dur:5.9s" points="17.2,38.6 29.6,59.9 54.2,59.9 66.5,38.6 54.2,17.2 29.6,17.2 17.2,38.6 41.9,38.6 66.5,38.6 91.2,38.6 103.5,17.2 115.8,38.6 128.1,59.9 140.4,38.6 128.1,17.2 103.5,17.2 78.8,17.2 91.2,38.6 103.5,59.9 128.1,59.9 152.8,59.9 140.4,38.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="17.2" y1="38.6" x2="21.2" y2="45.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="24.0,50.3 18.8,46.8 23.5,44.1" fill="#fc77be"/><circle cx="17.2" cy="38.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqawwqdwaqqwawqwa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `trigger` ([[Trigger]])
- **ID:** `hierophantics:make_damage_type_trigger`

```text
before   …            ◂ top
after    … │ trigger  ◂ top
```

## Description

Creates a trigger that activates whenever I take damage, but only if the damage was of a specific type. The trigger will be attuned to whatever type of damage I last received.

## Related patterns (same book page)

- [[Trigger Reflection Damage]] `— → trigger`
- [[Trigger Purification Health]] `num → trigger`
- [[Trigger Purification Breath]] `num → trigger`
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
- 🔧 **Pattern ID:** `hierophantics:make_damage_type_trigger`
