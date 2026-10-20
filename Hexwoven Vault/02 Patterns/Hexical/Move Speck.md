---
aliases: ["Move Speck"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:move_specklike"
signature: "adeqaa"
start_direction: "SOUTH_WEST"
input: "speck entity, vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Specks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Move Speck

◇ **Pattern** · [[Hexical]] · 📖 [[Specks]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,29.8 72.3,66.6 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2 72.3,140.2 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="93.5,29.8 72.3,66.6 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2 72.3,140.2 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="29.8" x2="86.7" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="81.9,50.0 82.7,39.2 90.8,43.9" fill="#fc77be"/><circle cx="93.5" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeqaa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `speck entity, vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:move_specklike`

```text
before   … │ speck entity │ vector  ◂ top
after    …                          ◂ top
```

## Description

Moves a speck entity to the position.

## Related patterns (same book page)

- [[Conjure Speck]] `any, vector, vector → entity`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Alter Speck]] `speck entity, any → —`
- [[Time Speck]] `speck entity, number → —`
- [[Resize Speck]] `speck entity, number → —`
- [[Thicken Speck]] `speck entity, number → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Dismiss Speck]] `speck entity → —`
- [[Zone Distillation Speck]] `vector, number → list of entities`

## Sources

- 📖 **Guidebook:** [[Specks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Move Speck](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks@hexical:move_specklike)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpecklikeProperty` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:move_specklike`
