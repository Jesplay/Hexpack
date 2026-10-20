---
aliases: ["Conjure Hextito"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:conjure_hextito"
signature: "qaqdqaqdwawaw"
start_direction: "EAST"
input: "vector, list of patterns"
output: ""
cost: "Costs about two Amethyst Dust and is subject to the Transgress Others mishap."
iotas_in: ["[[Vector]]", "[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Conjurable Delights]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Hextito

✦ **Spell** · [[Hexical]] · 📖 [[Conjurable Delights]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="85.0,60.5 123.6,60.5 143.0,27.0 104.3,27.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 85.0,60.5 65.7,94.0 46.4,127.4 85.0,127.4 123.6,127.4 104.3,94.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="85.0,60.5 123.6,60.5 143.0,27.0 104.3,27.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 85.0,60.5 65.7,94.0 46.4,127.4 85.0,127.4 123.6,127.4 104.3,94.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="60.5" x2="97.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="106.2,60.5 97.4,64.8 97.4,56.3" fill="#fc77be"/><circle cx="85.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqdqaqdwawaw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, list of patterns` ([[Vector]], [[List]], [[Pattern]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about two Amethyst Dust and is subject to the Transgress Others mishap.
- **ID:** `hexical:conjure_hextito`

```text
before   … │ vector │ list of patterns  ◂ top
after    …                              ◂ top
```

## Description

Conjures a **Hextito** of the given **Hex** at the location. Costs about two [[Amethyst|Amethyst Dust]] and is subject to the Transgress Others [[Mishaps|mishap]].

**Hextitos** are triangular crunchy snacks I can conjure. When consumed, the consumer immediately casts its **Hex**, using their [[Staves|Staff]]'s [[The Stack|stack]] unlike a regular [[Casting Items|casting device]]. I shall find them useful to make simple actions like raycasts or to give out limited uses of a **Hex** I have designed. If a consumer's stack has an unclosed [[Introspection]], it will not cast, making it quite safe to hand out.

## Related patterns (same book page)

- [[Hex Gummy]] `vector → —`
- [[Conjure Hexburst]] `vector, any → —`

## Sources

- 📖 **Guidebook:** [[Conjurable Delights]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Hextito](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/conjurable_delights@hexical:conjure_hextito)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureHextito` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:conjure_hextito`
