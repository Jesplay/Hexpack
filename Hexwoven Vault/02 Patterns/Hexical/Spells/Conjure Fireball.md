---
aliases: ["Conjure Fireball"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hexical"
op_id: "hexical:ghast_fireball"
signature: "wqqqqqwaeaeaeaeae"
start_direction: "SOUTH_EAST"
input: "vector"
output: "entity"
cost: "Costs about three Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Projectiles]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Fireball

✦ **Spell** · [[Hexical]] · 📖 [[Projectiles]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 24.5,115.3 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:629;--dur:5.0s" points="24.5,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 24.5,115.3 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="54.7" x2="30.1" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="34.1,71.4 26.7,66.3 33.4,62.5" fill="#fc77be"/><circle cx="24.5" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqqqwaeaeaeaeae` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs about three Amethyst Dust.
- **ID:** `hexical:ghast_fireball`

```text
before   … │ vector  ◂ top
after    … │ entity  ◂ top
```

## Description

Conjures an explosive fireball that can be *percussively propelled*. Costs about three [[Amethyst|Amethyst Dust]].

There exist other spells too complex to be given justice in this chapter—I have dedicated other chapters to them.

[[Magic Missile]] is a standard quick and versatile spear of amethyst that can be produced and fired at arbitrary speed.

I can also [[Pyrotechnics|utilize fireworks]] for battle.

Finally, a [[Dispense|certain spell]] allows the ability to fire potions and arrows.

## Related patterns (same book page)

- [[Conjure Egg]] `vector → entity`
- [[Conjure Spit]] `vector → entity`
- [[Conjure Snowball]] `vector → entity`

## Sources

- 📖 **Guidebook:** [[Projectiles]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Fireball](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/projectiles@hexical:ghast_fireball)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureEntity` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:ghast_fireball`
