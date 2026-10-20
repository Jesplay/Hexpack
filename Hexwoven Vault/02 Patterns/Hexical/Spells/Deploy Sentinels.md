---
aliases: ["Deploy Sentinels"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/sentinels]
kind: "Spell"
family: "Spell"
topics: ["Sentinels"]
mod: "Hexical"
op_id: "hexical:set_lesser_sentinels"
signature: "aeaae"
start_direction: "EAST"
input: "list of vectors"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[List]]"]
iotas_out: []
entry: "[[Lesser Sentinels]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Deploy Sentinels

✦ **Spell** · [[Hexical]] · 📖 [[Lesser Sentinels]] · Patterns › Spells

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,132.0 92.3,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="38.0,132.0 92.3,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="132.0" x2="55.4" y2="132.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.8,132.0 55.4,138.0 55.4,126.0" fill="#fc77be"/><circle cx="38.0" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeaae` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list of vectors` ([[Vector]], [[List]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:set_lesser_sentinels`

```text
before   … │ list of vectors  ◂ top
after    …                    ◂ top
```

## Description

Dismisses any existing **Lesser Sentinels** and deploys a **Lesser Sentinel** at every position in the list.

## Related patterns (same book page)

- [[Infiltration Reflection]] `— → list of vectors`

## Sources

- 📖 **Guidebook:** [[Lesser Sentinels]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Deploy Sentinels](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/lesser_sentinels@hexical:set_lesser_sentinels)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpLesserSentinelSet` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:set_lesser_sentinels`
