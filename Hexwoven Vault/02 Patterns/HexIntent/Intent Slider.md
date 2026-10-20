---
aliases: ["Intent Slider"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:intent_slider"
signature: "awwaqwedwwdaw"
start_direction: "NORTH_EAST"
input: "num, num, num, text"
output: "intent"
cost: ""
iotas_in: ["[[Number]]", "[[String]]"]
iotas_out: []
entry: "[[Menu Manifestation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Intent Slider

◇ **Pattern** · [[HexIntent]] · 📖 [[Menu Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="90" viewBox="0 0 170.0 90.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.8" cy="20.2" r="2"/><circle cx="34.6" cy="20.2" r="2"/><circle cx="63.4" cy="20.2" r="2"/><circle cx="92.2" cy="20.2" r="2"/><circle cx="121.0" cy="20.2" r="2"/><circle cx="149.8" cy="20.2" r="2"/><circle cx="20.2" cy="45.1" r="2"/><circle cx="49.0" cy="45.1" r="2"/><circle cx="77.8" cy="45.1" r="2"/><circle cx="106.6" cy="45.1" r="2"/><circle cx="135.4" cy="45.1" r="2"/><circle cx="164.2" cy="45.1" r="2"/><circle cx="5.8" cy="70.1" r="2"/><circle cx="34.6" cy="70.1" r="2"/><circle cx="63.4" cy="70.1" r="2"/><circle cx="92.2" cy="70.1" r="2"/><circle cx="121.0" cy="70.1" r="2"/><circle cx="149.8" cy="70.1" r="2"/></g><polyline class="hexpat-base" points="135.4,45.1 149.8,20.2 121.0,20.2 92.2,20.2 63.4,20.2 77.8,45.1 106.6,45.1 135.4,45.1 149.8,70.1 121.0,70.1 92.2,70.1 63.4,70.1 77.8,45.1 49.0,45.1 20.2,45.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:403;--dur:3.9s" points="135.4,45.1 149.8,20.2 121.0,20.2 92.2,20.2 63.4,20.2 77.8,45.1 106.6,45.1 135.4,45.1 149.8,70.1 121.0,70.1 92.2,70.1 63.4,70.1 77.8,45.1 49.0,45.1 20.2,45.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.4" y1="45.1" x2="140.0" y2="37.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="143.3,31.4 142.8,38.7 137.3,35.6" fill="#fc77be"/><circle cx="135.4" cy="45.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awwaqwedwwdaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num, num, text` ([[Number]], [[String]])
- **Leaves:** `intent` (—)
- **ID:** `manifestation:intent_slider`

```text
before   … │ num │ num │ num │ text  ◂ top
after    … │ intent                  ◂ top
```

## Description

Creates a slider from min, max, current values, and a label. The chosen value is pushed to the [[The Stack|stack]] of an executed action.

## Related patterns (same book page)

- [[Manifest List Menu]] `list, text → —`
- [[Manifest Grid Menu]] `list, text, num → —`
- [[Manifest Radial Menu]] `list, text → —`
- [[Intent Button]] `list, text → intent`
- [[Intent Input]] `text → intent`
- [[Intent Numeric Input]] `num, text → intent`
- [[Intent Checkbox]] `bool, text → intent`
- [[Intent Select List]] `list, num, text, bool? → intent`
- [[Intent Section]] `text → intent`
- [[Intent Dropdown]] `list, text → intent`

## Sources

- 📖 **Guidebook:** [[Menu Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpUiSlider` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:intent_slider`
