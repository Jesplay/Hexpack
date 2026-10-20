---
aliases: ["Bid: Sprinting"]
tags: [hexcasting/spell, mod/moves-the-mind, family/spell, topic/creatures, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities", "Movement"]
mod: "Moves the Mind"
op_id: "movesthemind:move/sprint"
signature: "edeaqad"
start_direction: "NORTH_WEST"
input: "player, bool"
output: ""
cost: ""
iotas_in: ["[[Boolean]]", "[[Entity]]"]
iotas_out: []
entry: "[[Carpet Player Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bid: Sprinting

✦ **Spell** · [[Moves the Mind]] · 📖 [[Carpet Player Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,121.6 35.0,78.3 60.0,35.0 85.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 85.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="60.0,121.6 35.0,78.3 60.0,35.0 85.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 85.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="121.6" x2="52.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.2,97.8 56.8,105.0 47.2,110.5" fill="#fc77be"/><circle cx="60.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeaqad` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player, bool` ([[Boolean]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `movesthemind:move/sprint`

```text
before   … │ player │ bool  ◂ top
after    …                  ◂ top
```

## Description

As previous, but with sprinting. Even when stationary, the creature produces sprinting particles to denote its intention.

## Related patterns (same book page)

- [[Halt Creature]] `player → —`
- [[Summon Creature]] `vec → —`
- [[Banish Creature]] `player → —`
- [[Bid Walking]] `player, num → —`
- [[Bid Strafing]] `player, num → —`
- [[Bid Looking]] `player, vec → —`
- [[Bid Jumping]] `player → —`
- [[Bid Sneaking]] `player, bool → —`
- [[Bid Use Item]] `player, num → —`
- [[Bid Attack]] `player, num → —`
- [[Bid Select Toolbar]] `player, num → —`
- [[Bid Swap Hands]] `player → —`
- [[Bid Drop Item]] `player → —`

## Sources

- 📖 **Guidebook:** [[Carpet Player Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Moves the Mind on Modrinth](https://modrinth.com/mod/zKp2hKIx)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSetSprint` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `movesthemind:move/sprint`
