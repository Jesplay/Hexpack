---
aliases: ["Allegiance Distillation"]
tags: [hexcasting/pattern, mod/hexal, family/distillation, topic/wisps]
kind: "Pattern"
family: "Distillation"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:wisp/owner"
signature: "dwqqwdedwqqwddwww"
start_direction: "SOUTH_WEST"
input: "entity, entity"
output: "bool"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Wisps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Allegiance Distillation

◇ **Pattern** · [[Hexal]] · 📖 [[Wisps]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="148.0,49.3 132.2,76.6 116.5,49.3 100.7,22.0 69.3,22.0 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 53.5,49.3 69.3,76.6 100.7,76.6 116.5,49.3 132.2,22.0 148.0,49.3 116.5,49.3 85.0,49.3 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="148.0,49.3 132.2,76.6 116.5,49.3 100.7,22.0 69.3,22.0 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 53.5,49.3 69.3,76.6 100.7,76.6 116.5,49.3 132.2,22.0 148.0,49.3 116.5,49.3 85.0,49.3 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="148.0" y1="49.3" x2="142.9" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="139.3,64.3 139.9,56.3 145.9,59.8" fill="#fc77be"/><circle cx="148.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwqqwdedwqqwddwww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, entity` ([[Entity]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexal:wisp/owner`

```text
before   … │ entity │ entity  ◂ top
after    … │ bool             ◂ top
```

## Description

Removes a reference to a wisp and an entity from the [[The Stack|stack]], and returns true if that entity owns that wisp, and false otherwise.

## Related patterns (same book page)

- [[Summon Projectile Wisp]] `[pattern], vec, vec, number → —`
- [[Summon Cyclic Wisp]] `[pattern], vec, number → —`
- [[Identity Reflection]] `— → entity`
- [[Reservoir Reflection]] `— → num`
- [[Manager's Purification]] `entity → [pattern]`
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
- 🌐 **Official online docs:** [Allegiance Distillation](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps@hexal:wisp/owner)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpWispOwner` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:wisp/owner`
