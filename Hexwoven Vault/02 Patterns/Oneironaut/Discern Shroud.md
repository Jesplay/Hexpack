---
aliases: ["Discern Shroud"]
tags: [hexcasting/pattern, mod/oneironaut, family/other, topic/effects]
kind: "Pattern"
family: "Other"
topics: ["Status Effects"]
mod: "Oneironaut"
op_id: "oneironaut:detectshroud"
signature: "qqqqqwwaawewaawdww"
start_direction: "SOUTH_EAST"
input: "vector, number"
output: "[vector]"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Vector]]", "[[List]]"]
entry: "[[Nondetection]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Discern Shroud

◇ **Pattern** · [[Oneironaut]] · 📖 [[Nondetection]] · Patterns › Great Spells

**Topics:** [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="129" viewBox="0 0 170.0 129.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.6" cy="18.6" r="2"/><circle cx="45.2" cy="18.6" r="2"/><circle cx="71.7" cy="18.6" r="2"/><circle cx="98.3" cy="18.6" r="2"/><circle cx="124.8" cy="18.6" r="2"/><circle cx="151.4" cy="18.6" r="2"/><circle cx="5.3" cy="41.6" r="2"/><circle cx="31.9" cy="41.6" r="2"/><circle cx="58.4" cy="41.6" r="2"/><circle cx="85.0" cy="41.6" r="2"/><circle cx="111.6" cy="41.6" r="2"/><circle cx="138.1" cy="41.6" r="2"/><circle cx="164.7" cy="41.6" r="2"/><circle cx="18.6" cy="64.6" r="2"/><circle cx="45.2" cy="64.6" r="2"/><circle cx="71.7" cy="64.6" r="2"/><circle cx="98.3" cy="64.6" r="2"/><circle cx="124.8" cy="64.6" r="2"/><circle cx="151.4" cy="64.6" r="2"/><circle cx="5.3" cy="87.6" r="2"/><circle cx="31.9" cy="87.6" r="2"/><circle cx="58.4" cy="87.6" r="2"/><circle cx="85.0" cy="87.6" r="2"/><circle cx="111.6" cy="87.6" r="2"/><circle cx="138.1" cy="87.6" r="2"/><circle cx="164.7" cy="87.6" r="2"/><circle cx="18.6" cy="110.6" r="2"/><circle cx="45.2" cy="110.6" r="2"/><circle cx="71.7" cy="110.6" r="2"/><circle cx="98.3" cy="110.6" r="2"/><circle cx="124.8" cy="110.6" r="2"/><circle cx="151.4" cy="110.6" r="2"/></g><polyline class="hexpat-base" points="98.3,64.6 111.6,87.6 138.1,87.6 151.4,64.6 138.1,41.6 111.6,41.6 98.3,64.6 85.0,87.6 71.7,110.6 98.3,110.6 85.0,87.6 71.7,64.6 85.0,41.6 98.3,18.6 71.7,18.6 85.0,41.6 98.3,64.6 71.7,64.6 45.2,64.6 18.6,64.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:505;--dur:5.3s" points="98.3,64.6 111.6,87.6 138.1,87.6 151.4,64.6 138.1,41.6 111.6,41.6 98.3,64.6 85.0,87.6 71.7,110.6 98.3,110.6 85.0,87.6 71.7,64.6 85.0,41.6 98.3,18.6 71.7,18.6 85.0,41.6 98.3,64.6 71.7,64.6 45.2,64.6 18.6,64.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="98.3" y1="64.6" x2="102.5" y2="72.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.6,77.3 100.0,73.4 105.1,70.5" fill="#fc77be"/><circle cx="98.3" cy="64.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwwaawewaawdww` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `[vector]` ([[Vector]], [[List]])
- **ID:** `oneironaut:detectshroud`

```text
before   … │ vector │ number  ◂ top
after    … │ [vector]         ◂ top
```

## Description

Accepts a vector and a number, and returns a list of unit vectors pointing from the initial vector to shrouded entities within a radius defined by the number.

## Related patterns (same book page)

- [[Stealth Shroud]] `entity, number → —`

## Sources

- 📖 **Guidebook:** [[Nondetection]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Discern Shroud](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#patterns/spells/great/detection_shielding@oneironaut:detectshroud)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDetectShrouded` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:detectshroud`
