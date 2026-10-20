---
aliases: ["Sextant Decomposition"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/decomposition, topic/sensing]
kind: "Pattern"
family: "Decomposition"
topics: ["Sensing & Raycasts"]
mod: "Hextra Patterns"
op_id: "hextrapats:entity/look_angles"
signature: "waa"
start_direction: "EAST"
input: "entity"
output: "number, number"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hextra Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sextant Decomposition

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:1.6s" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="51.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,78.3 51.0,83.8 51.0,72.8" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number, number` ([[Number]])
- **ID:** `hextrapats:entity/look_angles`

```text
before   … │ entity           ◂ top
after    … │ number │ number  ◂ top
```

## Description

Transforms an entity on the [[The Stack|stack]] into the pitch and yaw of its looking direction in radians.

I've seen some texts mention something called **F3**.

For yaw, north is pi, south is 0, east is -pi/2, and west is pi/2.

For pitch, pi/2 is upwards is pi/2, and downwards is -pi/2.

## Related patterns (same book page)

- [[Archer's Distillation II]] `vec, vec → vec | null`
- [[Architect's Distillation II]] `vec, vec → vec | null`
- [[Sextant Purification]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Hextra Basic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEntityLookAngles` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:entity/look_angles`
