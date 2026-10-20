---
aliases: ["Dye Ink"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI", "Items & Crafting"]
mod: "Hexical"
op_id: "hexical:color_scroll"
signature: "wqwawqwqawawaedd"
start_direction: "SOUTH_WEST"
input: "entity, vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Animated Scrolls]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dye Ink

✦ **Spell** · [[Hexical]] · 📖 [[Animated Scrolls]] · Items

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="101" height="170" viewBox="0 0 100.9 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="50.4,46.5 35.6,72.2 20.8,97.8 35.6,123.5 50.4,149.2 65.3,123.5 80.1,97.8 65.3,72.2 50.4,46.5 20.8,46.5 35.6,72.2 50.4,97.8 65.3,72.2 80.1,46.5 50.4,46.5 35.6,20.8 65.3,20.8 50.4,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.8s" points="50.4,46.5 35.6,72.2 20.8,97.8 35.6,123.5 50.4,149.2 65.3,123.5 80.1,97.8 65.3,72.2 50.4,46.5 20.8,46.5 35.6,72.2 50.4,97.8 65.3,72.2 80.1,46.5 50.4,46.5 35.6,20.8 65.3,20.8 50.4,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.4" y1="46.5" x2="45.7" y2="54.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.3,60.6 42.9,53.1 48.5,56.3" fill="#fc77be"/><circle cx="50.4" cy="46.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwawqwqawawaedd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:color_scroll`

```text
before   … │ entity │ vector  ◂ top
after    …                    ◂ top
```

## Description

Changes the color of an **Animated Scroll's** ink to the color specified by the vector, with the components being 0 to 1 values of red, green, and blue respectively.

## Related patterns (same book page)

- [[Restore Scroll]] `entity → —`
- [[Age Scroll]] `entity → —`
- [[Vanish Scroll]] `entity → —`
- [[Illuminate Ink]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Animated Scrolls]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Dye Ink](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#items/animated_scroll@hexical:color_scroll)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpColorScroll` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:color_scroll`
