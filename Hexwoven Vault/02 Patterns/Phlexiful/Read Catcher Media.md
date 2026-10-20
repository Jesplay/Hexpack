---
aliases: ["Read Catcher Media"]
tags: [hexcasting/pattern, mod/phlexiful, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "Phlexiful"
op_id: "phlexiful:read_pants_media"
signature: "aeawaw"
start_direction: "SOUTH_EAST"
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

# Read Catcher Media

◇ **Pattern** · [[Phlexiful]] · 📖 [[Media Catcher Patterns]] · Patterns

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,103.4 51.0,140.2 72.3,103.4 114.8,103.4 93.5,66.6 72.3,29.8 51.0,66.6 29.8,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="29.8,103.4 51.0,140.2 72.3,103.4 114.8,103.4 93.5,66.6 72.3,29.8 51.0,66.6 29.8,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="103.4" x2="36.6" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,123.7 32.5,117.5 40.6,112.9" fill="#fc77be"/><circle cx="29.8" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeawaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `phlexiful:read_pants_media`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Add the **[[Media|media]]** contained in the **media catcher** I'm wearing to the [[The Stack|stack]].

## Related patterns (same book page)

- [[Read Catcher Threshold]] `— → num`
- [[Write Catcher Threshold]] `num → —`

## Sources

- 📖 **Guidebook:** [[Media Catcher Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Phlexiful on Modrinth](https://modrinth.com/mod/wZmv2tbP)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadPantsMedia` is not a `SpellAction`
- 🔧 **Pattern ID:** `phlexiful:read_pants_media`
