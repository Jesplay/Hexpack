---
aliases: ["Entity Purification: Wisp"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/wisps]
kind: "Pattern"
family: "Purification"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:get_entity/wisp"
signature: "qqwdedwqqdaqaaww"
start_direction: "SOUTH_EAST"
input: "vec"
output: "entity | null"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]", "[[Null]]"]
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Entity Purification: Wisp

◇ **Pattern** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="72" viewBox="0 0 170.0 72.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.1" r="2"/><circle cx="27.6" cy="16.1" r="2"/><circle cx="50.5" cy="16.1" r="2"/><circle cx="73.5" cy="16.1" r="2"/><circle cx="96.5" cy="16.1" r="2"/><circle cx="119.5" cy="16.1" r="2"/><circle cx="142.4" cy="16.1" r="2"/><circle cx="165.4" cy="16.1" r="2"/><circle cx="16.1" cy="36.0" r="2"/><circle cx="39.1" cy="36.0" r="2"/><circle cx="62.0" cy="36.0" r="2"/><circle cx="85.0" cy="36.0" r="2"/><circle cx="108.0" cy="36.0" r="2"/><circle cx="130.9" cy="36.0" r="2"/><circle cx="153.9" cy="36.0" r="2"/><circle cx="4.6" cy="55.9" r="2"/><circle cx="27.6" cy="55.9" r="2"/><circle cx="50.5" cy="55.9" r="2"/><circle cx="73.5" cy="55.9" r="2"/><circle cx="96.5" cy="55.9" r="2"/><circle cx="119.5" cy="55.9" r="2"/><circle cx="142.4" cy="55.9" r="2"/><circle cx="165.4" cy="55.9" r="2"/></g><polyline class="hexpat-base" points="85.0,36.0 96.5,55.9 119.5,55.9 130.9,36.0 142.4,16.1 153.9,36.0 142.4,55.9 130.9,36.0 119.5,16.1 96.5,16.1 85.0,36.0 73.5,16.1 62.0,36.0 73.5,55.9 85.0,36.0 62.0,36.0 39.1,36.0 16.1,36.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:391;--dur:4.8s" points="85.0,36.0 96.5,55.9 119.5,55.9 130.9,36.0 142.4,16.1 153.9,36.0 142.4,55.9 130.9,36.0 119.5,16.1 96.5,16.1 85.0,36.0 73.5,16.1 62.0,36.0 73.5,55.9 85.0,36.0 62.0,36.0 39.1,36.0 16.1,36.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="36.0" x2="88.7" y2="42.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="91.3,46.9 86.5,43.6 90.9,41.1" fill="#fc77be"/><circle cx="85.0" cy="36.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqwdedwqqdaqaaww` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `entity | null` ([[Entity]], [[Null]])
- **ID:** `hexal:get_entity/wisp`

## Description

Transform the position on the [[The Stack|stack]] into the wisp at that location (or Null if there isn't one).

## Related patterns (same book page)

- [[Summon Projectile Wisp]] `[pattern], vec, vec, number → —`
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
- [[Zone Distillation Wisp]] `vec, num → [entity]`
- [[Zone Distillation Non-Wisp]] `vec, num → [entity]`
- [[Delay Wisp]] `number → —`
- [[Listen]] `— → —`
- [[Wander]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wisps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Entity Purification: Wisp](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:get_entity/wisp)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityAt` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:get_entity/wisp`
