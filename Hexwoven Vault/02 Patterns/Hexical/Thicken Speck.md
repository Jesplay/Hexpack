---
aliases: ["Thicken Speck"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:thickness_specklike"
signature: "adeeqw"
start_direction: "SOUTH_WEST"
input: "speck entity, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Specks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Thicken Speck

◇ **Pattern** · [[Hexical]] · 📖 [[Specks]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="132.2,22.0 116.5,49.3 148.0,49.3 132.2,76.6 100.7,76.6 85.0,49.3 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:2.0s" points="132.2,22.0 116.5,49.3 148.0,49.3 132.2,76.6 100.7,76.6 85.0,49.3 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="132.2" y1="22.0" x2="127.2" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="123.6,37.0 124.2,29.0 130.2,32.5" fill="#fc77be"/><circle cx="132.2" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeeqw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `speck entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:thickness_specklike`

```text
before   … │ speck entity │ number  ◂ top
after    …                          ◂ top
```

## Description

Changes the stroke thickness of a pattern speck, can range from 0 to 10. Represents twentieths of a block, and is zero by default.

## Related patterns (same book page)

- [[Conjure Speck]] `any, vector, vector → entity`
- [[Move Speck]] `speck entity, vector → —`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Alter Speck]] `speck entity, any → —`
- [[Time Speck]] `speck entity, number → —`
- [[Resize Speck]] `speck entity, number → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Dismiss Speck]] `speck entity → —`
- [[Zone Distillation Speck]] `vector, number → list of entities`

## Sources

- 📖 **Guidebook:** [[Specks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Thicken Speck](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks@hexical:thickness_specklike)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpecklikeProperty` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:thickness_specklike`
