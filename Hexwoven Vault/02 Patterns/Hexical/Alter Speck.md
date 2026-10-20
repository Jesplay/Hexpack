---
aliases: ["Alter Speck"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:iota_speck"
signature: "adeeaqa"
start_direction: "SOUTH_WEST"
input: "speck entity, any"
output: ""
cost: ""
iotas_in: ["[[Entity]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Specks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Alter Speck

◇ **Pattern** · [[Hexical]] · 📖 [[Specks]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="114.8,29.8 93.5,66.6 136.1,66.6 114.8,103.4 72.3,103.4 51.0,66.6 29.8,103.4 51.0,140.2 72.3,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:340;--dur:2.2s" points="114.8,29.8 93.5,66.6 136.1,66.6 114.8,103.4 72.3,103.4 51.0,66.6 29.8,103.4 51.0,140.2 72.3,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="114.8" y1="29.8" x2="108.0" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="103.1,50.0 104.0,39.2 112.1,43.9" fill="#fc77be"/><circle cx="114.8" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeeaqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `speck entity, any` ([[Entity]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:iota_speck`

```text
before   … │ speck entity │ any  ◂ top
after    …                       ◂ top
```

## Description

Changes the speck's [[Iotas|iota]].

## Related patterns (same book page)

- [[Conjure Speck]] `any, vector, vector → entity`
- [[Move Speck]] `speck entity, vector → —`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Time Speck]] `speck entity, number → —`
- [[Resize Speck]] `speck entity, number → —`
- [[Thicken Speck]] `speck entity, number → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Dismiss Speck]] `speck entity → —`
- [[Zone Distillation Speck]] `vector, number → list of entities`

## Sources

- 📖 **Guidebook:** [[Specks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Alter Speck](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks@hexical:iota_speck)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpIotaSpeck` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:iota_speck`
