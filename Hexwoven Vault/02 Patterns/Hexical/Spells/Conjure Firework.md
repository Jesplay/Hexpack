---
aliases: ["Conjure Firework"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:conjure_firework"
signature: "dedwaqwwawwqa"
start_direction: "SOUTH_WEST"
input: "vec, vec, num, num, list of vec, list of vec, bool, bool"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Boolean]]", "[[List]]"]
iotas_out: []
entry: "[[Pyrotechnics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Firework

✦ **Spell** · [[Hexical]] · 📖 [[Pyrotechnics]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="94.4,115.3 76.9,145.5 59.4,115.3 76.9,85.0 94.4,115.3 111.8,145.5 129.3,115.3 111.8,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:489;--dur:3.9s" points="94.4,115.3 76.9,145.5 59.4,115.3 76.9,85.0 94.4,115.3 111.8,145.5 129.3,115.3 111.8,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="94.4" y1="115.3" x2="88.8" y2="125.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.8,131.9 85.4,123.0 92.1,126.9" fill="#fc77be"/><circle cx="94.4" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedwaqwwawwqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec, num, num, list of vec, list of vec, bool, bool` ([[Number]], [[Vector]], [[Boolean]], [[List]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:conjure_firework`

```text
before   … │ vec │ vec │ num │ num │ list of vec │ list of vec │ bool │ bool  ◂ top
after    …                                                                    ◂ top
```

## Description

Conjures a firework of my *exact* specifications. It may be one of the most complex spells in existence. Costs equivalently to the other firework spell.

*Basic Theory*

The first two parameters of the spell are simple position and velocity. The third parameter dictates flight duration as a number from 1 to 3. I can imagine it as being the amount of **Gunpowder** I would add to a firework rocket.

The remaining parameters can be imagined as specifications for a **virtual firework star.**

*Shape*

The fourth parameter controls the shape of the explosion, based off a number 0 to 4 inclusive.
- 0 is a simple, small ball-shaped explosion.
- 1 is a large ball-shaped explosion.
- 2 is a star-shaped explosion.
- 3 is a large creeper face-shaped explosion.
- 4 is a conic burst.

*Colors*

The fifth and sixth parameters are lists of vectors, representing colors. The first list maps to the colors of the initial explosions, while the second maps to the colors that the particles fade into. There must be at least one vector specified for the first list, but the second list may be empty.

The vector components represent redness, greenness, and blueness on a scale of 0 to 1 respectively.

*Special Effects*

The seventh parameter controls whether the particles of the firework flicker, as if adding glowstone to the **virtual Firework Star**.

The eighth parameter controls whether the particles of the firework leave trails, as if adding a diamond to the **virtual Firework Star**.

## Related patterns (same book page)

- [[Simulate Firework]] `vector, vector, number → —`

## Sources

- 📖 **Guidebook:** [[Pyrotechnics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Firework](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/pyrotechnics@hexical:conjure_firework)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureFirework` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:conjure_firework`
