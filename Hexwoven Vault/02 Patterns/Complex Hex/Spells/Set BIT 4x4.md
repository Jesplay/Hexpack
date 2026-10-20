---
aliases: ["Set BIT 4x4"]
tags: [hexcasting/spell, mod/complex-hex, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Complex Hex"
op_id: "complexhex:set_bit_4x4"
signature: "eweweweweweewwdqdwwdq"
start_direction: "NORTH_EAST"
input: "BIT, matrix"
output: ""
cost: ""
iotas_in: ["[[Matrix]]", "[[BIT]]"]
iotas_out: []
entry: "[[BIT Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Set BIT 4x4

✦ **Spell** · [[Complex Hex]] · 📖 [[BIT Manipulation]] · Patterns › Great Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 69.3,49.3 100.7,49.3 132.2,49.3 116.5,76.6 132.2,103.8 100.7,103.8 69.3,103.8 37.8,103.8 53.5,76.6 37.8,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 69.3,49.3 100.7,49.3 132.2,49.3 116.5,76.6 132.2,103.8 100.7,103.8 69.3,103.8 37.8,103.8 53.5,76.6 37.8,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="49.3" x2="42.8" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.4,34.3 45.8,42.3 39.8,38.8" fill="#fc77be"/><circle cx="37.8" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eweweweweweewwdqdwwdq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `BIT, matrix` ([[Matrix]], [[BIT]])
- **Leaves:** `nothing` (—)
- **ID:** `complexhex:set_bit_4x4`

```text
before   … │ BIT │ matrix  ◂ top
after    …                 ◂ top
```

## Description

Sets the Transformation of a BIT to the given 4x4 matrix.

## Related patterns (same book page)

- [[Summon Block Display]] `vector, string → —`
- [[Summon Item Display]] `vector, string → —`
- [[Summon Text Display]] `vector, string → —`
- [[Entity Purification BIT]] `vector → entity`
- [[Zone Distillation BIT]] `vector, number → [entity]`
- [[Zone Distillation Not BIT]] `vector, number → [entity]`
- [[Translate BIT]] `BIT, vec → —`
- [[Rotate BIT]] `BIT, quat → —`
- [[Scale BIT]] `BIT, vec → —`
- [[Kill BIT]] `BIT → —`
- [[Get BIT 4x4]] `BIT → matrix`

## Sources

- 📖 **Guidebook:** [[BIT Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Set BIT 4x4](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/great_spells/bits@complexhex:set_bit_4x4)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSetBit4x4` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `complexhex:set_bit_4x4`
