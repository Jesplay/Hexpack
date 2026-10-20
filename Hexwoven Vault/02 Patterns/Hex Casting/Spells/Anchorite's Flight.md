---
aliases: ["Anchorite's Flight"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Movement"]
mod: "Hex Casting"
op_id: "hexcasting:flight/range"
signature: "awawaawq"
start_direction: "SOUTH_WEST"
input: "entity, number"
output: ""
cost: "Costs about 2 Amethyst Dust per meter of safety, though any radius less than 1 meter will still cost 2 Amethyst Dust."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Flight]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Anchorite's Flight

✦ **Spell** · [[Hex Casting]] · 📖 [[Flight]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 85.0,60.5 65.7,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 85.0,60.5 65.7,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="40.2" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,78.9 36.5,69.1 43.9,73.3" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awawaawq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about 2 Amethyst Dust per meter of safety, though any radius less than 1 meter will still cost 2 Amethyst Dust.
- **ID:** `hexcasting:flight/range`

```text
before   … │ entity │ number  ◂ top
after    …                    ◂ top
```

## Description

A flight limited in its range.

The second argument is a horizontal radius, in meters, in which the spell is stable. Moving outside of that radius will end the spell, dropping me out of the sky. As long as I stay inside the safe zone, however, the spell lasts indefinitely. An additional shimmer of **[[Media|media]]** marks the origin point of the safe zone.

Costs about 2 [[Amethyst|Amethyst Dust]] per meter of safety, though any radius less than 1 meter will still cost 2 [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Wayfarer's Flight]] `entity, number → —`
- [[Aviator's Purification]] `entity → boolean`

## Sources

- 📖 **Guidebook:** [[Flight]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Anchorite's Flight](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/flight@hexcasting:flight/range)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpFlight` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:flight/range`
