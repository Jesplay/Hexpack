---
aliases: ["Shrink/Grow Armor Stand"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/creatures]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities"]
mod: "YAHA"
op_id: "yaha:stand_toggle_tiny"
signature: "adaaea"
start_direction: "EAST"
input: "entity"
output: ""
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Armor Stand Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Shrink/Grow Armor Stand

✦ **Spell** · [[YAHA]] · 📖 [[Armor Stand Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,121.6 110.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 35.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="60.0,121.6 110.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 35.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="121.6" x2="76.0" y2="121.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="87.5,121.6 76.0,127.1 76.0,116.1" fill="#fc77be"/><circle cx="60.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adaaea` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `yaha:stand_toggle_tiny`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Gives the armor stand a smaller form. Casting again brings it back to its original size.

## Related patterns (same book page)

- [[ShowHide Arms]] `entity → —`
- [[ShowHide Baseplate]] `entity → —`
- [[Rotate Armor Stand]] `entity, number → —`
- [[Orient HeadBody]] `entity, vector → —`
- [[Orient LeftRight Arm]] `entity, vector → —`
- [[Orient LeftRight Leg]] `entity, vector → —`

## Sources

- 📖 **Guidebook:** [[Armor Stand Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `yaha:stand_toggle_tiny`
