---
aliases: ["Bid: Drop Item"]
tags: [hexcasting/spell, mod/moves-the-mind, family/spell, topic/creatures, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities", "Movement"]
mod: "Moves the Mind"
op_id: "movesthemind:move/drop"
signature: "edeaqe"
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

# Bid: Drop Item

✦ **Spell** · [[Moves the Mind]] · 📖 [[Carpet Player Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:2.0s" points="46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="40.2" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,75.6 43.9,81.1 36.5,85.4" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeaqe` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `movesthemind:move/drop`

```text
before   … │ player  ◂ top
after    …           ◂ top
```

## Description

Instructs the creature to drop the item or [[The Stack|stack]] of items it holds in its main hand. Can even yank an item from my own hand, as I'm so familiar with.

## Related patterns (same book page)

- [[Halt Creature]] `player → —`
- [[Summon Creature]] `vec → —`
- [[Banish Creature]] `player → —`
- [[Bid Walking]] `player, num → —`
- [[Bid Strafing]] `player, num → —`
- [[Bid Looking]] `player, vec → —`
- [[Bid Jumping]] `player → —`
- [[Bid Sneaking]] `player, bool → —`
- [[Bid Sprinting]] `player, bool → —`
- [[Bid Use Item]] `player, num → —`
- [[Bid Attack]] `player, num → —`
- [[Bid Select Toolbar]] `player, num → —`
- [[Bid Swap Hands]] `player → —`

## Sources

- 📖 **Guidebook:** [[Carpet Player Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Moves the Mind on Modrinth](https://modrinth.com/mod/zKp2hKIx)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpDropAndRoll` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `movesthemind:move/drop`
