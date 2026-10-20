---
aliases: ["Dye"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:dye"
signature: "dwaqqw"
start_direction: "NORTH_WEST"
input: "vector/entity, dye"
output: ""
cost: "Costs about an eighth of one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Pigment]]"]
iotas_out: []
entry: "[[Dyes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dye

✦ **Spell** · [[Hexical]] · 📖 [[Dyes]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="74.1,106.0 52.3,68.3 95.9,68.3 139.5,68.3 117.7,30.5 74.1,30.5 52.3,68.3 30.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="74.1,106.0 52.3,68.3 95.9,68.3 139.5,68.3 117.7,30.5 74.1,30.5 52.3,68.3 30.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="74.1" y1="106.0" x2="67.1" y2="93.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.1,85.3 71.3,91.5 63.0,96.3" fill="#fc77be"/><circle cx="74.1" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwaqqw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector/entity, dye` ([[Vector]], [[Entity]], [[Pigment]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about an eighth of one Amethyst Dust.
- **ID:** `hexical:dye`

```text
before   … │ vector/entity │ dye  ◂ top
after    …                        ◂ top
```

## Description

Dyes a dyeable block or entity. Costs about an eighth of one [[Amethyst|Amethyst Dust]].

The valid block targets for this spell include beds, candles, cakes with candles, carpets, concrete, concrete powder, glazed terracotta, sand into red sand and vice versa, sandstone and all blocks derived from it into its red variant and vice versa, shulker boxes, stained glass, stained glass panes, terracotta, tulips into their multiple colors, and wool blocks. As for entities, cat collars, dog collars, specklikes, shulkers, and item entities of all of the above blocks are valid.

## Related patterns (same book page)

- [[Chromatic Purification]] `id/vector/entity → dye/null`
- [[Vision Purification]] `dye → vector`

## Sources

- 📖 **Guidebook:** [[Dyes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Dye](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/dyes@hexical:dye)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpDye` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:dye`
