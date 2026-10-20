---
aliases: ["Green Sun's Nadir"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/effects, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Status Effects", "Combat & Damage"]
mod: "Hex Casting"
op_id: "hexcasting:potion/slowness"
signature: "qqqqqadwawaw"
start_direction: "SOUTH_EAST"
input: "entity, number, number"
output: ""
cost: "cost is one Amethyst Dust per 5 seconds."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Nadirs]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Green Sun's Nadir

✦ **Spell** · [[Hex Casting]] · 📖 [[Nadirs]] · Patterns › Spells

**Topics:** [[Status Effects (Topic)|Status Effects]] · [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 72.3,66.6 51.0,103.4 29.8,140.2 72.3,140.2 114.8,140.2 93.5,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:553;--dur:3.6s" points="29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 72.3,66.6 51.0,103.4 29.8,140.2 72.3,140.2 114.8,140.2 93.5,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="66.6" x2="36.6" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,86.8 32.5,80.7 40.6,76.0" fill="#fc77be"/><circle cx="29.8" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqadwawaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, number, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** cost is one Amethyst Dust per 5 seconds.
- **ID:** `hexcasting:potion/slowness`

```text
before   … │ entity │ number │ number  ◂ top
after    …                             ◂ top
```

## Description

Inflicts **slowness**. Base cost is one [[Amethyst|Amethyst Dust]] per 5 seconds.

## Related patterns (same book page)

- [[White Sun's Nadir]] `entity, number, number → —`
- [[Blue Sun's Nadir]] `entity, number → —`
- [[Black Sun's Nadir]] `entity, number, number → —`
- [[Red Sun's Nadir]] `entity, number, number → —`

## Sources

- 📖 **Guidebook:** [[Nadirs]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Green Sun's Nadir](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/nadirs@hexcasting:potion/slowness)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPotionEffect` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:potion/slowness`
