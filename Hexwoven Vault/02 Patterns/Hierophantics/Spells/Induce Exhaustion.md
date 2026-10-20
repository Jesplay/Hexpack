---
aliases: ["Induce Exhaustion"]
tags: [hexcasting/spell, mod/hierophantics, family/spell, topic/minds, topic/creatures]
kind: "Spell"
family: "Spell"
topics: ["Minds & Hierophantics", "Creatures & Entities"]
mod: "Hierophantics"
op_id: "hierophantics:villager_sleep"
signature: "qqwqqwqqqqq"
start_direction: "SOUTH_WEST"
input: "villager"
output: ""
cost: "Costs one Charged Amethyst."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Induce Exhaustion (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Induce Exhaustion

✦ **Spell** · [[Hierophantics]] · 📖 [[Induce Exhaustion (Book Entry)]] · Patterns › Spells

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]] · [[Creatures & Entities (Topic)|Creatures & Entities]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 46.4,60.5 65.7,94.0 104.3,94.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 46.4,60.5 65.7,94.0 104.3,94.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="40.2" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,78.9 36.5,69.1 43.9,73.3" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqwqqwqqqqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `villager` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one Charged Amethyst.
- **ID:** `hierophantics:villager_sleep`

```text
before   … │ villager  ◂ top
after    …             ◂ top
```

## Description

Causes the villager to seek out its bed and take a short nap. Not effective if the villager lacks a bed to use. Costs one [[Amethyst|Charged Amethyst]].

## Sources

- 📖 **Guidebook:** [[Induce Exhaustion (Book Entry)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpVillagerSleep` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:villager_sleep`
