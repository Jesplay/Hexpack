---
aliases: ["Resize Speck"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:size_specklike"
signature: "adeeqed"
start_direction: "SOUTH_WEST"
input: "speck entity, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Specks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Resize Speck

◇ **Pattern** · [[Hexical]] · 📖 [[Specks]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="128.4,24.3 111.0,54.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 41.6,54.3 24.3,24.3 59.0,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:2.2s" points="128.4,24.3 111.0,54.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 41.6,54.3 24.3,24.3 59.0,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="128.4" y1="24.3" x2="122.8" y2="33.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="118.8,40.8 119.5,32.0 126.1,35.8" fill="#fc77be"/><circle cx="128.4" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeeqed` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `speck entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:size_specklike`

```text
before   … │ speck entity │ number  ◂ top
after    …                          ◂ top
```

## Description

Scales the speck, can range from 0 to 10. Represents size in blocks and is zero by default.

## Related patterns (same book page)

- [[Conjure Speck]] `any, vector, vector → entity`
- [[Move Speck]] `speck entity, vector → —`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Alter Speck]] `speck entity, any → —`
- [[Time Speck]] `speck entity, number → —`
- [[Thicken Speck]] `speck entity, number → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Dismiss Speck]] `speck entity → —`
- [[Zone Distillation Speck]] `vector, number → list of entities`

## Sources

- 📖 **Guidebook:** [[Specks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Resize Speck](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks@hexical:size_specklike)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpecklikeProperty` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:size_specklike`
