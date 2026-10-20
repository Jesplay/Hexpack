---
aliases: ["Confetti"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:confetti"
signature: "awddeqaedd"
start_direction: "EAST"
input: "vector, number/vector"
output: ""
cost: "Costs about half of one Amethyst Dust."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Hexical Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Confetti

✦ **Spell** · [[Hexical]] · 📖 [[Hexical Spells]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,103.4 114.8,103.4 93.5,66.6 72.3,29.8 114.8,29.8 93.5,66.6 51.0,66.6 29.8,103.4 72.3,103.4 93.5,140.2 51.0,140.2 72.3,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="72.3,103.4 114.8,103.4 93.5,66.6 72.3,29.8 114.8,29.8 93.5,66.6 51.0,66.6 29.8,103.4 72.3,103.4 93.5,140.2 51.0,140.2 72.3,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="103.4" x2="85.9" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="95.7,103.4 85.9,108.1 85.9,98.7" fill="#fc77be"/><circle cx="72.3" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awddeqaedd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number/vector` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about half of one Amethyst Dust.
- **ID:** `hexical:confetti`

```text
before   … │ vector │ number/vector  ◂ top
after    …                           ◂ top
```

## Description

Creates a loud bang similar to that of [[Pyrotechnics|Fireworks]] and a burst of colorful particles, either in a direction or in a radial blast. Costs about half of one [[Amethyst|Amethyst Dust]].

The particles are harmless and flutter through the air for quite long until they reach the ground. The first vector is a position and the second controls direction. If it is a number, it indicates speed to explode in in all directions; a vector makes the particles tend towards the given direction. Both the number and the vector's magnitude may not exceed 2.

*The rest is confetti.*

## Related patterns (same book page)

- [[Vibrate]] `vector, entity/vector, number → —`
- [[Sparkle]] `vector, vector, number → —`
- [[Crack Device]] `— → —`
- [[Conjure Flower]] `vector, identifer → —`
- [[Illuminate]] `vector, number → —`
- [[Gasp]] `entity → —`
- [[Squawk]] `vector, identifier → —`

## Sources

- 📖 **Guidebook:** [[Hexical Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Confetti](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/hexical_spells@hexical:confetti)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConfetti` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:confetti`
