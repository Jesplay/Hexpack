---
aliases: ["Intent Checkbox"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:intent_checkbox"
signature: "awwaqwedwwdaeedd"
start_direction: "NORTH_EAST"
input: "bool, text"
output: "intent"
cost: ""
iotas_in: ["[[Boolean]]", "[[String]]"]
iotas_out: []
entry: "[[Menu Manifestation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Intent Checkbox

◇ **Pattern** · [[HexIntent]] · 📖 [[Menu Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/><circle cx="22.0" cy="103.8" r="2"/><circle cx="53.5" cy="103.8" r="2"/><circle cx="85.0" cy="103.8" r="2"/><circle cx="116.5" cy="103.8" r="2"/><circle cx="148.0" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="132.2,76.6 148.0,49.3 116.5,49.3 85.0,49.3 53.5,49.3 69.3,76.6 100.7,76.6 132.2,76.6 148.0,103.8 116.5,103.8 85.0,103.8 53.5,103.8 69.3,76.6 37.8,76.6 22.0,49.3 37.8,22.0 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:535;--dur:4.8s" points="132.2,76.6 148.0,49.3 116.5,49.3 85.0,49.3 53.5,49.3 69.3,76.6 100.7,76.6 132.2,76.6 148.0,103.8 116.5,103.8 85.0,103.8 53.5,103.8 69.3,76.6 37.8,76.6 22.0,49.3 37.8,22.0 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="132.2" y1="76.6" x2="137.3" y2="67.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="140.9,61.6 140.3,69.6 134.3,66.1" fill="#fc77be"/><circle cx="132.2" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awwaqwedwwdaeedd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `bool, text` ([[Boolean]], [[String]])
- **Leaves:** `intent` (—)
- **ID:** `manifestation:intent_checkbox`

```text
before   … │ bool │ text  ◂ top
after    … │ intent       ◂ top
```

## Description

Creates a checkbox from a default checked state and label. The resulting boolean is pushed to the [[The Stack|stack]] of an executed action.

## Related patterns (same book page)

- [[Manifest List Menu]] `list, text → —`
- [[Manifest Grid Menu]] `list, text, num → —`
- [[Manifest Radial Menu]] `list, text → —`
- [[Intent Button]] `list, text → intent`
- [[Intent Input]] `text → intent`
- [[Intent Numeric Input]] `num, text → intent`
- [[Intent Slider]] `num, num, num, text → intent`
- [[Intent Select List]] `list, num, text, bool? → intent`
- [[Intent Section]] `text → intent`
- [[Intent Dropdown]] `list, text → intent`

## Sources

- 📖 **Guidebook:** [[Menu Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpUiCheckbox` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:intent_checkbox`
