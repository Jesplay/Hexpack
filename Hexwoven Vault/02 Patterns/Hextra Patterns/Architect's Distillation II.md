---
aliases: ["Architect's Distillation II"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Hextra Patterns"
op_id: "hextrapats:raycast/outaxis"
signature: "weddqdd"
start_direction: "EAST"
input: "vec, vec"
output: "vec | null"
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]", "[[Null]]"]
entry: "[[Hextra Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Architect's Distillation II

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,106.0 95.9,106.0 117.7,68.3 95.9,30.5 139.5,30.5 117.7,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,106.0 95.9,106.0 117.7,68.3 95.9,30.5 139.5,30.5 117.7,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="68.3" x2="44.5" y2="68.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="54.5,68.3 44.5,73.1 44.5,63.5" fill="#fc77be"/><circle cx="30.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weddqdd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `vec | null` ([[Vector]], [[Null]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `hextrapats:raycast/outaxis`

## Description

Like [[Architect's Distillation]], but is able to hit collisionless blocks like tall grass. Costs a negligible amount of **[[Media|media]]**.

## Related patterns (same book page)

- [[Archer's Distillation II]] `vec, vec → vec | null`
- [[Sextant Decomposition]] `entity → number, number`
- [[Sextant Purification]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Hextra Basic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpOutlineAxisRaycast` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:raycast/outaxis`
