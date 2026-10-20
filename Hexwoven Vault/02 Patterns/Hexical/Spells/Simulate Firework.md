---
aliases: ["Simulate Firework"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:simulate_firework"
signature: "dedwaqwqqwqa"
start_direction: "SOUTH_WEST"
input: "vector, vector, number"
output: ""
cost: "Costs about one Amethyst Shard."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Pyrotechnics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Simulate Firework

✦ **Spell** · [[Hexical]] · 📖 [[Pyrotechnics]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="104.3,94.0 85.0,127.4 65.7,94.0 85.0,60.5 104.3,94.0 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4 65.7,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="104.3,94.0 85.0,127.4 65.7,94.0 85.0,60.5 104.3,94.0 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4 65.7,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="94.0" x2="98.1" y2="104.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,112.4 94.5,102.5 101.8,106.8" fill="#fc77be"/><circle cx="104.3" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedwaqwqqwqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Shard.
- **ID:** `hexical:simulate_firework`

```text
before   … │ vector │ vector │ number  ◂ top
after    …                             ◂ top
```

## Description

Analyzes the **Firework Star** in my other hand and conjures a firework of that star, with the position, velocity, and gunpowder amount specified. Costs about one [[Amethyst|Amethyst Shard]].

## Related patterns (same book page)

- [[Conjure Firework]] `vec, vec, num, num, list of vec, list of vec, bool, bool → —`

## Sources

- 📖 **Guidebook:** [[Pyrotechnics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Simulate Firework](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/pyrotechnics@hexical:simulate_firework)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSimulateFirework` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:simulate_firework`
