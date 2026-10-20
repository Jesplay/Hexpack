---
aliases: ["Hopper"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:hopper"
signature: "qwawqwaeqqq"
start_direction: "SOUTH_EAST"
input: "source, num?, dest., num?"
output: ""
cost: "Cost scales based on the amount of mass transferred - a stack of 64 items will cost about three Amethyst Dust."
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Hopper (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hopper

◇ **Pattern** · [[Hexical]] · 📖 [[Hopper (Book Entry)]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="46.4,60.5 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="52.5" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.0,78.9 48.9,73.3 56.2,69.1" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwawqwaeqqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `source, num?, dest., num?` ([[Number]])
- **Leaves:** `nothing` (—)
- **Cost:** Cost scales based on the amount of mass transferred - a stack of 64 items will cost about three Amethyst Dust.
- **ID:** `hexical:hopper`

```text
before   … │ source │ num? │ dest. │ num?  ◂ top
after    …                                 ◂ top
```

## Description

Instantly transports all items from **source** to **destination**. Cost scales based on the amount of mass transferred - a [[The Stack|stack]] of 64 items will cost about three [[Amethyst|Amethyst Dust]]. Numbers are optional; details later.

Entity references to the caster, an **Armor Stand**, a **Minecart with a Chest**, a **Hopper Minecart**, or a **Boat with a Chest** within ambit are valid **sources** that refer to the respective inventory. They also serve as valid **destinations**.

Entity references to **Item Frames** and dropped **Items** are also valid **sources** and **destinations**. For **Items**, pushes will attempt to stack onto it.

Vectors are also possible **sources** and **destinations**. If a block with an inventory does not exist at a location, it is a valid **destination** to push items out into the world.

If a block with an inventory does exist, it serves as both a **source** and a **destination**. If the vector is closest to the center, it pushes and pulls from any inventory slot; if it is instead closer to a face, it will only push to and pull from that face, similar to **Hoppers**.

If I pass in Null, it seems to refer to my **Ender Chest**'s inventory. Naturally, this means it will only work for humans casting this spell.

Because **sources** and **destinations** can never be numbers, it is distinguishable to pass an optional number after a **source** or **destination**. This is the num? in the pattern signature and correlates to *slot*.

Slots are a slightly esoteric way that Nature has chosen to represent specific spaces in the inventory. For example, 0, 1, 2, and 3 corresponds to boots, leggings, chestplate, and helmet spaces of an **Armor Stand source** respectively. Passing in slots allows me to dictate to [[Hopper]] which space to take items from and move items to.

For quick reference, my hotbar is 0-8, my offhand is 40, and my [[Wristpocket (Book Entry)|Wristpocket]] is -1.

## Related patterns (same book page)

- [[Hopper Purification]] `source → list`

## Sources

- 📖 **Guidebook:** [[Hopper (Book Entry)]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpHopper` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:hopper`
