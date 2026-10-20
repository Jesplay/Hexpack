---
aliases: ["Conjure Hexburst"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:conjure_hexburst"
signature: "aadaadqaq"
start_direction: "EAST"
input: "vector, any"
output: ""
cost: "Costs about one Amethyst Dust and is subject to the Transgress Others mishap."
iotas_in: ["[[Vector]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Conjurable Delights]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Hexburst

✦ **Spell** · [[Hexical]] · 📖 [[Conjurable Delights]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,66.6 114.8,66.6 93.5,29.8 72.3,66.6 51.0,29.8 29.8,66.6 72.3,66.6 51.0,103.4 72.3,140.2 93.5,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="72.3,66.6 114.8,66.6 93.5,29.8 72.3,66.6 51.0,29.8 29.8,66.6 72.3,66.6 51.0,103.4 72.3,140.2 93.5,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="66.6" x2="85.9" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="95.7,66.6 85.9,71.3 85.9,61.9" fill="#fc77be"/><circle cx="72.3" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aadaadqaq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, any` ([[Vector]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust and is subject to the Transgress Others mishap.
- **ID:** `hexical:conjure_hexburst`

```text
before   … │ vector │ any  ◂ top
after    …                 ◂ top
```

## Description

Conjures a **Hexburst** of the given [[Iotas|iota]] at the location. Costs about one [[Amethyst|Amethyst Dust]] and is subject to the Transgress Others [[Mishaps|mishap]].

**Hexbursts** are edible items created by taking an iota and wrapping **[[Media|media]]** around it, over and over until it gains size. When eaten, a **Hexburst** instantly adds its iota to the consumer's [[The Stack|stack]]. I shall find this property considerably useful if I ever want to trade my entity reference. Should my stack have an unclosed [[Introspection]], it pushes it into the forming list.

## Related patterns (same book page)

- [[Hex Gummy]] `vector → —`
- [[Conjure Hextito]] `vector, list of patterns → —`

## Sources

- 📖 **Guidebook:** [[Conjurable Delights]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Hexburst](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/conjurable_delights@hexical:conjure_hexburst)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureHexburst` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:conjure_hexburst`
