---
aliases: ["Speedometer's Reflection"]
tags: [hexcasting/pattern, mod/hexal, family/reflection, topic/wisps]
kind: "Pattern"
family: "Reflection"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:wisp/move/speed/get"
signature: "eeewdqdee"
start_direction: "EAST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Speedometer's Reflection

◇ **Pattern** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0 74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 95.9,68.3 74.1,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:436;--dur:2.8s" points="74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0 74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 95.9,68.3 74.1,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="74.1" y1="30.5" x2="88.1" y2="30.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="98.1,30.5 88.1,35.3 88.1,25.7" fill="#fc77be"/><circle cx="74.1" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeewdqdee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:wisp/move/speed/get`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Gets the executing Cyclic Wisp's current maximum speed.

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
- 🌐 **Official online docs:** [Speedometer's Reflection](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:wisp/move/speed/get)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMoveSpeedGet` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:wisp/move/speed/get`
