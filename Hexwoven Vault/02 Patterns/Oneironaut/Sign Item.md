---
aliases: ["Sign Item"]
tags: [hexcasting/pattern, mod/oneironaut, family/other, topic/items, hexcasting/not-in-book]
kind: "Pattern"
family: "Other"
topics: ["Items & Crafting"]
mod: "Oneironaut"
op_id: "oneironaut:signitem"
signature: "qqaqwedeea"
start_direction: "EAST"
input: ""
output: ""
cost: "Costs 1 amethyst dust."
iotas_in: []
iotas_out: []
entry: "[[Undocumented Oneironaut Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: false
---

# Sign Item

> [!caution] Not in the guidebook
> This pattern exists in the mod's code, but the in-game book doesn't describe it. The description below comes from the official online docs.

◇ **Pattern** · [[Oneironaut]] · 📖 [[Undocumented Oneironaut Patterns]] · Not in the Book

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,85.0 59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3 59.4,145.5 41.9,115.3 59.4,85.0 94.4,85.0 76.9,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:384;--dur:3.1s" points="24.5,85.0 59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3 59.4,145.5 41.9,115.3 59.4,85.0 94.4,85.0 76.9,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="85.0" x2="35.6" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.7,85.0 35.6,88.8 35.6,81.2" fill="#fc77be"/><circle cx="24.5" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqaqwedeea` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **Cost:** Costs 1 amethyst dust.
- **ID:** `oneironaut:signitem`

## Description

Invisibly engraves my soulprint onto the item in my other hand, or erases it if it is already there. Costs 1 amethyst dust.I should be careful to seal any focuses or similar that I sign, as acquiring an unsealed signed [[Focus|focus]] may allow someone to impersonate me, and Erase Item gets rid of the signature.

\*(Description from the official online docs; the in-game book doesn't include it.)\*

## Related patterns (same book page)

- [[Cellular Generation]] `— → —`
- [[Signature Purification]] `soulprint → bool`
- [[Circle]] `— → —`
- [[Stabilize Matter]] `— → —`
- [[Soulprint Reflection]] `— → soulprint`

## Sources

- 📖 **Guidebook:** [[Undocumented Oneironaut Patterns]] *(not in the in-game book)*
- 🌐 **Official online docs:** [Sign Item](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#patterns/soulprints@oneironaut:signitem)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSignItem` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:signitem`
