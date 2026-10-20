---
aliases: ["Switch"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:set_hotbar"
signature: "dwewdwe"
start_direction: "WEST"
input: "number"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Hotbar]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Switch

◇ **Pattern** · [[Hexical]] · 📖 [[Hotbar]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 27.0,94.0 46.4,60.5 65.7,27.0 104.3,27.0 143.0,27.0 123.6,60.5 104.3,94.0 65.7,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="65.7,94.0 27.0,94.0 46.4,60.5 65.7,27.0 104.3,27.0 143.0,27.0 123.6,60.5 104.3,94.0 65.7,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="53.3" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.4,94.0 53.3,89.7 53.3,98.2" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwewdwe` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:set_hotbar`

```text
before   … │ number  ◂ top
after    …           ◂ top
```

## Description

Takes a number 0 to 8 and switches my current item to it.

*Remember, switching to your pistol is always faster than reloading.*

## Related patterns (same book page)

- [[Handy Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Hotbar]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetHotbar` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:set_hotbar`
