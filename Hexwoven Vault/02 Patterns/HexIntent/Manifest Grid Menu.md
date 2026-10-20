---
aliases: ["Manifest Grid Menu"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:create_grid_menu"
signature: "awwaeawwaqwddad"
start_direction: "NORTH_EAST"
input: "list, text, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[List]]", "[[String]]"]
iotas_out: []
entry: "[[Menu Manifestation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifest Grid Menu

◇ **Pattern** · [[HexIntent]] · 📖 [[Menu Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 85.0,60.5 46.4,60.5 65.7,27.0 85.0,60.5 104.3,27.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:618;--dur:4.5s" points="123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 85.0,60.5 46.4,60.5 65.7,27.0 85.0,60.5 104.3,27.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="60.5" x2="129.8" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="134.3,42.1 133.5,51.9 126.1,47.7" fill="#fc77be"/><circle cx="123.6" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awwaeawwaqwddad` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, text, num` ([[Number]], [[List]], [[String]])
- **Leaves:** `nothing` (—)
- **ID:** `manifestation:create_grid_menu`

```text
before   … │ list │ text │ num  ◂ top
after    …                      ◂ top
```

## Description

Takes a list of intents, a menu title, and a number of columns to manifest a grid menu. Columns are limited to 1-10.

## Related patterns (same book page)

- [[Manifest List Menu]] `list, text → —`
- [[Manifest Radial Menu]] `list, text → —`
- [[Intent Button]] `list, text → intent`
- [[Intent Input]] `text → intent`
- [[Intent Numeric Input]] `num, text → intent`
- [[Intent Slider]] `num, num, num, text → intent`
- [[Intent Checkbox]] `bool, text → intent`
- [[Intent Select List]] `list, num, text, bool? → intent`
- [[Intent Section]] `text → intent`
- [[Intent Dropdown]] `list, text → intent`

## Sources

- 📖 **Guidebook:** [[Menu Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCreateGridMenu` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:create_grid_menu`
