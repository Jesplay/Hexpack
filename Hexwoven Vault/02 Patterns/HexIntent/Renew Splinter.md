---
aliases: ["Renew Splinter"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:renew_splinter"
signature: "dedaded"
start_direction: "SOUTH_WEST"
input: "vec, num"
output: "none"
cost: "Cost: replacing at the same position is free but moving to a new position costs 0.5 dust."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Splinters]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Renew Splinter

◇ **Pattern** · [[HexIntent]] · 📖 [[Splinters]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 60.0,121.6 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="85.0,78.3 60.0,121.6 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="77.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="71.2,102.1 72.2,89.4 81.8,94.9" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedaded` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `none` (—)
- **Cost:** Cost: replacing at the same position is free but moving to a new position costs 0.5 dust.
- **ID:** `manifestation:renew_splinter`

```text
before   … │ vec │ num  ◂ top
after    … │ none       ◂ top
```

## Description

Only castable by a splinter. Renews the current splinter, with the same pattern. Preserves the Ravenmind between splinters.

Inputs:
position: vector to summon at.
delay: time in ticks before cast after current splinter work has finished.

Cost: replacing at the same position is free but moving to a new position costs 0.5 dust.

## Related patterns (same book page)

- [[Manifest Splinter]] `vec, num, list → none`
- [[Saturn's Gambit]] `pattern → none`
- [[Get Splinter Location]] `— → vec`
- [[Destroy Current Splinter]] `— → none`
- [[Destroy Splinters]] `— → none`

## Sources

- 📖 **Guidebook:** [[Splinters]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRenewSplinter` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:renew_splinter`
