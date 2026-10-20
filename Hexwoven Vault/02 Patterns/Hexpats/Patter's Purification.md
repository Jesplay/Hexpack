---
aliases: ["Patter's Purification"]
tags: [hexcasting/pattern, mod/hexpats, family/purification, topic/misc]
kind: "Pattern"
family: "Purification"
topics: ["Miscellaneous"]
mod: "Hexpats"
op_id: "hexpats:check_fire"
signature: "qqqqqwqw"
start_direction: "WEST"
input: "entity"
output: "bool"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Comfort Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Patter's Purification

◇ **Pattern** · [[Hexpats]] · 📖 [[Comfort Patterns]] · Patterns

**Topics:** [[Miscellaneous (Topic)|Miscellaneous]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="123.6,60.5 85.0,60.5 65.7,94.0 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="123.6,60.5 85.0,60.5 65.7,94.0 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="60.5" x2="111.3" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.4,60.5 111.3,56.3 111.3,64.8" fill="#fc77be"/><circle cx="123.6" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwqw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexpats:check_fire`

```text
before   … │ entity  ◂ top
after    … │ bool    ◂ top
```

## Description

This pattern checks to see if an entity needs pats to stop burning.

## Related patterns (same book page)

- [[Patter's Gambit]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Comfort Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpats on Modrinth](https://modrinth.com/mod/dmOz7d0b)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CheckFire` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpats:check_fire`
