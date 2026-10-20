---
aliases: ["Allocate Item"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/wisps]
kind: "Spell"
family: "Spell"
topics: ["Wisps & Links"]
mod: "Lapisworks"
op_id: "lapisworks:gib_wisp_item"
signature: "aqawewewaqaweqqaqqedeae"
start_direction: "NORTH_WEST"
input: "wisp, item entity"
output: ""
cost: "Costs two and a half dust."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Wisp Handling]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Allocate Item

✦ **Spell** · [[Lapisworks]] · 📖 [[Wisp Handling]] · Lapisworks › Interop › Lapse

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="129" viewBox="0 0 170.0 129.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/><circle cx="18.6" cy="87.6" r="2"/><circle cx="45.2" cy="87.6" r="2"/><circle cx="71.7" cy="87.6" r="2"/><circle cx="98.3" cy="87.6" r="2"/><circle cx="124.8" cy="87.6" r="2"/><circle cx="151.4" cy="87.6" r="2"/><circle cx="5.3" cy="110.6" r="2"/><circle cx="31.9" cy="110.6" r="2"/><circle cx="58.4" cy="110.6" r="2"/><circle cx="85.0" cy="110.6" r="2"/><circle cx="111.6" cy="110.6" r="2"/><circle cx="138.1" cy="110.6" r="2"/><circle cx="164.7" cy="110.6" r="2"/></g><polyline class="hexpat-base" points="45.2,41.6 31.9,18.6 18.6,41.6 31.9,64.6 45.2,41.6 58.4,18.6 85.0,18.6 111.6,18.6 124.8,41.6 138.1,64.6 151.4,41.6 138.1,18.6 124.8,41.6 111.6,64.6 85.0,64.6 71.7,87.6 85.0,110.6 98.3,87.6 85.0,64.6 58.4,64.6 45.2,41.6 71.7,41.6 85.0,64.6 98.3,41.6 124.8,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:638;--dur:6.0s" points="45.2,41.6 31.9,18.6 18.6,41.6 31.9,64.6 45.2,41.6 58.4,18.6 85.0,18.6 111.6,18.6 124.8,41.6 138.1,64.6 151.4,41.6 138.1,18.6 124.8,41.6 111.6,64.6 85.0,64.6 71.7,87.6 85.0,110.6 98.3,87.6 85.0,64.6 58.4,64.6 45.2,41.6 71.7,41.6 85.0,64.6 98.3,41.6 124.8,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.2" y1="41.6" x2="40.9" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.9,28.9 43.4,32.8 38.4,35.7" fill="#fc77be"/><circle cx="45.2" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawewewaqaweqqaqqedeae` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `wisp, item entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs two and a half dust.
- **ID:** `lapisworks:gib_wisp_item`

```text
before   … │ wisp │ item entity  ◂ top
after    …                       ◂ top
```

## Description

Swaps the item the provided wisp (that I must own) is holding and the provided item. Costs two and a half dust.

## Related patterns (same book page)

- [[Command Drop]] `wisp → —`

## Sources

- 📖 **Guidebook:** [[Wisp Handling]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `GibWispItem` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:gib_wisp_item`
