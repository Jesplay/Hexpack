---
aliases: ["Intent Numeric Input"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:intent_numeric_input"
signature: "awwaqwedwwdadq"
start_direction: "NORTH_EAST"
input: "num, text"
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

# Intent Numeric Input

◇ **Pattern** · [[HexIntent]] · 📖 [[Menu Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="128.4,84.4 145.7,54.3 111.0,54.3 76.3,54.3 41.6,54.3 59.0,84.4 93.7,84.4 128.4,84.4 145.7,114.4 111.0,114.4 76.3,114.4 41.6,114.4 59.0,84.4 24.3,84.4 41.6,54.3 24.3,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:520;--dur:4.2s" points="128.4,84.4 145.7,54.3 111.0,54.3 76.3,54.3 41.6,54.3 59.0,84.4 93.7,84.4 128.4,84.4 145.7,114.4 111.0,114.4 76.3,114.4 41.6,114.4 59.0,84.4 24.3,84.4 41.6,54.3 24.3,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="128.4" y1="84.4" x2="133.9" y2="74.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="137.9,67.9 137.2,76.7 130.6,72.9" fill="#fc77be"/><circle cx="128.4" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awwaqwedwwdadq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, text` ([[Number]], [[String]])
- **Leaves:** `intent` (—)
- **ID:** `manifestation:intent_numeric_input`

```text
before   … │ num │ text  ◂ top
after    … │ intent      ◂ top
```

## Description

Creates a numeric input from a default value and label. The entered number is pushed to the [[The Stack|stack]] of an executed action.

## Related patterns (same book page)

- [[Manifest List Menu]] `list, text → —`
- [[Manifest Grid Menu]] `list, text, num → —`
- [[Manifest Radial Menu]] `list, text → —`
- [[Intent Button]] `list, text → intent`
- [[Intent Input]] `text → intent`
- [[Intent Slider]] `num, num, num, text → intent`
- [[Intent Checkbox]] `bool, text → intent`
- [[Intent Select List]] `list, num, text, bool? → intent`
- [[Intent Section]] `text → intent`
- [[Intent Dropdown]] `list, text → intent`

## Sources

- 📖 **Guidebook:** [[Menu Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpUiNumericInput` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:intent_numeric_input`
