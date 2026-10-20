---
aliases: ["Zone Distillation: Non-Wisp"]
tags: [hexcasting/pattern, mod/hexal, family/distillation, topic/wisps]
kind: "Pattern"
family: "Distillation"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:zone_entity/not_wisp"
signature: "eewaqaweewaqaaww"
start_direction: "NORTH_EAST"
input: "vec, num"
output: "[entity]"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Entity]]", "[[List]]"]
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Zone Distillation: Non-Wisp

◇ **Pattern** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="72" viewBox="0 0 170.0 72.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.1" r="2"/><circle cx="27.6" cy="16.1" r="2"/><circle cx="50.5" cy="16.1" r="2"/><circle cx="73.5" cy="16.1" r="2"/><circle cx="96.5" cy="16.1" r="2"/><circle cx="119.5" cy="16.1" r="2"/><circle cx="142.4" cy="16.1" r="2"/><circle cx="165.4" cy="16.1" r="2"/><circle cx="16.1" cy="36.0" r="2"/><circle cx="39.1" cy="36.0" r="2"/><circle cx="62.0" cy="36.0" r="2"/><circle cx="85.0" cy="36.0" r="2"/><circle cx="108.0" cy="36.0" r="2"/><circle cx="130.9" cy="36.0" r="2"/><circle cx="153.9" cy="36.0" r="2"/><circle cx="4.6" cy="55.9" r="2"/><circle cx="27.6" cy="55.9" r="2"/><circle cx="50.5" cy="55.9" r="2"/><circle cx="73.5" cy="55.9" r="2"/><circle cx="96.5" cy="55.9" r="2"/><circle cx="119.5" cy="55.9" r="2"/><circle cx="142.4" cy="55.9" r="2"/><circle cx="165.4" cy="55.9" r="2"/></g><polyline class="hexpat-base" points="85.0,36.0 96.5,16.1 119.5,16.1 130.9,36.0 142.4,55.9 153.9,36.0 142.4,16.1 130.9,36.0 119.5,55.9 96.5,55.9 85.0,36.0 73.5,16.1 62.0,36.0 73.5,55.9 85.0,36.0 62.0,36.0 39.1,36.0 16.1,36.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:391;--dur:4.8s" points="85.0,36.0 96.5,16.1 119.5,16.1 130.9,36.0 142.4,55.9 153.9,36.0 142.4,16.1 130.9,36.0 119.5,55.9 96.5,55.9 85.0,36.0 73.5,16.1 62.0,36.0 73.5,55.9 85.0,36.0 62.0,36.0 39.1,36.0 16.1,36.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="36.0" x2="88.7" y2="29.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="91.3,25.0 90.9,30.9 86.5,28.3" fill="#fc77be"/><circle cx="85.0" cy="36.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eewaqaweewaqaaww` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `[entity]` ([[Entity]], [[List]])
- **ID:** `hexal:zone_entity/not_wisp`

```text
before   … │ vec │ num  ◂ top
after    … │ [entity]   ◂ top
```

## Description

Take a position and maximum distance on the [[The Stack|stack]], and combine them into a list of non-wisp entities near the position.

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
- [[Entity Purification Wisp]] `vec → entity | null`
- [[Zone Distillation Wisp]] `vec, num → [entity]`
- [[Delay Wisp]] `number → —`
- [[Listen]] `— → —`
- [[Wander]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wisps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Zone Distillation: Non-Wisp](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:zone_entity/not_wisp)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntitiesBy` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:zone_entity/not_wisp`
