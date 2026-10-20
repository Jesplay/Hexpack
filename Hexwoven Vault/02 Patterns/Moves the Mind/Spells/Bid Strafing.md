---
aliases: ["Bid: Strafing"]
tags: [hexcasting/spell, mod/moves-the-mind, family/spell, topic/creatures, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities", "Movement"]
mod: "Moves the Mind"
op_id: "movesthemind:move/strafe"
signature: "edewa"
start_direction: "NORTH_WEST"
input: "player, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Carpet Player Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bid: Strafing

✦ **Spell** · [[Moves the Mind]] · 📖 [[Carpet Player Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2 72.3,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2 72.3,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="103.4" x2="44.2" y2="91.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,83.2 48.3,89.3 40.2,94.0" fill="#fc77be"/><circle cx="51.0" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edewa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player, num` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `movesthemind:move/strafe`

```text
before   … │ player │ num  ◂ top
after    …                 ◂ top
```

## Description

As previous, but along strafing. A 1, then, beckons it to the left, and a -1 to the right.

## Related patterns (same book page)

- [[Halt Creature]] `player → —`
- [[Summon Creature]] `vec → —`
- [[Banish Creature]] `player → —`
- [[Bid Walking]] `player, num → —`
- [[Bid Looking]] `player, vec → —`
- [[Bid Jumping]] `player → —`
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
- 🏷️ **Type (Spell):** from the mod code: its action class `OpStrafeInAforementionedLouisVuittons` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `movesthemind:move/strafe`
