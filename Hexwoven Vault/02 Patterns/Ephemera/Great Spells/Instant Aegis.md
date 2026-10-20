---
aliases: ["Instant Aegis"]
tags: [hexcasting/great-spell, mod/ephemera, family/great-spell, topic/effects, hexcasting/needs-enlightenment]
kind: "Great Spell"
family: "Great Spell"
topics: ["Status Effects"]
mod: "Ephemera"
op_id: "ephemera:magearmor"
signature: "qaweqqwqqewaqeqqqqqad"
start_direction: "NORTH_WEST"
input: "player, num, num, status, num"
output: ""
cost: "cost for this spell is a bit complicated: A base of one amethyst dust per 60 durability, multiplied by the armor's protection value raised to the power of 1.5, times the level+1 of any status effect."
iotas_in: ["[[Number]]", "[[Entity]]", "[[Status]]"]
iotas_out: []
entry: "[[Instant Aegis (Book Entry)]]"
needs_enlightenment: true
per_world: true
kind_source: "tag"
topic_source: "page"
in_book: true
---

# Instant Aegis

⭐ **Great Spell** · [[Ephemera]] · 📖 [[Instant Aegis (Book Entry)]] · Patterns › Great Spells

**Topics:** [[Status Effects (Topic)|Status Effects]]

**Unlock:** 🔒 needs **enlightenment** ([[Progression - Enlightenment & Great Spells|how]]) · 🎲 strokes **differ per world**: find the scroll

> [!abstract] ⭐ Great Spell
> This Great Spell is **world-altering**, has **per-world strokes** and usually needs **enlightenment**. See [[Pattern Naming Guide]] · [[Patterns by Family#Great Spell|all Great Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 53.5,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 116.5,76.6 132.2,49.3 148.0,22.0 116.5,22.0 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 85.0,76.6 69.3,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 53.5,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 116.5,76.6 132.2,49.3 148.0,22.0 116.5,22.0 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 85.0,76.6 69.3,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="49.3" x2="64.2" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,34.3 67.2,38.8 61.2,42.3" fill="#fc77be"/><circle cx="69.3" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaweqqwqqewaqeqqqqqad` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

> [!warning] Per-world pattern
> This pattern's strokes are **randomised for every world**. The shape above is only the default. Find the real one on scrolls in loot chests (see [[Progression - Enlightenment & Great Spells]]).

## Stack effect

- **Takes:** `player, num, num, status, num` ([[Number]], [[Entity]], [[Status]])
- **Leaves:** `nothing` (—)
- **Cost:** cost for this spell is a bit complicated: A base of one amethyst dust per 60 durability, multiplied by the armor's protection value raised to the power of 1.5, times the level+1 of any status effect.
- **ID:** `ephemera:magearmor`

```text
before   … │ player │ num │ num │ status │ num  ◂ top
after    …                                      ◂ top
```

## Description

Accepts a player to grant the armor to, the durability of the armor, the protection value of the armor (max 10), a status effect to apply, and that effect's level. The latter two can be replaced with any non-status [[Iotas|iota]] to ignore them.

Of course, this barrier is by no means indestructible, and its durability decays steadily in addition to any damage it takes from hits. The base rate of this decay can be described as one "point" per second, plus additional points equal to the squared strength of any effect imbued into it. Additionally, it will disappear instantly if removed from the wearer, so there is no way to repair it, short of recasting the spell to overwrite its current state.

The [[Media|media]] cost for this spell is a bit complicated: A base of one amethyst dust per 60 durability, multiplied by the armor's protection value raised to the power of 1.5, times the level+1 of any status effect. Thus, producing no-status strength 10 armor with one minute's worth of durability costs ~31.5 dust. Adding Regeneration 2 increases that media cost to ~94.5 dust, and causes it to only last 36 seconds.

This explanation assumes I am wearing no other armor, in practice the cost can be reduced to as little as 25%% of that formula's output if all but one of my armor slots is already occupied by more conventional armor.

## Sources

- 📖 **Guidebook:** [[Instant Aegis (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Instant Aegis](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/great/magearmor@ephemera:magearmor)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Great Spell):** the mod tags it as a per-world (Great Spell) pattern
- 🔧 **Pattern ID:** `ephemera:magearmor`
