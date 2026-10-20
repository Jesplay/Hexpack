---
aliases: ["Schrödinger's Purification"]
tags: [hexcasting/pattern, mod/hexcellular, family/purification, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hexcellular"
op_id: "hexcellular:readonly_property"
signature: "aawa"
start_direction: "SOUTH_WEST"
input: "property"
output: "property"
cost: ""
iotas_in: ["[[Property]]"]
iotas_out: ["[[Property]]"]
entry: "[[Property Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Schrödinger's Purification

◇ **Pattern** · [[Hexcellular]] · 📖 [[Property Iota]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="146.5" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,85.0 65.1,132.0 119.4,132.0 92.3,85.0 65.1,38.0 38.0,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="92.3,85.0 65.1,132.0 119.4,132.0 92.3,85.0 65.1,38.0 38.0,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="85.0" x2="83.6" y2="100.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="77.3,110.9 78.4,97.1 88.8,103.0" fill="#fc77be"/><circle cx="92.3" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `property` ([[Property]])
- **Leaves:** `property` ([[Property]])
- **ID:** `hexcellular:readonly_property`

```text
before   … │ property  ◂ top
after    … │ property  ◂ top
```

## Description

Creates a read-only property [[Iotas|iota]] that refers to the same **Property** as the given property iota.

## Related patterns (same book page)

- [[Schrödinger's Reflection]] `— → property`
- [[Observation Purification]] `property → any`
- [[Schrödinger's Gambit]] `property, any → —`

## Sources

- 📖 **Guidebook:** [[Property Iota]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Schrödinger's Purification](https://hexcellular.hexxy.media/v/1.1.1/1.0.0/en_us/#properties@hexcellular:readonly_property)
- 🧩 **Mod page:** [Hexcellular on Modrinth](https://modrinth.com/mod/mN6t9Bt4)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadonlyProperty` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcellular:readonly_property`
