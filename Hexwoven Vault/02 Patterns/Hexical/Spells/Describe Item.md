---
aliases: ["Describe Item"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:item_lore"
signature: "dwewdweedwa"
start_direction: "NORTH_WEST"
input: "entity, list of text/null"
output: ""
cost: ""
iotas_in: ["[[Entity]]", "[[List]]", "[[Null]]", "[[String]]"]
iotas_out: []
entry: "[[Item Renaming]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Describe Item

✦ **Spell** · [[Hexical]] · 📖 [[Item Renaming]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 27.0,27.0 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 104.3,94.0 65.7,94.0 46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="46.4,60.5 27.0,27.0 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 104.3,94.0 65.7,94.0 46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="40.2" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,42.1 43.9,47.7 36.5,51.9" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwewdweedwa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, list of text/null` ([[Entity]], [[List]], [[Null]], [[String]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:item_lore`

```text
before   … │ entity │ list of text/null  ◂ top
after    …                               ◂ top
```

## Description

Takes an item and a list of text to append as a description. Clears if given Null. Free.

## Related patterns (same book page)

- [[Name Item]] `entity, text/null → —`

## Sources

- 📖 **Guidebook:** [[Item Renaming]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpItemLore` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:item_lore`
