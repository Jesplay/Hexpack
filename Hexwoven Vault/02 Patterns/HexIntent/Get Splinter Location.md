---
aliases: ["Get Splinter Location"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:get_splinter_location"
signature: "dedadeeweewewewee"
start_direction: "SOUTH_WEST"
input: ""
output: "vec"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Splinters]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Get Splinter Location

◇ **Pattern** · [[HexIntent]] · 📖 [[Splinters]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="85.0,76.6 69.3,103.8 53.5,76.6 69.3,49.3 85.0,76.6 100.7,49.3 116.5,76.6 100.7,103.8 69.3,103.8 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 100.7,103.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="85.0,76.6 69.3,103.8 53.5,76.6 69.3,49.3 85.0,76.6 100.7,49.3 116.5,76.6 100.7,103.8 69.3,103.8 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 100.7,103.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="76.6" x2="80.0" y2="85.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,91.6 77.0,83.6 83.0,87.0" fill="#fc77be"/><circle cx="85.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedadeeweewewewee` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vec` ([[Vector]])
- **ID:** `manifestation:get_splinter_location`

```text
before   …        ◂ top
after    … │ vec  ◂ top
```

## Description

Only castable by a splinter. Returns the current vector of the splinter. Useful for self-renewal.

## Related patterns (same book page)

- [[Manifest Splinter]] `vec, num, list → none`
- [[Renew Splinter]] `vec, num → none`
- [[Saturn's Gambit]] `pattern → none`
- [[Destroy Current Splinter]] `— → none`
- [[Destroy Splinters]] `— → none`

## Sources

- 📖 **Guidebook:** [[Splinters]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetSplinterLocation` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:get_splinter_location`
