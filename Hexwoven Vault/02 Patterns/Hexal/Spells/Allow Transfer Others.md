---
aliases: ["Allow Transfer Others"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/wisps]
kind: "Spell"
family: "Spell"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:wisp/transfer/others/allow"
signature: "eeeeeqwweqqqqq"
start_direction: "SOUTH_WEST"
input: "wisp, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Allow Transfer Others

✦ **Spell** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="63" viewBox="0 0 170.0 63.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.0" cy="14.2" r="2"/><circle cx="24.3" cy="14.2" r="2"/><circle cx="44.5" cy="14.2" r="2"/><circle cx="64.8" cy="14.2" r="2"/><circle cx="85.0" cy="14.2" r="2"/><circle cx="105.2" cy="14.2" r="2"/><circle cx="125.5" cy="14.2" r="2"/><circle cx="145.7" cy="14.2" r="2"/><circle cx="166.0" cy="14.2" r="2"/><circle cx="14.2" cy="31.7" r="2"/><circle cx="34.4" cy="31.7" r="2"/><circle cx="54.6" cy="31.7" r="2"/><circle cx="74.9" cy="31.7" r="2"/><circle cx="95.1" cy="31.7" r="2"/><circle cx="115.4" cy="31.7" r="2"/><circle cx="135.6" cy="31.7" r="2"/><circle cx="155.8" cy="31.7" r="2"/><circle cx="4.0" cy="49.2" r="2"/><circle cx="24.3" cy="49.2" r="2"/><circle cx="44.5" cy="49.2" r="2"/><circle cx="64.8" cy="49.2" r="2"/><circle cx="85.0" cy="49.2" r="2"/><circle cx="105.2" cy="49.2" r="2"/><circle cx="125.5" cy="49.2" r="2"/><circle cx="145.7" cy="49.2" r="2"/><circle cx="166.0" cy="49.2" r="2"/></g><polyline class="hexpat-base" points="54.6,31.7 44.5,49.2 24.3,49.2 14.2,31.7 24.3,14.2 44.5,14.2 54.6,31.7 74.9,31.7 95.1,31.7 115.4,31.7 125.5,49.2 145.7,49.2 155.8,31.7 145.7,14.2 125.5,14.2 115.4,31.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:304;--dur:4.2s" points="54.6,31.7 44.5,49.2 24.3,49.2 14.2,31.7 24.3,14.2 44.5,14.2 54.6,31.7 74.9,31.7 95.1,31.7 115.4,31.7 125.5,49.2 145.7,49.2 155.8,31.7 145.7,14.2 125.5,14.2 115.4,31.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="54.6" y1="31.7" x2="51.4" y2="37.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.1,41.3 49.5,36.2 53.3,38.4" fill="#fc77be"/><circle cx="54.6" cy="31.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeeqwweqqqqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `wisp, num` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:wisp/transfer/others/allow`

```text
before   … │ wisp │ num  ◂ top
after    …               ◂ top
```

## Description

This takes a wisp that you own and an index, and allows the wisp to exchange [[Media|media]] with the linkable at that index.

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
- [[Disallow Transfer Others]] `wisp, num → —`
- [[Entity Purification Wisp]] `vec → entity | null`
- [[Zone Distillation Wisp]] `vec, num → [entity]`
- [[Zone Distillation Non-Wisp]] `vec, num → [entity]`
- [[Delay Wisp]] `number → —`
- [[Listen]] `— → —`
- [[Wander]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wisps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Allow Transfer Others](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:wisp/transfer/others/allow)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpTransferAllowedOthers` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:wisp/transfer/others/allow`
