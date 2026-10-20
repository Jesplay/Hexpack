---
aliases: ["Open Casting Screen"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices, topic/control-flow]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices", "Control Flow & Meta-evaluation"]
mod: "HexIntent"
op_id: "manifestation:open_casting_screen"
signature: "aqaeawqqwqwqqw"
start_direction: "SOUTH_WEST"
input: "none"
output: "none"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Charming Intent]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Open Casting Screen

◇ **Pattern** · [[HexIntent]] · 📖 [[Charming Intent]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="76.3,54.3 59.0,84.4 93.7,84.4 111.0,54.3 76.3,54.3 59.0,24.3 41.6,54.3 24.3,84.4 41.6,114.4 76.3,114.4 111.0,114.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:520;--dur:4.2s" points="76.3,54.3 59.0,84.4 93.7,84.4 111.0,54.3 76.3,54.3 59.0,24.3 41.6,54.3 24.3,84.4 41.6,114.4 76.3,114.4 111.0,114.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="54.3" x2="70.8" y2="63.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="66.8,70.9 67.5,62.0 74.1,65.9" fill="#fc77be"/><circle cx="76.3" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaeawqqwqwqqw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `none` (—)
- **Leaves:** `none` (—)
- **ID:** `manifestation:open_casting_screen`

```text
before   … │ none  ◂ top
after    … │ none  ◂ top
```

## Description

Opens the spell grid and must be cast by a person.

## Related patterns (same book page)

- [[Set Charm Cast Sound]] `string | text | null, item entity? → none`
- [[Exit If Interacting]] `none → none`
- [[Clear Stack]] `none → none`

## Sources

- 📖 **Guidebook:** [[Charming Intent]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpOpenCastingScreen` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:open_casting_screen`
