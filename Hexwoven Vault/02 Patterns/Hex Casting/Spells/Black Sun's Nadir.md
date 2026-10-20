---
aliases: ["Black Sun's Nadir"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/effects, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Status Effects", "Combat & Damage"]
mod: "Hex Casting"
op_id: "hexcasting:potion/wither"
signature: "qqqqqaewawawe"
start_direction: "SOUTH_WEST"
input: "entity, number, number"
output: ""
cost: "cost is one Amethyst Dust per second."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Nadirs]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Black Sun's Nadir

✦ **Spell** · [[Hex Casting]] · 📖 [[Nadirs]] · Patterns › Spells

**Topics:** [[Status Effects (Topic)|Status Effects]] · [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2 72.3,140.2 114.8,140.2 93.5,103.4 72.3,66.6 93.5,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:595;--dur:3.9s" points="51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2 72.3,140.2 114.8,140.2 93.5,103.4 72.3,66.6 93.5,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="29.8" x2="44.2" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,50.0 40.2,39.2 48.3,43.9" fill="#fc77be"/><circle cx="51.0" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqaewawawe` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, number, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** cost is one Amethyst Dust per second.
- **ID:** `hexcasting:potion/wither`

```text
before   … │ entity │ number │ number  ◂ top
after    …                             ◂ top
```

## Description

Inflicts **withering**. Base cost is one [[Amethyst|Amethyst Dust]] per second.

## Related patterns (same book page)

- [[White Sun's Nadir]] `entity, number, number → —`
- [[Blue Sun's Nadir]] `entity, number → —`
- [[Red Sun's Nadir]] `entity, number, number → —`
- [[Green Sun's Nadir]] `entity, number, number → —`

## Sources

- 📖 **Guidebook:** [[Nadirs]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Black Sun's Nadir](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/nadirs@hexcasting:potion/wither)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPotionEffect` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:potion/wither`
