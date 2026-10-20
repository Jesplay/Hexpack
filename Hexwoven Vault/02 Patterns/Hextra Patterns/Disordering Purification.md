---
aliases: ["Disordering Purification"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/purification, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hextra Patterns"
op_id: "hextrapats:list_to_set"
signature: "qqaeaadwaddqdee"
start_direction: "NORTH_EAST"
input: "[num]|[vec]|[entity]"
output: "set"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]", "[[List]]"]
iotas_out: ["[[Set]]"]
entry: "[[Hash Set Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Disordering Purification

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 46.4,60.5 85.0,60.5 123.6,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0 85.0,60.5 104.3,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:618;--dur:4.5s" points="65.7,94.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 46.4,60.5 85.0,60.5 123.6,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0 85.0,60.5 104.3,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="71.9" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,75.6 75.5,85.4 68.2,81.1" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqaeaadwaddqdee` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[num]|[vec]|[entity]` ([[Number]], [[Vector]], [[Entity]], [[List]])
- **Leaves:** `set` ([[Set]])
- **ID:** `hextrapats:list_to_set`

## Description

Converts the list at the top of the [[The Stack|stack]] to a set. The list must be non-empty and entirely numbers, vectors, or entities.

## Related patterns (same book page)

- [[Vacant Reflection Number Set]] `— → num set`
- [[Vacant Reflection Vector Set]] `— → vec set`
- [[Vacant Reflection Entity Set]] `— → entity set`
- [[Insertion Gambit]] `set, (num|vec|entity) → set, bool`
- [[Removal Gambit]] `set, (num|vec|entity) → set, bool`
- [[Ordering Purification]] `set → list`
- [[Bucketing Gambit]] `many, num → set`
- [[Spilling Disintegration]] `set → many`

## Sources

- 📖 **Guidebook:** [[Hash Set Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpListToSet` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:list_to_set`
