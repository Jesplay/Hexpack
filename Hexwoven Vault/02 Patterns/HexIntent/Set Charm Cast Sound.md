---
aliases: ["Set Charm Cast Sound"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices, topic/control-flow]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices", "Control Flow & Meta-evaluation"]
mod: "HexIntent"
op_id: "manifestation:set_charm_cast_sound"
signature: "wedwwdwee"
start_direction: "EAST"
input: "string | text | null, item entity?"
output: "none"
cost: "Cost: 10 dust."
iotas_in: ["[[Entity]]", "[[Null]]", "[[String]]"]
iotas_out: []
entry: "[[Charming Intent]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Set Charm Cast Sound

◇ **Pattern** · [[HexIntent]] · 📖 [[Charming Intent]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 85.0,60.5 123.6,60.5 143.0,94.0 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="46.4,60.5 85.0,60.5 123.6,60.5 143.0,94.0 104.3,94.0 65.7,94.0 27.0,94.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="58.7" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.6,60.5 58.7,64.8 58.7,56.3" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wedwwdwee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `string | text | null, item entity?` ([[Entity]], [[Null]], [[String]])
- **Leaves:** `none` (—)
- **Cost:** Cost: 10 dust.
- **ID:** `manifestation:set_charm_cast_sound`

## Description

Changes the sound a charmed item makes when casting.

Top of [[The Stack|stack]] is string, text, or null.
Cost: 10 dust.

Use string sound id (namespace:path) to change the charm in the offhand to that sound, or null to mute.

Optionally you can pass a charmed item entity at the top of the stack to change that instead.

## Related patterns (same book page)

- [[Exit If Interacting]] `none → none`
- [[Open Casting Screen]] `none → none`
- [[Clear Stack]] `none → none`

## Sources

- 📖 **Guidebook:** [[Charming Intent]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetCharmCastSound` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:set_charm_cast_sound`
