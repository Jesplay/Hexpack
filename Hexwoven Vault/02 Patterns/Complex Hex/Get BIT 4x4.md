---
aliases: ["Get BIT 4x4"]
tags: [hexcasting/pattern, mod/complex-hex, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Complex Hex"
op_id: "complexhex:get_bit_4x4"
signature: "qwqwqwqwqwqqeawwaeaww"
start_direction: "SOUTH_WEST"
input: "BIT"
output: "matrix"
cost: ""
iotas_in: ["[[BIT]]"]
iotas_out: ["[[Matrix]]"]
entry: "[[BIT Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Get BIT 4x4

◇ **Pattern** · [[Complex Hex]] · 📖 [[BIT Manipulation]] · Patterns › Great Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 53.5,76.6 37.8,103.8 69.3,103.8 100.7,103.8 132.2,103.8 116.5,76.6 132.2,49.3 100.7,49.3 69.3,49.3 37.8,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 53.5,76.6 37.8,103.8 69.3,103.8 100.7,103.8 132.2,103.8 116.5,76.6 132.2,49.3 100.7,49.3 69.3,49.3 37.8,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="49.3" x2="32.7" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="29.1,64.3 29.7,56.3 35.7,59.8" fill="#fc77be"/><circle cx="37.8" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwqwqwqwqwqqeawwaeaww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `BIT` ([[BIT]])
- **Leaves:** `matrix` ([[Matrix]])
- **ID:** `complexhex:get_bit_4x4`

```text
before   … │ BIT     ◂ top
after    … │ matrix  ◂ top
```

## Description

Pushes a 4x4 matrix representing the Bit's Transformation.

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
- [[Set BIT 4x4]] `BIT, matrix → —`

## Sources

- 📖 **Guidebook:** [[BIT Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Get BIT 4x4](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/great_spells/bits@complexhex:get_bit_4x4)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetBit4x4` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:get_bit_4x4`
