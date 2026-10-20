---
aliases: ["Scale Icon"]
tags: [hexcasting/pattern, mod/scryglass, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Scryglass"
op_id: "scryglass:scale_icon"
signature: "aaqwdwwwdwwwdweede"
start_direction: "NORTH_EAST"
input: "index, number"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Viewforming]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Scale Icon

◇ **Pattern** · [[Scryglass]] · 📖 [[Viewforming]] · Patterns

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="154.3" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="154.3" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="154.3" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="80.1,46.5 94.9,20.8 65.3,20.8 80.1,46.5 109.8,46.5 139.4,46.5 124.6,72.2 109.8,97.8 94.9,123.5 80.1,149.2 65.3,123.5 50.4,97.8 35.6,72.2 20.8,46.5 50.4,46.5 80.1,46.5 94.9,72.2 80.1,97.8 65.3,72.2 80.1,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:564;--dur:5.3s" points="80.1,46.5 94.9,20.8 65.3,20.8 80.1,46.5 109.8,46.5 139.4,46.5 124.6,72.2 109.8,97.8 94.9,123.5 80.1,149.2 65.3,123.5 50.4,97.8 35.6,72.2 20.8,46.5 50.4,46.5 80.1,46.5 94.9,72.2 80.1,97.8 65.3,72.2 80.1,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="80.1" y1="46.5" x2="84.8" y2="38.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="88.3,32.3 87.7,39.9 82.0,36.6" fill="#fc77be"/><circle cx="80.1" cy="46.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqwdwwwdwwwdweede` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `index, number` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `scryglass:scale_icon`

```text
before   … │ index │ number  ◂ top
after    …                   ◂ top
```

## Description

Takes an index and a scaling factor. Scales the icon.

## Related patterns (same book page)

- [[Periphery Reflection]] `— → vector`
- [[Icons Reflection]] `— → list of indices`
- [[Clear Icon]] `index → —`
- [[Draw Text]] `index, vector, number, any → —`
- [[Rotate Icon]] `index, number → —`
- [[Draw Rectangle]] `index, vector, vector, vector → —`
- [[Draw Line]] `index, vector, vector, vector → —`

## Sources

- 📖 **Guidebook:** [[Viewforming]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpScaleIcon` is not a `SpellAction`
- 🔧 **Pattern ID:** `scryglass:scale_icon`
