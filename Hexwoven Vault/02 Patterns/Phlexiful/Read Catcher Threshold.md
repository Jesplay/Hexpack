---
aliases: ["Read Catcher Threshold"]
tags: [hexcasting/pattern, mod/phlexiful, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "Phlexiful"
op_id: "phlexiful:get_pants_threshhold"
signature: "dqawa"
start_direction: "NORTH_EAST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Media Catcher Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Read Catcher Threshold

◇ **Pattern** · [[Phlexiful]] · 📖 [[Media Catcher Patterns]] · Patterns

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,121.6 60.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="35.0,121.6 60.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="121.6" x2="43.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,97.8 47.8,110.5 38.2,105.0" fill="#fc77be"/><circle cx="35.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqawa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `phlexiful:get_pants_threshhold`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Add the threshold of the **[[Media|media]] catcher** I'm wearing to the [[The Stack|stack]]. Returns Null if I have not set a threshold.

## Related patterns (same book page)

- [[Write Catcher Threshold]] `num → —`
- [[Read Catcher Media]] `— → num`

## Sources

- 📖 **Guidebook:** [[Media Catcher Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Phlexiful on Modrinth](https://modrinth.com/mod/wZmv2tbP)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadPantsThreshhold` is not a `SpellAction`
- 🔧 **Pattern ID:** `phlexiful:get_pants_threshhold`
