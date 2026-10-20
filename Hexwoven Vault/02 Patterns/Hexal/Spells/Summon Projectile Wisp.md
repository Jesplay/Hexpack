---
aliases: ["Summon Projectile Wisp"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/wisps]
kind: "Spell"
family: "Spell"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:wisp/summon/projectile"
signature: "aqaeqeeeee"
start_direction: "NORTH_WEST"
input: "[pattern], vec, vec, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Summon Projectile Wisp

✦ **Spell** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 53.5,49.3 85.0,49.3 100.7,22.0 132.2,22.0 148.0,49.3 132.2,76.6 100.7,76.6 85.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:346;--dur:3.1s" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 53.5,49.3 85.0,49.3 100.7,22.0 132.2,22.0 148.0,49.3 132.2,76.6 100.7,76.6 85.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="49.3" x2="48.5" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,34.3 51.5,38.8 45.5,42.3" fill="#fc77be"/><circle cx="53.5" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaeqeeeee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern], vec, vec, number` ([[Number]], [[Vector]], [[List]], [[Pattern]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:wisp/summon/projectile`

```text
before   … │ [pattern] │ vec │ vec │ number  ◂ top
after    …                                   ◂ top
```

## Description

This wisp is summoned and launched like an arrow, casting its hex on whatever it hits. The initial [[The Stack|stack]] when it executes will be a reference to itself, then a reference to what it hit.

This could be a block position or an entity reference. The first vector accepted is the location the wisp will be summoned, while the direction of the second is the direction the wisp will be launched, and the second's magnitude seems to affect the wisp's velocity. The cost of the spell increases with the squared magnitude of velocity. The number accepted determines the wisp's reservoir in [[Amethyst|Amethyst Dust]]. It loses approximately 3 tenths of an [[Amethyst|Amethyst Dust]] of [[Media|media]] every second once summoned, and has a range of influence of 4 blocks.

## Related patterns (same book page)

- [[Summon Cyclic Wisp]] `[pattern], vec, number → —`
- [[Identity Reflection]] `— → entity`
- [[Reservoir Reflection]] `— → num`
- [[Manager's Purification]] `entity → [pattern]`
- [[Allegiance Distillation]] `entity, entity → bool`
- [[Pathfinder's Gambit]] `vec → —`
- [[Pathfinder's Reflection]] `— → vec`
- [[Haste]] `num → —`
- [[Speedometer's Reflection]] `— → num`
- [[Allow Transfer]] `num → —`
- [[Disallow Transfer]] `num → —`
- [[Allow Transfer Others]] `wisp, num → —`
- [[Disallow Transfer Others]] `wisp, num → —`
- [[Entity Purification Wisp]] `vec → entity | null`
- [[Zone Distillation Wisp]] `vec, num → [entity]`
- [[Zone Distillation Non-Wisp]] `vec, num → [entity]`
- [[Delay Wisp]] `number → —`
- [[Listen]] `— → —`
- [[Wander]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wisps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Summon Projectile Wisp](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:wisp/summon/projectile)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSummonWisp` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:wisp/summon/projectile`
