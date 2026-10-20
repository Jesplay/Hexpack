---
aliases: ["Conjure Egg"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hexical"
op_id: "hexical:egg"
signature: "qqqwaqaaqeeewdedde"
start_direction: "SOUTH_EAST"
input: "vector"
output: "entity"
cost: "Costs about two Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Projectiles]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Egg

✦ **Spell** · [[Hexical]] · 📖 [[Projectiles]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="169" viewBox="0 0 170.0 168.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/><circle cx="24.3" cy="144.5" r="2"/><circle cx="59.0" cy="144.5" r="2"/><circle cx="93.7" cy="144.5" r="2"/><circle cx="128.4" cy="144.5" r="2"/><circle cx="163.1" cy="144.5" r="2"/></g><polyline class="hexpat-base" points="76.3,114.4 93.7,144.5 128.4,144.5 145.7,114.4 128.4,84.4 111.0,54.3 93.7,84.4 111.0,114.4 128.4,84.4 93.7,84.4 76.3,114.4 41.6,114.4 24.3,84.4 41.6,54.3 59.0,24.3 76.3,54.3 59.0,84.4 41.6,54.3 76.3,54.3 93.7,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:659;--dur:5.3s" points="76.3,114.4 93.7,144.5 128.4,144.5 145.7,114.4 128.4,84.4 111.0,54.3 93.7,84.4 111.0,114.4 128.4,84.4 93.7,84.4 76.3,114.4 41.6,114.4 24.3,84.4 41.6,54.3 59.0,24.3 76.3,54.3 59.0,84.4 41.6,54.3 76.3,54.3 93.7,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="114.4" x2="81.9" y2="124.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="85.9,130.9 78.6,125.9 85.2,122.1" fill="#fc77be"/><circle cx="76.3" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqwaqaaqeeewdedde` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs about two Amethyst Dust.
- **ID:** `hexical:egg`

```text
before   … │ vector  ◂ top
after    … │ entity  ◂ top
```

## Description

Conjures an egg. May be fertile. Costs about two [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Conjure Spit]] `vector → entity`
- [[Conjure Snowball]] `vector → entity`
- [[Conjure Fireball]] `vector → entity`

## Sources

- 📖 **Guidebook:** [[Projectiles]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Egg](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/projectiles@hexical:egg)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureEntity` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:egg`
