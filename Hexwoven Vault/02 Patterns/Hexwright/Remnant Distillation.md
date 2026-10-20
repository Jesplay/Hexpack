---
aliases: ["Remnant Distillation"]
tags: [hexcasting/pattern, mod/hexwright, family/distillation, topic/hexwright]
kind: "Pattern"
family: "Distillation"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:remnant_distillation"
signature: "wqwqwqwqweawwaweqqqqe"
start_direction: "SOUTH_WEST"
input: "remnants, num | str"
output: "remnant"
cost: "Costs 1/4 Dust."
iotas_in: ["[[Number]]", "[[String]]", "[[Remnant]]"]
iotas_out: ["[[Remnant]]"]
entry: "[[Distillation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Remnant Distillation

◇ **Pattern** · [[Hexwright]] · 📖 [[Distillation]] · Hex Wright › Remnants

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="154.3" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="154.3" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="154.3" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="50.4,46.5 35.6,72.2 20.8,97.8 35.6,123.5 50.4,149.2 80.1,149.2 109.8,149.2 124.6,123.5 139.4,97.8 124.6,72.2 109.8,46.5 124.6,20.8 94.9,20.8 65.3,20.8 35.6,20.8 50.4,46.5 65.3,72.2 50.4,97.8 65.3,123.5 94.9,123.5 109.8,97.8 94.9,72.2 109.8,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:653;--dur:6.0s" points="50.4,46.5 35.6,72.2 20.8,97.8 35.6,123.5 50.4,149.2 80.1,149.2 109.8,149.2 124.6,123.5 139.4,97.8 124.6,72.2 109.8,46.5 124.6,20.8 94.9,20.8 65.3,20.8 35.6,20.8 50.4,46.5 65.3,72.2 50.4,97.8 65.3,123.5 94.9,123.5 109.8,97.8 94.9,72.2 109.8,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.4" y1="46.5" x2="45.7" y2="54.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.3,60.6 42.9,53.1 48.5,56.3" fill="#fc77be"/><circle cx="50.4" cy="46.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqweawwaweqqqqe` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `remnants, num | str` ([[Number]], [[String]], [[Remnant]])
- **Leaves:** `remnant` ([[Remnant]])
- **Cost:** Costs 1/4 Dust.
- **ID:** `hexwright:remnant_distillation`

## Description

Distils one remnant from the remnants, by index or name. Empty if not present.
Costs 1/4 Dust.

## Related patterns (same book page)

- [[Remnant's Reflection]] `remnants → list`

## Sources

- 📖 **Guidebook:** [[Distillation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RemnantActions$5` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:remnant_distillation`
