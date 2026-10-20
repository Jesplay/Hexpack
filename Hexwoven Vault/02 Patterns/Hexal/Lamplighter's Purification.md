---
aliases: ["Lamplighter's Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexal"
op_id: "hexal:light_level"
signature: "qedqde"
start_direction: "NORTH_EAST"
input: "vec"
output: "num"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hexal Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Lamplighter's Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Hexal Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 114.8,66.6 93.5,103.4 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="29.8,140.2 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 114.8,66.6 93.5,103.4 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qedqde` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:light_level`

```text
before   … │ vec  ◂ top
after    … │ num  ◂ top
```

## Description

Removes a position vector from the [[The Stack|stack]] and returns the light level at that position.

## Related patterns (same book page)

- [[Timekeeper's Reflection]] `— → number`
- [[Diver's Purification]] `entity → num`
- [[Nurse's Purification]] `entity → num`
- [[Squire's Purification]] `entity → num`
- [[Boxer's Purification]] `entity → num`

## Sources

- 📖 **Guidebook:** [[Hexal Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Lamplighter's Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/hexal_basics@hexal:light_level)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetLightLevel` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:light_level`
