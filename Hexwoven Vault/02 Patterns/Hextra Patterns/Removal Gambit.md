---
aliases: ["Removal Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/data-structures]
kind: "Pattern"
family: "Gambit"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hextra Patterns"
op_id: "hextrapats:set_remove_ret"
signature: "edqdewaqaaed"
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

# Removal Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,60.5 65.7,94.0 27.0,94.0 46.4,60.5 27.0,27.0 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5 104.3,27.0 85.0,60.5 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="85.0,60.5 65.7,94.0 27.0,94.0 46.4,60.5 27.0,27.0 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5 104.3,27.0 85.0,60.5 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="60.5" x2="78.8" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,78.9 75.1,69.1 82.5,73.3" fill="#fc77be"/><circle cx="85.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edqdewaqaaed` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `set, (num|vec|entity)` ([[Number]], [[Vector]], [[Entity]], [[Set]])
- **Leaves:** `set, bool` ([[Boolean]], [[Set]])
- **ID:** `hextrapats:set_remove_ret`

## Description

Remove the [[Iotas|iota]] from top of the [[The Stack|stack]], then remove it set. Return True if it was previously in the set, otherwise return False. The iota's type must match the set's type.

## Related patterns (same book page)

- [[Vacant Reflection Number Set]] `— → num set`
- [[Vacant Reflection Vector Set]] `— → vec set`
- [[Vacant Reflection Entity Set]] `— → entity set`
- [[Insertion Gambit]] `set, (num|vec|entity) → set, bool`
- [[Ordering Purification]] `set → list`
- [[Disordering Purification]] `[num]|[vec]|[entity] → set`
- [[Bucketing Gambit]] `many, num → set`
- [[Spilling Disintegration]] `set → many`

## Sources

- 📖 **Guidebook:** [[Hash Set Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:set_remove_ret`
