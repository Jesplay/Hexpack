---
aliases: ["Pathfinder's Gambit"]
tags: [hexcasting/pattern, mod/hexal, family/gambit, topic/wisps]
kind: "Pattern"
family: "Gambit"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:wisp/move/target/set"
signature: "awqwawqaw"
start_direction: "WEST"
input: "vec"
output: ""
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Pathfinder's Gambit

◇ **Pattern** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,27.0 27.0,27.0 46.4,60.5 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 85.0,60.5 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="65.7,27.0 27.0,27.0 46.4,60.5 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 85.0,60.5 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="27.0" x2="53.3" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.4,27.0 53.3,22.8 53.3,31.3" fill="#fc77be"/><circle cx="65.7" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awqwawqaw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:wisp/move/target/set`

```text
before   … │ vec  ◂ top
after    …        ◂ top
```

## Description

Removes a position vector from the [[The Stack|stack]]; The Cyclic Wisp that executed this pattern will move in a straight line towards that position (colliding with terrain in its path) until it reaches it.

## Related patterns (same book page)

- [[Summon Projectile Wisp]] `[pattern], vec, vec, number → —`
- [[Summon Cyclic Wisp]] `[pattern], vec, number → —`
- [[Identity Reflection]] `— → entity`
- [[Reservoir Reflection]] `— → num`
- [[Manager's Purification]] `entity → [pattern]`
- [[Allegiance Distillation]] `entity, entity → bool`
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
- 🌐 **Official online docs:** [Pathfinder's Gambit](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:wisp/move/target/set)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMoveTargetSet` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:wisp/move/target/set`
