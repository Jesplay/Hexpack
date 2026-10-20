---
aliases: ["Review Broadcast"]
tags: [hexcasting/pattern, mod/slate-works, family/other, topic/circles, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Spell Circles, Loci & Engines", "Items & Crafting"]
mod: "Slate Works"
op_id: "slate_work:read_broadcast"
signature: "aqwqaweeeeewwaaw"
start_direction: "WEST"
input: "vec"
output: "any"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Locus Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Review Broadcast

◇ **Pattern** · [[Slate Works]] · 📖 [[Locus Spells]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,85.0 24.5,85.0 41.9,115.3 76.9,115.3 111.8,115.3 129.3,85.0 94.4,85.0 59.4,85.0 41.9,54.7 59.4,24.5 94.4,24.5 111.8,54.7 94.4,85.0 76.9,115.3 59.4,145.5 94.4,145.5 76.9,115.3 59.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:594;--dur:4.8s" points="59.4,85.0 24.5,85.0 41.9,115.3 76.9,115.3 111.8,115.3 129.3,85.0 94.4,85.0 59.4,85.0 41.9,54.7 59.4,24.5 94.4,24.5 111.8,54.7 94.4,85.0 76.9,115.3 59.4,145.5 94.4,145.5 76.9,115.3 59.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="85.0" x2="48.2" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="40.2,85.0 48.2,81.2 48.2,88.8" fill="#fc77be"/><circle cx="59.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqwqaweeeeewwaaw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `slate_work:read_broadcast`

```text
before   … │ vec  ◂ top
after    … │ any  ◂ top
```

## Description

Attempts to read an [[Iotic Door]] at the given vector. If there is not a [[Iotic Door|Door]], returns garbage.
Does not [[Mishaps|mishap]] if casted outside of a [[Spell Circles|Spell Circle]], free, and does not require ambit.

## Related patterns (same book page)

- [[Lay Item]] `entity → —`
- [[Get Vessels]] `— → [vecs]`
- [[Reawaken Item]] `[patterns] → —`
- [[Check Item]] `[patterns] → bool`
- [[Reorient Items]] `— → —`
- [[Bind Macro]] `vec, any iota, pattern → —`

## Sources

- 📖 **Guidebook:** [[Locus Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadBroadcast` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:read_broadcast`
