---
aliases: ["Reach Purification"]
tags: [hexcasting/pattern, mod/hexwright, family/purification, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Purification"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_reach"
signature: "wqwqwqwqwqwaeqqqqqaww"
start_direction: "SOUTH_WEST"
input: "region"
output: "boolean"
cost: ""
iotas_in: ["[[Region]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Executing Regions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Reach Purification

◇ **Pattern** · [[Hexwright]] · 📖 [[Executing Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 69.3,49.3 85.0,76.6 100.7,103.8 116.5,131.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 69.3,49.3 85.0,76.6 100.7,103.8 116.5,131.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="22.0" x2="48.5" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,37.0 45.5,29.0 51.5,32.5" fill="#fc77be"/><circle cx="53.5" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqwqwaeqqqqqaww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region` ([[Region]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexwright:region_reach`

```text
before   … │ region   ◂ top
after    … │ boolean  ◂ top
```

## Description

Returns true if the whole region lies within your ambit, false if any part of it is out of reach.

## Related patterns (same book page)

- [[Entities Within]] `region → [entities]`
- [[Region Thoth]] `region, [pattern] → list`
- [[Region Fill]] `region → —`
- [[Region Break]] `region → —`

## Sources

- 📖 **Guidebook:** [[Executing Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RegionActions$9` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:region_reach`
