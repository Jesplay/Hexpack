---
aliases: ["Get Vessels"]
tags: [hexcasting/pattern, mod/slate-works, family/other, topic/circles, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Spell Circles, Loci & Engines", "Items & Crafting"]
mod: "Slate Works"
op_id: "slate_work:get_storage"
signature: "eaqwqaeqqdeewweedq"
start_direction: "SOUTH_WEST"
input: ""
output: "[vecs]"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]", "[[List]]"]
entry: "[[Locus Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Get Vessels

◇ **Pattern** · [[Slate Works]] · 📖 [[Locus Spells]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/><circle cx="22.0" cy="103.8" r="2"/><circle cx="53.5" cy="103.8" r="2"/><circle cx="85.0" cy="103.8" r="2"/><circle cx="116.5" cy="103.8" r="2"/><circle cx="148.0" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 69.3,76.6 37.8,76.6 53.5,103.8 85.0,103.8 116.5,103.8 132.2,76.6 100.7,76.6 85.0,49.3 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 69.3,22.0 100.7,22.0 132.2,22.0 148.0,49.3 132.2,76.6 116.5,49.3 85.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:598;--dur:5.3s" points="85.0,49.3 69.3,76.6 37.8,76.6 53.5,103.8 85.0,103.8 116.5,103.8 132.2,76.6 100.7,76.6 85.0,49.3 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 69.3,22.0 100.7,22.0 132.2,22.0 148.0,49.3 132.2,76.6 116.5,49.3 85.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="80.0" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,64.3 77.0,56.3 83.0,59.8" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaeqqdeewweedq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[vecs]` ([[Vector]], [[List]])
- **ID:** `slate_work:get_storage`

```text
before   …           ◂ top
after    … │ [vecs]  ◂ top
```

## Description

This simply returns what **Vessels** the current [[Spell Circles|Spell Circle]] has activated. I should likely use this to check if the [[Spell Circles|Spell Circle]] has collected any **Vessels**.

## Related patterns (same book page)

- [[Lay Item]] `entity → —`
- [[Reawaken Item]] `[patterns] → —`
- [[Check Item]] `[patterns] → bool`
- [[Reorient Items]] `— → —`
- [[Bind Macro]] `vec, any iota, pattern → —`
- [[Review Broadcast]] `vec → any`

## Sources

- 📖 **Guidebook:** [[Locus Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetStorageLoci` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:get_storage`
