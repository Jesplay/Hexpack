---
aliases: ["Explosion"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hex Casting"
op_id: "hexcasting:explode"
signature: "aawaawaa"
start_direction: "EAST"
input: "vector, number"
output: ""
cost: "Costs a negligible amount at power 0, plus 3 extra Amethyst Dust per point of explosion power."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Basic Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Explosion

✦ **Spell** · [[Hex Casting]] · 📖 [[Basic Spells]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3 60.0,35.0 35.0,78.3 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:2.5s" points="85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3 60.0,35.0 35.0,78.3 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawaawaa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a negligible amount at power 0, plus 3 extra Amethyst Dust per point of explosion power.
- **ID:** `hexcasting:explode`

```text
before   … │ vector │ number  ◂ top
after    …                    ◂ top
```

## Description

Remove a number and vector from the [[The Stack|stack]], then create an explosion at the given location with the given power.

A power of 3 is about as much as a Creeper's blast; 4 is about as much as a TNT blast. Nature refuses to give me a blast of more than 10 power, though.

Strangely, this explosion doesn't seem to harm me. Perhaps it's because I am the one exploding?

Costs a negligible amount at power 0, plus 3 extra [[Amethyst|Amethyst Dust]] per point of explosion power.

## Example

Blow up the block you're looking at: [[Archer's Distillation]] setup → [[Numerical Reflection]] `3` → **Explosion**. Power 3 is about a creeper. It costs 3 dust per point of power, so `3` costs about 9 dust.

## Related patterns (same book page)

- [[Fireball]] `vector, number → —`
- [[Impulse]] `entity, vector → —`
- [[Blink]] `entity, number → —`
- [[Make Note]] `vector, number, number → —`

## Sources

- 📖 **Guidebook:** [[Basic Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Explosion](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/basic@hexcasting:explode)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpExplode` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:explode`
