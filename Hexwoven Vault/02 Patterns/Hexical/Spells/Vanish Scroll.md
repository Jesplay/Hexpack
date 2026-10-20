---
aliases: ["Vanish Scroll"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI", "Items & Crafting"]
mod: "Hexical"
op_id: "hexical:vanish_scroll"
signature: "wqwawqwqaqqa"
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

# Vanish Scroll

✦ **Spell** · [[Hexical]] · 📖 [[Animated Scrolls]] · Items

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 59.4,24.5 24.5,24.5 41.9,54.7 76.9,54.7 94.4,24.5 59.4,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:454;--dur:3.6s" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 59.4,24.5 24.5,24.5 41.9,54.7 76.9,54.7 94.4,24.5 59.4,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="24.5" x2="53.8" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,41.1 50.5,32.2 57.2,36.1" fill="#fc77be"/><circle cx="59.4" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwawqwqaqqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:vanish_scroll`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Causes the **Animated Scroll** to become invisible leaving only the pattern, making it appear as though the pattern were magically etched into the surface the scroll is on.

## Related patterns (same book page)

- [[Restore Scroll]] `entity → —`
- [[Age Scroll]] `entity → —`
- [[Dye Ink]] `entity, vector → —`
- [[Illuminate Ink]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Animated Scrolls]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Vanish Scroll](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#items/animated_scroll@hexical:vanish_scroll)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpAlterScroll` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:vanish_scroll`
