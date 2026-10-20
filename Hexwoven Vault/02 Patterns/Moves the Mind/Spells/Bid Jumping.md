---
aliases: ["Bid: Jumping"]
tags: [hexcasting/spell, mod/moves-the-mind, family/spell, topic/creatures, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities", "Movement"]
mod: "Moves the Mind"
op_id: "movesthemind:move/jump"
signature: "edeqda"
start_direction: "NORTH_WEST"
input: "player"
output: ""
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Carpet Player Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bid: Jumping

✦ **Spell** · [[Moves the Mind]] · 📖 [[Carpet Player Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="84" height="170" viewBox="0 0 83.9 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,85.0 24.5,54.7 41.9,24.5 59.4,54.7 41.9,85.0 59.4,115.3 24.5,115.3 41.9,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:2.0s" points="41.9,85.0 24.5,54.7 41.9,24.5 59.4,54.7 41.9,85.0 59.4,115.3 24.5,115.3 41.9,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="85.0" x2="36.3" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="32.3,68.4 39.7,73.4 33.0,77.2" fill="#fc77be"/><circle cx="41.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeqda` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `movesthemind:move/jump`

```text
before   … │ player  ◂ top
after    …           ◂ top
```

## Description

Simply instructs the creature to hop. Also works on myself.

## Related patterns (same book page)

- [[Halt Creature]] `player → —`
- [[Summon Creature]] `vec → —`
- [[Banish Creature]] `player → —`
- [[Bid Walking]] `player, num → —`
- [[Bid Strafing]] `player, num → —`
- [[Bid Looking]] `player, vec → —`
- [[Bid Sneaking]] `player, bool → —`
- [[Bid Sprinting]] `player, bool → —`
- [[Bid Use Item]] `player, num → —`
- [[Bid Attack]] `player, num → —`
- [[Bid Select Toolbar]] `player, num → —`
- [[Bid Swap Hands]] `player → —`
- [[Bid Drop Item]] `player → —`

## Sources

- 📖 **Guidebook:** [[Carpet Player Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Moves the Mind on Modrinth](https://modrinth.com/mod/zKp2hKIx)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpJumpButWithYourFeet` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `movesthemind:move/jump`
