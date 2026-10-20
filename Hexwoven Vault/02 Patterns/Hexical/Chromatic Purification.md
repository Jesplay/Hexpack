---
aliases: ["Chromatic Purification"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/visuals]
kind: "Pattern"
family: "Purification"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:get_dye"
signature: "weedwa"
start_direction: "NORTH_EAST"
input: "id/vector/entity"
output: "dye/null"
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Identifier]]"]
iotas_out: ["[[Null]]", "[[Pigment]]"]
entry: "[[Dyes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Chromatic Purification

◇ **Pattern** · [[Hexical]] · 📖 [[Dyes]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,106.0 52.3,68.3 74.1,30.5 117.7,30.5 139.5,68.3 95.9,68.3 52.3,68.3 74.1,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="30.5,106.0 52.3,68.3 74.1,30.5 117.7,30.5 139.5,68.3 95.9,68.3 52.3,68.3 74.1,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="106.0" x2="37.5" y2="93.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,85.3 41.6,96.3 33.3,91.5" fill="#fc77be"/><circle cx="30.5" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weedwa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `id/vector/entity` ([[Vector]], [[Entity]], [[Identifier]])
- **Leaves:** `dye/null` ([[Null]], [[Pigment]])
- **ID:** `hexical:get_dye`

```text
before   … │ id/vector/entity  ◂ top
after    … │ dye/null          ◂ top
```

## Description

Gets the dye of a block, entity, or block/item identifier.

## Related patterns (same book page)

- [[Dye]] `vector/entity, dye → —`
- [[Vision Purification]] `dye → vector`

## Sources

- 📖 **Guidebook:** [[Dyes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Chromatic Purification](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/dyes@hexical:get_dye)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetDye` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:get_dye`
