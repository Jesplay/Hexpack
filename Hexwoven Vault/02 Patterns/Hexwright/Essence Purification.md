---
aliases: ["Essence Purification"]
tags: [hexcasting/pattern, mod/hexwright, family/purification, topic/hexwright, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Hex Wright Magic", "Items & Crafting"]
mod: "Hexwright"
op_id: "hexwright:essence_purification"
signature: "ewedadadew"
start_direction: "WEST"
input: "vec"
output: "[string | null]"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[List]]", "[[Null]]", "[[String]]"]
entry: "[[Endless Pouch]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Essence Purification

◇ **Pattern** · [[Hexwright]] · 📖 [[Endless Pouch]] · Hex Wright › Fire & Essence

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="104.3,127.4 65.7,127.4 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="104.3,127.4 65.7,127.4 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="127.4" x2="92.0" y2="127.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="83.1,127.4 92.0,123.2 92.0,131.7" fill="#fc77be"/><circle cx="104.3" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewedadadew` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `[string | null]` ([[List]], [[Null]], [[String]])
- **ID:** `hexwright:essence_purification`

## Description

Returns a list of the Essences in a pouch at a location. Returns null if that Essence isn't present.

## Related patterns (same book page)

- [[Aspect Distillation]] `vec, num → num`

## Sources

- 📖 **Guidebook:** [[Endless Pouch]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `HexwrightActions$1` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:essence_purification`
