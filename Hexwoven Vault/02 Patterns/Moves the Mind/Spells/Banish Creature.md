---
aliases: ["Banish Creature"]
tags: [hexcasting/spell, mod/moves-the-mind, family/spell, topic/creatures, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities", "Movement"]
mod: "Moves the Mind"
op_id: "movesthemind:destroy"
signature: "edeaqqwadawqq"
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

# Banish Creature

✦ **Spell** · [[Moves the Mind]] · 📖 [[Carpet Player Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="85.0,127.4 65.7,94.0 85.0,60.5 104.3,94.0 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 85.0,60.5 65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="85.0,127.4 65.7,94.0 85.0,60.5 104.3,94.0 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 85.0,60.5 65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="127.4" x2="78.8" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,109.0 82.5,114.6 75.1,118.8" fill="#fc77be"/><circle cx="85.0" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeaqqwadawqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `movesthemind:destroy`

```text
before   … │ player  ◂ top
after    …           ◂ top
```

## Description

Banishes the creature, making it disappear until I call it back. One Shard.

## Related patterns (same book page)

- [[Halt Creature]] `player → —`
- [[Summon Creature]] `vec → —`
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
- [[Bid Drop Item]] `player → —`

## Sources

- 📖 **Guidebook:** [[Carpet Player Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Moves the Mind on Modrinth](https://modrinth.com/mod/zKp2hKIx)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpDestroyFakeplayer` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `movesthemind:destroy`
