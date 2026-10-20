---
aliases: ["Bury Item"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "YAHA"
op_id: "yaha:sussify_block"
signature: "eqqqeawqwqwqwqwqw"
start_direction: "EAST"
input: "vector, entity"
output: ""
cost: "Costs an eighth of an Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Yaha Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bury Item

✦ **Spell** · [[YAHA]] · 📖 [[Yaha Spells]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="22.0,76.6 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="22.0,76.6 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="76.6" x2="32.1" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.4,76.6 32.1,80.0 32.1,73.1" fill="#fc77be"/><circle cx="22.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqeawqwqwqwqwqw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, entity` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs an eighth of an Amethyst Dust.
- **ID:** `yaha:sussify_block`

```text
before   … │ vector │ entity  ◂ top
after    …                    ◂ top
```

## Description

Buries the item entity into a sand or gravel block, forming suspicious sand/gravel. Costs an eighth of an [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Pluck]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Yaha Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSussifyBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `yaha:sussify_block`
