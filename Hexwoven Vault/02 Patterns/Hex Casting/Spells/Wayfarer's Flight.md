---
aliases: ["Wayfarer's Flight"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Movement"]
mod: "Hex Casting"
op_id: "hexcasting:flight/time"
signature: "dwdwdewq"
start_direction: "NORTH_EAST"
input: "entity, number"
output: ""
cost: "Costs about 2 Amethyst Dust per second of flight; I believe it is best suited for travel."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Flight]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Wayfarer's Flight

✦ **Spell** · [[Hex Casting]] · 📖 [[Flight]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="52.5" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.0,42.1 56.2,51.9 48.9,47.7" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwdwdewq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about 2 Amethyst Dust per second of flight; I believe it is best suited for travel.
- **ID:** `hexcasting:flight/time`

```text
before   … │ entity │ number  ◂ top
after    …                    ◂ top
```

## Description

A flight limited in its duration.

The second argument is an amount of time in seconds for which the spell is stable. After that time, the spell ends and I am dropped from the sky.

Costs about 2 [[Amethyst|Amethyst Dust]] per second of flight; I believe it is best suited for travel.

## Related patterns (same book page)

- [[Anchorite's Flight]] `entity, number → —`
- [[Aviator's Purification]] `entity → boolean`

## Sources

- 📖 **Guidebook:** [[Flight]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Wayfarer's Flight](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/flight@hexcasting:flight/time)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpFlight` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:flight/time`
