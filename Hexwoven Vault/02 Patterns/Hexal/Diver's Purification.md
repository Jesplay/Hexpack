---
aliases: ["Diver's Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexal"
op_id: "hexal:breath"
signature: "aqawdwaqawd"
start_direction: "NORTH_WEST"
input: "entity"
output: "num"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hexal Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Diver's Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Hexal Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 85.0,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 85.0,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="59.5" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,42.1 63.2,47.7 55.8,51.9" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawdwaqawd` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:breath`

```text
before   … │ entity  ◂ top
after    … │ num     ◂ top
```

## Description

Removes a Living entity from the [[The Stack|stack]] and returns how much breath it has left (i.e., how close it is to drowning).

## Related patterns (same book page)

- [[Timekeeper's Reflection]] `— → number`
- [[Nurse's Purification]] `entity → num`
- [[Squire's Purification]] `entity → num`
- [[Boxer's Purification]] `entity → num`
- [[Lamplighter's Purification]] `vec → num`

## Sources

- 📖 **Guidebook:** [[Hexal Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Diver's Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/hexal_basics@hexal:breath)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetBreath` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:breath`
