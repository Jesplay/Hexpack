---
aliases: ["Rotate BIT"]
tags: [hexcasting/spell, mod/complex-hex, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Complex Hex"
op_id: "complexhex:rotatebit"
signature: "wqwqwqwqwqwaqeeeeedww"
start_direction: "SOUTH_EAST"
input: "BIT, quat"
output: ""
cost: ""
iotas_in: ["[[Quaternion]]", "[[BIT]]"]
iotas_out: []
entry: "[[BIT Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Rotate BIT

✦ **Spell** · [[Complex Hex]] · 📖 [[BIT Manipulation]] · Patterns › Great Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 116.5,76.6 100.7,103.8 69.3,103.8 53.5,76.6 85.0,76.6 116.5,76.6 148.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 116.5,76.6 100.7,103.8 69.3,103.8 53.5,76.6 85.0,76.6 116.5,76.6 148.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="76.6" x2="27.1" y2="85.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,91.6 24.1,87.0 30.1,83.6" fill="#fc77be"/><circle cx="22.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqwqwaqeeeeedww` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `BIT, quat` ([[Quaternion]], [[BIT]])
- **Leaves:** `nothing` (—)
- **ID:** `complexhex:rotatebit`

```text
before   … │ BIT │ quat  ◂ top
after    …               ◂ top
```

## Description

Rotates the BIT to the given Quaternion.

## Related patterns (same book page)

- [[Summon Block Display]] `vector, string → —`
- [[Summon Item Display]] `vector, string → —`
- [[Summon Text Display]] `vector, string → —`
- [[Entity Purification BIT]] `vector → entity`
- [[Zone Distillation BIT]] `vector, number → [entity]`
- [[Zone Distillation Not BIT]] `vector, number → [entity]`
- [[Translate BIT]] `BIT, vec → —`
- [[Scale BIT]] `BIT, vec → —`
- [[Kill BIT]] `BIT → —`
- [[Get BIT 4x4]] `BIT → matrix`
- [[Set BIT 4x4]] `BIT, matrix → —`

## Sources

- 📖 **Guidebook:** [[BIT Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Rotate BIT](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/great_spells/bits@complexhex:rotatebit)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpRotateBIT` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `complexhex:rotatebit`
