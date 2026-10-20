---
aliases: ["Bucketing Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/data-structures]
kind: "Pattern"
family: "Gambit"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hextra Patterns"
op_id: "hextrapats:last_n_set"
signature: "ewdwaawaqde"
start_direction: "SOUTH_WEST"
input: "many, num"
output: "set"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Set]]"]
entry: "[[Hash Set Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bucketing Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,68.3 117.7,106.0 74.1,106.0 30.5,106.0 52.3,68.3 74.1,30.5 30.5,30.5 52.3,68.3 74.1,106.0 95.9,68.3 74.1,30.5 117.7,30.5 139.5,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:523;--dur:3.4s" points="139.5,68.3 117.7,106.0 74.1,106.0 30.5,106.0 52.3,68.3 74.1,30.5 30.5,30.5 52.3,68.3 74.1,106.0 95.9,68.3 74.1,30.5 117.7,30.5 139.5,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="68.3" x2="132.5" y2="80.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,89.0 128.4,77.9 136.7,82.7" fill="#fc77be"/><circle cx="139.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewdwaawaqde` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many, num` ([[Number]])
- **Leaves:** `set` ([[Set]])
- **ID:** `hextrapats:last_n_set`

## Description

Remove num [[Iotas|iota]] from the top of the [[The Stack|stack]], then add them to a set at the top of the stack. Must be either all numbers, all vectors, or all entities.

## Related patterns (same book page)

- [[Vacant Reflection Number Set]] `— → num set`
- [[Vacant Reflection Vector Set]] `— → vec set`
- [[Vacant Reflection Entity Set]] `— → entity set`
- [[Insertion Gambit]] `set, (num|vec|entity) → set, bool`
- [[Removal Gambit]] `set, (num|vec|entity) → set, bool`
- [[Ordering Purification]] `set → list`
- [[Disordering Purification]] `[num]|[vec]|[entity] → set`
- [[Spilling Disintegration]] `set → many`

## Sources

- 📖 **Guidebook:** [[Hash Set Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpLastNToSet` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:last_n_set`
