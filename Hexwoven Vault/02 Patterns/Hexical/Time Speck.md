---
aliases: ["Time Speck"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:lifetime_specklike"
signature: "adeqqaawdd"
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

# Time Speck

◇ **Pattern** · [[Hexical]] · 📖 [[Specks]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="111.8,24.5 94.4,54.7 129.3,54.7 111.8,85.0 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 59.4,115.3 24.5,115.3 41.9,85.0 59.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:384;--dur:3.1s" points="111.8,24.5 94.4,54.7 129.3,54.7 111.8,85.0 76.9,85.0 59.4,115.3 76.9,145.5 94.4,115.3 59.4,115.3 24.5,115.3 41.9,85.0 59.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="111.8" y1="24.5" x2="106.2" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.2,41.1 102.9,32.2 109.6,36.1" fill="#fc77be"/><circle cx="111.8" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeqqaawdd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `speck entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:lifetime_specklike`

```text
before   … │ speck entity │ number  ◂ top
after    …                          ◂ top
```

## Description

Commands the speck to disappear after that many twentieths of a second.

## Related patterns (same book page)

- [[Conjure Speck]] `any, vector, vector → entity`
- [[Move Speck]] `speck entity, vector → —`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Alter Speck]] `speck entity, any → —`
- [[Resize Speck]] `speck entity, number → —`
- [[Thicken Speck]] `speck entity, number → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Dismiss Speck]] `speck entity → —`
- [[Zone Distillation Speck]] `vector, number → list of entities`

## Sources

- 📖 **Guidebook:** [[Specks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Time Speck](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks@hexical:lifetime_specklike)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpecklikeProperty` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:lifetime_specklike`
