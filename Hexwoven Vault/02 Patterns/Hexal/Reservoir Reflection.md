---
aliases: ["Reservoir Reflection"]
tags: [hexcasting/pattern, mod/hexal, family/reflection, topic/wisps]
kind: "Pattern"
family: "Reflection"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:wisp/media"
signature: "aqaweewaqaweedw"
start_direction: "NORTH_WEST"
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

# Reservoir Reflection

◇ **Pattern** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 53.5,49.3 69.3,22.0 100.7,22.0 116.5,49.3 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3 100.7,76.6 69.3,76.6 53.5,49.3 85.0,49.3 116.5,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.5s" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 53.5,49.3 69.3,22.0 100.7,22.0 116.5,49.3 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3 100.7,76.6 69.3,76.6 53.5,49.3 85.0,49.3 116.5,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="49.3" x2="48.5" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,34.3 51.5,38.8 45.5,42.3" fill="#fc77be"/><circle cx="53.5" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaweewaqaweedw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:wisp/media`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Add the amount of [[Media|media]] in the wisp's reservoir to the [[The Stack|stack]]. This must be cast by a wisp; trying to cast it through a [[Staves|staff]] will fail rather spectacularly.

## Related patterns (same book page)

- [[Summon Projectile Wisp]] `[pattern], vec, vec, number → —`
- [[Summon Cyclic Wisp]] `[pattern], vec, number → —`
- [[Identity Reflection]] `— → entity`
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
- 🌐 **Official online docs:** [Reservoir Reflection](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:wisp/media)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpWispMedia` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:wisp/media`
