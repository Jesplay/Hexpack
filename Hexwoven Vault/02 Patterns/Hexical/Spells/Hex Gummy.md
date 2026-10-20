---
aliases: ["Hex Gummy"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:conjure_gummy"
signature: "eeewdw"
start_direction: "SOUTH_WEST"
input: "vector"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Conjurable Delights]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hex Gummy

✦ **Spell** · [[Hexical]] · 📖 [[Conjurable Delights]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="114.8,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 72.3,29.8 93.5,66.6 114.8,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="114.8,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 72.3,29.8 93.5,66.6 114.8,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="114.8" y1="103.4" x2="108.0" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="103.1,123.7 104.0,112.9 112.1,117.5" fill="#fc77be"/><circle cx="114.8" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeewdw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexical:conjure_gummy`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Conjures a **Hex Gummy**: a delightful light snack that also provides about a tenth of an [[Amethyst|Amethyst Dust]]'s worth of **[[Media|media]]**. Costs about one [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Conjure Hexburst]] `vector, any → —`
- [[Conjure Hextito]] `vector, list of patterns → —`

## Sources

- 📖 **Guidebook:** [[Conjurable Delights]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Hex Gummy](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/conjurable_delights@hexical:conjure_gummy)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureGummy` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:conjure_gummy`
