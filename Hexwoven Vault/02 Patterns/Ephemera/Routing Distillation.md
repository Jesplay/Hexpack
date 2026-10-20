---
aliases: ["Routing Distillation"]
tags: [hexcasting/pattern, mod/ephemera, family/distillation, topic/wisps]
kind: "Pattern"
family: "Distillation"
topics: ["Wisps & Links"]
mod: "Ephemera"
op_id: "ephemera:networkindex"
signature: "eqqqqqaweqaeaqa"
start_direction: "EAST"
input: "linkable, iota"
output: "vec"
cost: "Costs half an amethyst dust."
iotas_in: ["[[Linkable]]", "[[Any Iota]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Link Amenities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Routing Distillation

◇ **Pattern** · [[Ephemera]] · 📖 [[Link Amenities]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="90" viewBox="0 0 170.0 90.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.8" cy="20.2" r="2"/><circle cx="34.6" cy="20.2" r="2"/><circle cx="63.4" cy="20.2" r="2"/><circle cx="92.2" cy="20.2" r="2"/><circle cx="121.0" cy="20.2" r="2"/><circle cx="149.8" cy="20.2" r="2"/><circle cx="20.2" cy="45.1" r="2"/><circle cx="49.0" cy="45.1" r="2"/><circle cx="77.8" cy="45.1" r="2"/><circle cx="106.6" cy="45.1" r="2"/><circle cx="135.4" cy="45.1" r="2"/><circle cx="164.2" cy="45.1" r="2"/><circle cx="5.8" cy="70.1" r="2"/><circle cx="34.6" cy="70.1" r="2"/><circle cx="63.4" cy="70.1" r="2"/><circle cx="92.2" cy="70.1" r="2"/><circle cx="121.0" cy="70.1" r="2"/><circle cx="149.8" cy="70.1" r="2"/></g><polyline class="hexpat-base" points="20.2,45.1 49.0,45.1 63.4,70.1 92.2,70.1 106.6,45.1 92.2,20.2 63.4,20.2 49.0,45.1 77.8,45.1 106.6,45.1 121.0,70.1 149.8,70.1 135.4,45.1 149.8,20.2 121.0,20.2 106.6,45.1 135.4,45.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:461;--dur:4.5s" points="20.2,45.1 49.0,45.1 63.4,70.1 92.2,70.1 106.6,45.1 92.2,20.2 63.4,20.2 49.0,45.1 77.8,45.1 106.6,45.1 121.0,70.1 149.8,70.1 135.4,45.1 149.8,20.2 121.0,20.2 106.6,45.1 135.4,45.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.2" y1="45.1" x2="29.4" y2="45.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="36.0,45.1 29.4,48.3 29.4,42.0" fill="#fc77be"/><circle cx="20.2" cy="45.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqqqaweqaeaqa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `linkable, iota` ([[Linkable]], [[Any Iota]])
- **Leaves:** `vec` ([[Vector]])
- **Cost:** Costs half an amethyst dust.
- **ID:** `ephemera:networkindex`

```text
before   … │ linkable │ iota  ◂ top
after    … │ vec              ◂ top
```

## Description

Accepts a linkable to use for network access, and any [[Iotas|iota]]. If a relay connected to the network has an adjacent Network Routing Index with the same iota, returns that relay's position. Costs half an amethyst dust.

## Related patterns (same book page)

- [[Network Distillation]] `linkable, num → [linkable]`
- [[Network Gateway]] `entity, linkable, vec → —`

## Sources

- 📖 **Guidebook:** [[Link Amenities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Routing Distillation](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/network@ephemera:networkindex)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNodeIndex` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:networkindex`
