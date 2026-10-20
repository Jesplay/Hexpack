---
aliases: ["Conjure Trident"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "YAHA"
op_id: "yaha:trident"
signature: "ddwwdaaeaa"
start_direction: "SOUTH_EAST"
input: "vec"
output: "entity"
cost: "Costs one Charged Amethyst."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Caster's Armaments]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Trident

✦ **Spell** · [[YAHA]] · 📖 [[Caster's Armaments]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,115.3 59.4,145.5 24.5,145.5 41.9,115.3 59.4,85.0 76.9,54.7 94.4,85.0 111.8,54.7 76.9,54.7 59.4,24.5 41.9,54.7 76.9,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:384;--dur:3.1s" points="41.9,115.3 59.4,145.5 24.5,145.5 41.9,115.3 59.4,85.0 76.9,54.7 94.4,85.0 111.8,54.7 76.9,54.7 59.4,24.5 41.9,54.7 76.9,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="115.3" x2="47.5" y2="125.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.6,131.9 44.2,126.9 50.9,123.0" fill="#fc77be"/><circle cx="41.9" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddwwdaaeaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs one Charged Amethyst.
- **ID:** `yaha:trident`

```text
before   … │ vec     ◂ top
after    … │ entity  ◂ top
```

## Description

Summons a trident that pierces mobs and pulls them inwards. Costs one [[Amethyst|Charged Amethyst]]. Adds itself onto the [[The Stack|stack]].

Shatters after piercing 3 mobs, or against any block. Damage dealt is identical to a conjured anvil with the same velocity.

Get over here!

## Related patterns (same book page)

- [[Conjure Paper Plane]] `vector, entity → entity`
- [[Aima Purification]] `entity → entity | null`
- [[Conjure Time Bomb]] `vec, [pattern], num, num → entity`
- [[Kavruka Reflection]] `— → vector`
- [[Conjure Anvil]] `vec → entity`

## Sources

- 📖 **Guidebook:** [[Caster's Armaments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpTrident` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `yaha:trident`
