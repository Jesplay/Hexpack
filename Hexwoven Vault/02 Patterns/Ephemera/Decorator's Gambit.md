---
aliases: ["Decorator's Gambit"]
tags: [hexcasting/pattern, mod/ephemera, family/gambit, topic/items]
kind: "Pattern"
family: "Gambit"
topics: ["Items & Crafting"]
mod: "Ephemera"
op_id: "ephemera:setframerotation"
signature: "wwawwqwwawwaqdq"
start_direction: "SOUTH_WEST"
input: "item frame, number"
output: ""
cost: "Costs a negligible amount of media."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Misc. Ephemera Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Decorator's Gambit

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="115" viewBox="0 0 170.0 115.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.8" cy="20.2" r="2"/><circle cx="34.6" cy="20.2" r="2"/><circle cx="63.4" cy="20.2" r="2"/><circle cx="92.2" cy="20.2" r="2"/><circle cx="121.0" cy="20.2" r="2"/><circle cx="149.8" cy="20.2" r="2"/><circle cx="20.2" cy="45.1" r="2"/><circle cx="49.0" cy="45.1" r="2"/><circle cx="77.8" cy="45.1" r="2"/><circle cx="106.6" cy="45.1" r="2"/><circle cx="135.4" cy="45.1" r="2"/><circle cx="164.2" cy="45.1" r="2"/><circle cx="5.8" cy="70.1" r="2"/><circle cx="34.6" cy="70.1" r="2"/><circle cx="63.4" cy="70.1" r="2"/><circle cx="92.2" cy="70.1" r="2"/><circle cx="121.0" cy="70.1" r="2"/><circle cx="149.8" cy="70.1" r="2"/><circle cx="20.2" cy="95.0" r="2"/><circle cx="49.0" cy="95.0" r="2"/><circle cx="77.8" cy="95.0" r="2"/><circle cx="106.6" cy="95.0" r="2"/><circle cx="135.4" cy="95.0" r="2"/><circle cx="164.2" cy="95.0" r="2"/></g><polyline class="hexpat-base" points="63.4,20.2 49.0,45.1 34.6,70.1 20.2,95.0 49.0,95.0 77.8,95.0 106.6,95.0 121.0,70.1 135.4,45.1 149.8,20.2 121.0,20.2 92.2,20.2 63.4,20.2 77.8,45.1 106.6,45.1 92.2,70.1 106.6,95.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:461;--dur:4.5s" points="63.4,20.2 49.0,45.1 34.6,70.1 20.2,95.0 49.0,95.0 77.8,95.0 106.6,95.0 121.0,70.1 135.4,45.1 149.8,20.2 121.0,20.2 92.2,20.2 63.4,20.2 77.8,45.1 106.6,45.1 92.2,70.1 106.6,95.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.4" y1="20.2" x2="58.8" y2="28.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.5,33.9 56.0,26.6 61.5,29.7" fill="#fc77be"/><circle cx="63.4" cy="20.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwawwqwwawwaqdq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item frame, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a negligible amount of media.
- **ID:** `ephemera:setframerotation`

```text
before   … │ item frame │ number  ◂ top
after    …                        ◂ top
```

## Description

Accepts an item frame entity and an integer from 0 to 7, and sets its rotation to the number. Costs a negligible amount of [[Media|media]].

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[Entropy Reflection II]] `— → num`
- [[Clearance Purification]] `vec → bool`
- [[Flock's Reflection II]] `— → num`
- [[Flock's Purification]] `— → num`
- [[State Purification]] `vec → [string]`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[Ride Wisp]] `wisp → —`
- [[Dismount]] `— → —`
- [[Equestrian Purification]] `entity → [entity]`
- [[Equestrian Purification II]] `entity → entity`

## Sources

- 📖 **Guidebook:** [[Misc. Ephemera Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Decorator's Gambit](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:setframerotation)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpFrameRotation` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:setframerotation`
