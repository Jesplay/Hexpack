---
aliases: ["Appendage"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/circles]
kind: "Spell"
family: "Spell"
topics: ["Spell Circles, Loci & Engines"]
mod: "Hexical"
op_id: "hexical:absorb_arm"
signature: "aaqqadaqwqa"
start_direction: "WEST"
input: "vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Circle Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Appendage

✦ **Spell** · [[Hexical]] · 📖 [[Circle Spells]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="104.3,60.5 65.7,60.5 85.0,94.0 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="104.3,60.5 65.7,60.5 85.0,94.0 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="60.5" x2="92.0" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="83.1,60.5 92.0,56.3 92.0,64.8" fill="#fc77be"/><circle cx="104.3" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqqadaqwqa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:absorb_arm`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Allows a [[Spell Circles|circle]] to hook onto a [[Pedestals|Pedestal]] within ambit to use as its *other arm*. Useful for certain spells that demand an item held in my other hand. It also works as a [[Wristpocket (Book Entry)|Wristpocket]].

## Related patterns (same book page)

- [[Displace]] `entity, vector → —`
- [[Export Media]] `vector, number → —`

## Sources

- 📖 **Guidebook:** [[Circle Spells]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpAbsorbArm` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:absorb_arm`
