---
aliases: ["Phianglement Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/devices]
kind: "Pattern"
family: "Purification"
topics: ["Casting Devices"]
mod: "Lapisworks"
op_id: "lapisworks:get_linkable_links"
signature: "qaqdaweqaqewaqwawaw"
start_direction: "NORTH_EAST"
input: "vec"
output: "[vec]"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]", "[[List]]"]
entry: "[[Media Condensing Unit]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Phianglement Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Media Condensing Unit]] · Lapisworks › Imbuement

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="88" height="170" viewBox="0 0 87.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="43.8,62.7 56.7,40.4 43.8,18.0 30.9,40.4 43.8,62.7 18.0,62.7 30.9,85.0 43.8,107.3 30.9,129.6 43.8,152.0 56.7,129.6 43.8,107.3 56.7,85.0 69.6,62.7 43.8,62.7 30.9,85.0 18.0,107.3 43.8,107.3 69.6,107.3 56.7,85.0 43.8,62.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:515;--dur:5.6s" points="43.8,62.7 56.7,40.4 43.8,18.0 30.9,40.4 43.8,62.7 18.0,62.7 30.9,85.0 43.8,107.3 30.9,129.6 43.8,152.0 56.7,129.6 43.8,107.3 56.7,85.0 69.6,62.7 43.8,62.7 30.9,85.0 18.0,107.3 43.8,107.3 69.6,107.3 56.7,85.0 43.8,62.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="43.8" y1="62.7" x2="47.9" y2="55.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.9,50.4 50.4,57.0 45.5,54.1" fill="#fc77be"/><circle cx="43.8" cy="62.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqdaweqaqewaqwawaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `[vec]` ([[Vector]], [[List]])
- **ID:** `lapisworks:get_linkable_links`

```text
before   … │ vec    ◂ top
after    … │ [vec]  ◂ top
```

## Description

Returns the positions the **phiangleable** at the specified position (within ambit) is phiangled to. Free.

## Related patterns (same book page)

- [[Phiangle]] `vec, vec → —`
- [[Dephiangle]] `vec, vec → —`

## Sources

- 📖 **Guidebook:** [[Media Condensing Unit]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetLinkableLinks` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_linkable_links`
