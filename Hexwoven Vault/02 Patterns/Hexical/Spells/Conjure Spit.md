---
aliases: ["Conjure Spit"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hexical"
op_id: "hexical:llama_spit"
signature: "dwqaqw"
start_direction: "EAST"
input: "vector"
output: "entity"
cost: "Costs about a fourth of one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Projectiles]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Spit

✦ **Spell** · [[Hexical]] · 📖 [[Projectiles]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,29.8 72.3,29.8 51.0,66.6 29.8,103.4 51.0,140.2 72.3,103.4 51.0,66.6 29.8,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="29.8,29.8 72.3,29.8 51.0,66.6 29.8,103.4 51.0,140.2 72.3,103.4 51.0,66.6 29.8,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="29.8" x2="43.4" y2="29.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="53.2,29.8 43.4,34.4 43.4,25.1" fill="#fc77be"/><circle cx="29.8" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwqaqw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs about a fourth of one Amethyst Dust.
- **ID:** `hexical:llama_spit`

```text
before   … │ vector  ◂ top
after    … │ entity  ◂ top
```

## Description

Conjures a sticky low-damaging projectile. Costs about a fourth of one [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Conjure Egg]] `vector → entity`
- [[Conjure Snowball]] `vector → entity`
- [[Conjure Fireball]] `vector → entity`

## Sources

- 📖 **Guidebook:** [[Projectiles]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Spit](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/projectiles@hexical:llama_spit)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureEntity` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:llama_spit`
