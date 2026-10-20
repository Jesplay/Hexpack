---
aliases: ["Delay Wisp"]
tags: [hexcasting/pattern, mod/hexal, family/other, topic/wisps]
kind: "Pattern"
family: "Other"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:wisp/trigger/tick"
signature: "aqawded"
start_direction: "NORTH_WEST"
input: "number"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Delay Wisp

◇ **Pattern** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 60.0,35.0 35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="85.0,78.3 60.0,35.0 35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="77.0" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="71.2,54.5 81.8,61.7 72.2,67.2" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawded` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:wisp/trigger/tick`

```text
before   … │ number  ◂ top
after    …           ◂ top
```

## Description

Removes an integer from the [[The Stack|stack]], then delays the wisp's next cast until at least that many twentieth's of a second have passed. This and other triggers like it will halve the wisp's upkeep cost while it isn't casting.

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
- [[Zone Distillation Non-Wisp]] `vec, num → [entity]`
- [[Listen]] `— → —`
- [[Wander]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wisps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Delay Wisp](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:wisp/trigger/tick)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpWispSetTrigger` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:wisp/trigger/tick`
