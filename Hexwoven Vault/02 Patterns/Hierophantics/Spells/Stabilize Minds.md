---
aliases: ["Stabilize Minds"]
tags: [hexcasting/spell, mod/hierophantics, family/spell, topic/minds]
kind: "Spell"
family: "Spell"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:reenable_minds"
signature: "qaqqaeawaeaeqqqeqqwwqqeqqq"
start_direction: "NORTH_EAST"
input: ""
output: ""
cost: "Costs one Amethyst Dust per mind restored."
iotas_in: []
iotas_out: []
entry: "[[Hierophantic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Stabilize Minds

✦ **Spell** · [[Hierophantics]] · 📖 [[Hierophantic Patterns]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/><circle cx="22.0" cy="103.8" r="2"/><circle cx="53.5" cy="103.8" r="2"/><circle cx="85.0" cy="103.8" r="2"/><circle cx="116.5" cy="103.8" r="2"/><circle cx="148.0" cy="103.8" r="2"/><circle cx="6.3" cy="131.1" r="2"/><circle cx="37.8" cy="131.1" r="2"/><circle cx="69.3" cy="131.1" r="2"/><circle cx="100.7" cy="131.1" r="2"/><circle cx="132.2" cy="131.1" r="2"/><circle cx="163.7" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,103.8 100.7,76.6 85.0,49.3 69.3,76.6 85.0,103.8 116.5,103.8 100.7,76.6 116.5,49.3 85.0,49.3 53.5,49.3 69.3,76.6 53.5,103.8 85.0,103.8 100.7,131.1 132.2,131.1 148.0,103.8 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 22.0,103.8 37.8,131.1 69.3,131.1 85.0,103.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:850;--dur:6.0s" points="85.0,103.8 100.7,76.6 85.0,49.3 69.3,76.6 85.0,103.8 116.5,103.8 100.7,76.6 116.5,49.3 85.0,49.3 53.5,49.3 69.3,76.6 53.5,103.8 85.0,103.8 100.7,131.1 132.2,131.1 148.0,103.8 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 22.0,103.8 37.8,131.1 69.3,131.1 85.0,103.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="103.8" x2="90.0" y2="95.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,88.8 93.0,96.8 87.0,93.4" fill="#fc77be"/><circle cx="85.0" cy="103.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqaeawaeaeqqqeqqwwqqeqqq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **Cost:** Costs one Amethyst Dust per mind restored.
- **ID:** `hierophantics:reenable_minds`

## Description

When I die, my embedded minds become unstable and cease to function. This spell restores them to working order. Costs one [[Amethyst|Amethyst Dust]] per mind restored.

## Related patterns (same book page)

- [[Hierophant's Reflection]] `— → [mind]`
- [[Payload Gambit]] `mind, [pattern] | null → —`
- [[Payload Purification]] `mind → [pattern] | null`
- [[Trigger Gambit]] `mind, trigger | null → —`
- [[Trigger Purification]] `mind → trigger | null`
- [[Mute Mind]] `mind, bool → —`
- [[Free Mind]] `mind → —`

## Sources

- 📖 **Guidebook:** [[Hierophantic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpReenableMinds` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:reenable_minds`
