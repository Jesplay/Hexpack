---
aliases: ["Squire's Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexal"
op_id: "hexal:armour"
signature: "wqqqqw"
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

# Squire's Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Hexal Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,106.0 117.7,68.3 95.9,30.5 52.3,30.5 30.5,68.3 52.3,106.0 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="139.5,106.0 117.7,68.3 95.9,30.5 52.3,30.5 30.5,68.3 52.3,106.0 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="106.0" x2="132.5" y2="93.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,85.3 136.7,91.5 128.4,96.3" fill="#fc77be"/><circle cx="139.5" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqqw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:armour`

```text
before   … │ entity  ◂ top
after    … │ num     ◂ top
```

## Description

Removes a Living entity from the [[The Stack|stack]] and returns how much armour it has.

## Related patterns (same book page)

- [[Timekeeper's Reflection]] `— → number`
- [[Diver's Purification]] `entity → num`
- [[Nurse's Purification]] `entity → num`
- [[Boxer's Purification]] `entity → num`
- [[Lamplighter's Purification]] `vec → num`

## Sources

- 📖 **Guidebook:** [[Hexal Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Squire's Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/hexal_basics@hexal:armour)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetArmour` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:armour`
