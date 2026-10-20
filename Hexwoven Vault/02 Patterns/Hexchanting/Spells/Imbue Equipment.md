---
aliases: ["Imbue Equipment"]
tags: [hexcasting/spell, mod/hexchanting, family/spell, topic/enchanting, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Enchanting & Imbuement", "Items & Crafting"]
mod: "Hexchanting"
op_id: "hexchanting:imbue_equipment"
signature: "dqaqdqaqdqaeadawadadawadadawa"
start_direction: "WEST"
input: "[pattern]"
output: ""
cost: "Costs 9 Charged Amethyst except on arrows, which cost 5 Shards."
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Imbuing Equipment]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Imbue Equipment

✦ **Spell** · [[Hexchanting]] · 📖 [[Imbuing Equipment]] · Items

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="116.5,76.6 85.0,76.6 100.7,49.3 85.0,22.0 69.3,49.3 85.0,76.6 53.5,76.6 37.8,103.8 69.3,103.8 85.0,76.6 100.7,103.8 132.2,103.8 116.5,76.6 132.2,49.3 100.7,49.3 116.5,22.0 85.0,22.0 53.5,22.0 69.3,49.3 37.8,49.3 53.5,76.6 22.0,76.6 37.8,103.8 53.5,131.1 69.3,103.8 85.0,131.1 100.7,103.8 116.5,131.1 132.2,103.8 148.0,76.6 116.5,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:944;--dur:6.0s" points="116.5,76.6 85.0,76.6 100.7,49.3 85.0,22.0 69.3,49.3 85.0,76.6 53.5,76.6 37.8,103.8 69.3,103.8 85.0,76.6 100.7,103.8 132.2,103.8 116.5,76.6 132.2,49.3 100.7,49.3 116.5,22.0 85.0,22.0 53.5,22.0 69.3,49.3 37.8,49.3 53.5,76.6 22.0,76.6 37.8,103.8 53.5,131.1 69.3,103.8 85.0,131.1 100.7,103.8 116.5,131.1 132.2,103.8 148.0,76.6 116.5,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="116.5" y1="76.6" x2="106.4" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="99.2,76.6 106.4,73.1 106.4,80.0" fill="#fc77be"/><circle cx="116.5" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqaqdqaqdqaeadawadadawadadawa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern]` ([[List]], [[Pattern]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 9 Charged Amethyst except on arrows, which cost 5 Shards.
- **ID:** `hexchanting:imbue_equipment`

```text
before   … │ [pattern]  ◂ top
after    …              ◂ top
```

## Description

Imbues the supplied hexes into the [[Amethyst]] equipment in my other hand, so that they will be cast whenever the item is used. Costs 9 [[Amethyst|Charged Amethyst]] except on arrows, which cost 5 [[Amethyst|Shards]].

## Sources

- 📖 **Guidebook:** [[Imbuing Equipment]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexchanting on Modrinth](https://modrinth.com/mod/UlOmzEJF)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpImbueEquipment` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexchanting:imbue_equipment`
