---
aliases: ["Intent Select List"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:intent_select_list"
signature: "awwaqwedwwdaaedd"
start_direction: "NORTH_EAST"
input: "list, num, text, bool?"
output: "intent"
cost: ""
iotas_in: ["[[Number]]", "[[Boolean]]", "[[List]]", "[[String]]"]
iotas_out: []
entry: "[[Menu Manifestation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Intent Select List

◇ **Pattern** · [[HexIntent]] · 📖 [[Menu Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0 53.5,22.0 69.3,49.3 100.7,49.3 132.2,49.3 148.0,76.6 116.5,76.6 85.0,76.6 53.5,76.6 69.3,49.3 37.8,49.3 53.5,76.6 37.8,103.8 22.0,76.6 53.5,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:535;--dur:4.8s" points="132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0 53.5,22.0 69.3,49.3 100.7,49.3 132.2,49.3 148.0,76.6 116.5,76.6 85.0,76.6 53.5,76.6 69.3,49.3 37.8,49.3 53.5,76.6 37.8,103.8 22.0,76.6 53.5,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="132.2" y1="49.3" x2="137.3" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="140.9,34.3 140.3,42.3 134.3,38.8" fill="#fc77be"/><circle cx="132.2" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awwaqwedwwdaaedd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, num, text, bool?` ([[Number]], [[Boolean]], [[List]], [[String]])
- **Leaves:** `intent` (—)
- **ID:** `manifestation:intent_select_list`

```text
before   … │ list │ num │ text │ bool?  ◂ top
after    … │ intent                     ◂ top
```

## Description

Creates a selectable list from options, a max visible row count, and a label. Optionally accepts a boolean to enable multi-select. The selected option or options are pushed to the [[The Stack|stack]] of an executed action.

## Related patterns (same book page)

- [[Manifest List Menu]] `list, text → —`
- [[Manifest Grid Menu]] `list, text, num → —`
- [[Manifest Radial Menu]] `list, text → —`
- [[Intent Button]] `list, text → intent`
- [[Intent Input]] `text → intent`
- [[Intent Numeric Input]] `num, text → intent`
- [[Intent Slider]] `num, num, num, text → intent`
- [[Intent Checkbox]] `bool, text → intent`
- [[Intent Section]] `text → intent`
- [[Intent Dropdown]] `list, text → intent`

## Sources

- 📖 **Guidebook:** [[Menu Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpUiSelectList` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:intent_select_list`
