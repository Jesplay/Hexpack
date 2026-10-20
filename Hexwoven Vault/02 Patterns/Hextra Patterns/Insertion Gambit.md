---
aliases: ["Insertion Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/data-structures]
kind: "Pattern"
family: "Gambit"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hextra Patterns"
op_id: "hextrapats:set_insert_ret"
signature: "edqdewd"
start_direction: "SOUTH_WEST"
input: "set, (num|vec|entity)"
output: "set, bool"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]", "[[Set]]"]
iotas_out: ["[[Boolean]]", "[[Set]]"]
entry: "[[Hash Set Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Insertion Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,78.3 85.0,121.6 35.0,121.6 60.0,78.3 35.0,35.0 85.0,35.0 110.0,78.3 135.0,121.6 85.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="110.0,78.3 85.0,121.6 35.0,121.6 60.0,78.3 35.0,35.0 85.0,35.0 110.0,78.3 135.0,121.6 85.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="78.3" x2="102.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="96.2,102.1 97.2,89.4 106.8,94.9" fill="#fc77be"/><circle cx="110.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edqdewd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `set, (num|vec|entity)` ([[Number]], [[Vector]], [[Entity]], [[Set]])
- **Leaves:** `set, bool` ([[Boolean]], [[Set]])
- **ID:** `hextrapats:set_insert_ret`

## Description

Remove the [[Iotas|iota]] at the top of the [[The Stack|stack]], then add it to the set. Return True if it wasn't already in the set, otherwise return False. The iota's type must match the set's type.

## Related patterns (same book page)

- [[Vacant Reflection Number Set]] `— → num set`
- [[Vacant Reflection Vector Set]] `— → vec set`
- [[Vacant Reflection Entity Set]] `— → entity set`
- [[Removal Gambit]] `set, (num|vec|entity) → set, bool`
- [[Ordering Purification]] `set → list`
- [[Disordering Purification]] `[num]|[vec]|[entity] → set`
- [[Bucketing Gambit]] `many, num → set`
- [[Spilling Disintegration]] `set → many`

## Sources

- 📖 **Guidebook:** [[Hash Set Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:set_insert_ret`
