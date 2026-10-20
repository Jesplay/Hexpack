---
aliases: ["Illuminate Ink"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI", "Items & Crafting"]
mod: "Hexical"
op_id: "hexical:glow_scroll"
signature: "wqwawqwqawawaewdwdw"
start_direction: "SOUTH_WEST"
input: "entity"
output: ""
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Animated Scrolls]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Illuminate Ink

✦ **Spell** · [[Hexical]] · 📖 [[Animated Scrolls]] · Items

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="88" height="170" viewBox="0 0 87.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="43.8,62.7 30.9,85.0 18.0,107.3 30.9,129.6 43.8,152.0 56.7,129.6 69.6,107.3 56.7,85.0 43.8,62.7 18.0,62.7 30.9,85.0 43.8,107.3 56.7,85.0 69.6,62.7 43.8,62.7 30.9,40.4 18.0,18.0 43.8,18.0 69.6,18.0 56.7,40.4 43.8,62.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:515;--dur:5.6s" points="43.8,62.7 30.9,85.0 18.0,107.3 30.9,129.6 43.8,152.0 56.7,129.6 69.6,107.3 56.7,85.0 43.8,62.7 18.0,62.7 30.9,85.0 43.8,107.3 56.7,85.0 69.6,62.7 43.8,62.7 30.9,40.4 18.0,18.0 43.8,18.0 69.6,18.0 56.7,40.4 43.8,62.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="43.8" y1="62.7" x2="39.7" y2="69.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="36.7,75.0 37.2,68.4 42.1,71.2" fill="#fc77be"/><circle cx="43.8" cy="62.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwawqwqawawaewdwdw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:glow_scroll`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Causes the ink of an **Animated Scroll** to brightly glow, regardless of lighting conditions.

## Related patterns (same book page)

- [[Restore Scroll]] `entity → —`
- [[Age Scroll]] `entity → —`
- [[Vanish Scroll]] `entity → —`
- [[Dye Ink]] `entity, vector → —`

## Sources

- 📖 **Guidebook:** [[Animated Scrolls]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Illuminate Ink](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#items/animated_scroll@hexical:glow_scroll)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpAlterScroll` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:glow_scroll`
