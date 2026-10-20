---
aliases: ["Lay Item"]
tags: [hexcasting/spell, mod/slate-works, family/spell, topic/circles, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Spell Circles, Loci & Engines", "Items & Crafting"]
mod: "Slate Works"
op_id: "slate_work:store_item"
signature: "eaqwqaeqwaeaeqqeaeaw"
start_direction: "SOUTH_WEST"
input: "entity"
output: ""
cost: "Costs an 1/8th of a dust per activated Vessel."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Locus Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Lay Item

✦ **Spell** · [[Slate Works]] · 📖 [[Locus Spells]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="85.0,22.0 69.3,49.3 37.8,49.3 53.5,76.6 85.0,76.6 116.5,76.6 132.2,49.3 100.7,49.3 85.0,22.0 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 148.0,76.6 132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:661;--dur:5.9s" points="85.0,22.0 69.3,49.3 37.8,49.3 53.5,76.6 85.0,76.6 116.5,76.6 132.2,49.3 100.7,49.3 85.0,22.0 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 148.0,76.6 132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="22.0" x2="80.0" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,37.0 77.0,29.0 83.0,32.5" fill="#fc77be"/><circle cx="85.0" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaeqwaeaeqqeaeaw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs an 1/8th of a dust per activated Vessel.
- **ID:** `slate_work:store_item`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

It is much easier to store an item into a **Vessel** than to pull it out. This pattern simply takes an Item Entity, and neatly sorts it into the activated **Storage Vessels**. Costs an 1/8th of a [[Amethyst|dust]] per activated **Vessel**.

## Related patterns (same book page)

- [[Get Vessels]] `— → [vecs]`
- [[Reawaken Item]] `[patterns] → —`
- [[Check Item]] `[patterns] → bool`
- [[Reorient Items]] `— → —`
- [[Bind Macro]] `vec, any iota, pattern → —`
- [[Review Broadcast]] `vec → any`

## Sources

- 📖 **Guidebook:** [[Locus Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpStoreItem` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `slate_work:store_item`
