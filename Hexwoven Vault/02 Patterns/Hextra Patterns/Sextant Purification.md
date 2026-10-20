---
aliases: ["Sextant Purification"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hextra Patterns"
op_id: "hextrapats:entity/body_yaw"
signature: "wdd"
start_direction: "EAST"
input: "entity"
output: "number"
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

# Sextant Purification

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="35.0,35.0 85.0,35.0 135.0,35.0 110.0,78.3 85.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:1.6s" points="35.0,35.0 85.0,35.0 135.0,35.0 110.0,78.3 85.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="35.0" x2="51.0" y2="35.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,35.0 51.0,40.5 51.0,29.5" fill="#fc77be"/><circle cx="35.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hextrapats:entity/body_yaw`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Transforms an entity on the [[The Stack|stack]] into the yaw of its body in radians. Yet again, mentions of [[Sextant Decomposition|F3]] turn up.

## Related patterns (same book page)

- [[Archer's Distillation II]] `vec, vec → vec | null`
- [[Architect's Distillation II]] `vec, vec → vec | null`
- [[Sextant Decomposition]] `entity → number, number`

## Sources

- 📖 **Guidebook:** [[Hextra Basic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEntityBodyYaw` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:entity/body_yaw`
