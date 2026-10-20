---
aliases: ["Scout's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/sensing]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:raycast/entity"
signature: "weaqa"
start_direction: "EAST"
input: "vector, vector"
output: "entity | null"
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]", "[[Null]]"]
entry: "[[Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Scout's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:232;--dur:1.7s" points="27.0,60.5 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weaqa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `entity | null` ([[Entity]], [[Null]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `hexcasting:raycast/entity`

## Description

Like [[Archer's Distillation]], but instead returns the entity I am looking at. Costs a negligible amount of **[[Media|media]]**.

## Example

Same setup as [[Archer's Distillation]], but returns the **entity** you're looking at. Great for targeting mobs or friends: Scout's Distillation → [[Impulse]] or [[White Sun's Nadir]]…

## Related patterns (same book page)

- [[Mind's Reflection]] `— → entity | null`
- [[Compass' Purification]] `entity → vector`
- [[Compass' Purification II]] `entity → vector`
- [[Alidade's Purification]] `entity → vector`
- [[Archer's Distillation]] `vector, vector → vector | null`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Reveal]] `any → any`
- [[Stadiometer's Purification]] `entity → num`
- [[Pace Purification]] `entity → vector`
- [[Media Reflection (Hex Casting)]] `— → num`

## Sources

- 📖 **Guidebook:** [[Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Scout's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics@hexcasting:raycast/entity)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEntityRaycast` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:raycast/entity`
