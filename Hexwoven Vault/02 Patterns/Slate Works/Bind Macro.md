---
aliases: ["Bind Macro"]
tags: [hexcasting/pattern, mod/slate-works, family/other, topic/circles, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Spell Circles, Loci & Engines", "Items & Crafting"]
mod: "Slate Works"
op_id: "slate_work:set_macro"
signature: "qqqwqqqqqaqeeaqwqae"
start_direction: "WEST"
input: "vec, any iota, pattern"
output: ""
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]", "[[Pattern]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Locus Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bind Macro

◇ **Pattern** · [[Slate Works]] · 📖 [[Locus Spells]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,85.0 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 111.8,85.0 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 76.9,85.0 94.4,54.7 129.3,54.7 111.8,24.5 76.9,24.5 41.9,24.5 24.5,54.7 59.4,54.7 76.9,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:699;--dur:5.6s" points="76.9,85.0 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 111.8,85.0 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 76.9,85.0 94.4,54.7 129.3,54.7 111.8,24.5 76.9,24.5 41.9,24.5 24.5,54.7 59.4,54.7 76.9,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="85.0" x2="65.7" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.7,85.0 65.7,81.2 65.7,88.8" fill="#fc77be"/><circle cx="76.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqwqqqqqaqeeaqwqae` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, any iota, pattern` ([[Vector]], [[Pattern]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a negligible amount of media.
- **ID:** `slate_work:set_macro`

```text
before   … │ vec │ any iota │ pattern  ◂ top
after    …                             ◂ top
```

## Description

This binds both a pattern and an [[Iotas|iota]] to a [[Spell Imprinter]], which is at the targeted vector.

Does not [[Mishaps|mishap]] if casted outside of a [[Spell Circles|Spell Circle]]. Costs a negligible amount of **[[Media|media]]**.

## Related patterns (same book page)

- [[Lay Item]] `entity → —`
- [[Get Vessels]] `— → [vecs]`
- [[Reawaken Item]] `[patterns] → —`
- [[Check Item]] `[patterns] → bool`
- [[Reorient Items]] `— → —`
- [[Review Broadcast]] `vec → any`

## Sources

- 📖 **Guidebook:** [[Locus Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetMacro` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:set_macro`
