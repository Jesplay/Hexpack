---
aliases: ["Fireball"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hex Casting"
op_id: "hexcasting:explode/fire"
signature: "ddwddwdd"
start_direction: "EAST"
input: "vector, number"
output: ""
cost: "Costs one Amethyst Dust, plus about 3 extra Amethyst Dusts per point of explosion power."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Basic Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Fireball

✦ **Spell** · [[Hex Casting]] · 📖 [[Basic Spells]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,121.6 85.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3 60.0,121.6 35.0,78.3 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:2.5s" points="85.0,78.3 135.0,78.3 110.0,121.6 85.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3 60.0,121.6 35.0,78.3 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddwddwdd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one Amethyst Dust, plus about 3 extra Amethyst Dusts per point of explosion power.
- **ID:** `hexcasting:explode/fire`

```text
before   … │ vector │ number  ◂ top
after    …                    ◂ top
```

## Description

Remove a number and vector from the [[The Stack|stack]], then create a fiery explosion at the given location with the given power.

Costs one [[Amethyst|Amethyst Dust]], plus about 3 extra [[Amethyst|Amethyst Dust]]s per point of explosion power. Otherwise, the same as [[Explosion]], except with fire.

## Related patterns (same book page)

- [[Explosion]] `vector, number → —`
- [[Impulse]] `entity, vector → —`
- [[Blink]] `entity, number → —`
- [[Make Note]] `vector, number, number → —`

## Sources

- 📖 **Guidebook:** [[Basic Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Fireball](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/basic@hexcasting:explode/fire)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpExplode` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:explode/fire`
