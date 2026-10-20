---
aliases: ["Conjure Snowball"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hexical"
op_id: "hexical:snowball"
signature: "ddeeeeewd"
start_direction: "NORTH_EAST"
input: "vector"
output: "entity"
cost: "Costs about half of an Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Projectiles]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Snowball

✦ **Spell** · [[Hexical]] · 📖 [[Projectiles]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,103.4 72.3,66.6 93.5,103.4 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 72.3,140.2 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="51.0,103.4 72.3,66.6 93.5,103.4 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 72.3,140.2 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="103.4" x2="57.8" y2="91.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.7,83.2 61.9,94.0 53.8,89.3" fill="#fc77be"/><circle cx="51.0" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddeeeeewd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs about half of an Amethyst Dust.
- **ID:** `hexical:snowball`

```text
before   … │ vector  ◂ top
after    … │ entity  ◂ top
```

## Description

Conjures a harmless snowball. Costs about half of an [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Conjure Egg]] `vector → entity`
- [[Conjure Spit]] `vector → entity`
- [[Conjure Fireball]] `vector → entity`

## Sources

- 📖 **Guidebook:** [[Projectiles]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Snowball](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/projectiles@hexical:snowball)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureEntity` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:snowball`
