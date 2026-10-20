---
aliases: ["Archer's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:raycast"
signature: "wqaawdd"
start_direction: "EAST"
input: "vector, vector"
output: "vector | null"
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]", "[[Null]]"]
entry: "[[Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Archer's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,30.5 95.9,30.5 117.7,68.3 139.5,106.0 95.9,106.0 117.7,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,30.5 95.9,30.5 117.7,68.3 139.5,106.0 95.9,106.0 117.7,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="68.3" x2="44.5" y2="68.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="54.5,68.3 44.5,73.1 44.5,63.5" fill="#fc77be"/><circle cx="30.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaawdd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `vector | null` ([[Vector]], [[Null]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `hexcasting:raycast`

## Description

Combines two vectors (a position and a direction) into the answer to the question: If I stood at the position and looked in the direction, what block would I be looking at? Costs a negligible amount of **[[Media|media]]**.

If it doesn't hit anything, the vectors will combine into **Null**.

A common sequence of patterns, the so-called "raycast mantra," is [[Mind's Reflection]], [[Compass' Purification]], [[Mind's Reflection]], [[Alidade's Purification|Alidade Purification]], [[Archer's Distillation]]. Together, they return the vector position of the block I am looking at.

## Example

**"The block I'm looking at"**, the most common building block of all:
1. [[Mind's Reflection]] → [[Compass' Purification]] (where the ray starts: your eyes)
2. [[Mind's Reflection]] → [[Alidade's Purification]] (which way it goes)
3. **Archer's Distillation** → the block position you're looking at (or [[Null]] if nothing is in range)

Follow it with [[Break Block]], [[Explosion]], [[Conjure Light]]…

## Related patterns (same book page)

- [[Mind's Reflection]] `— → entity | null`
- [[Compass' Purification]] `entity → vector`
- [[Compass' Purification II]] `entity → vector`
- [[Alidade's Purification]] `entity → vector`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Scout's Distillation]] `vector, vector → entity | null`
- [[Reveal]] `any → any`
- [[Stadiometer's Purification]] `entity → num`
- [[Pace Purification]] `entity → vector`
- [[Media Reflection (Hex Casting)]] `— → num`

## Sources

- 📖 **Guidebook:** [[Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Archer's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics@hexcasting:raycast)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpBlockRaycast` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:raycast`
