---
aliases: ["Exit If Interacting"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices, topic/control-flow]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices", "Control Flow & Meta-evaluation"]
mod: "HexIntent"
op_id: "manifestation:exit_if_interacting"
signature: "qaqqqqe"
start_direction: "EAST"
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

# Exit If Interacting

◇ **Pattern** · [[HexIntent]] · 📖 [[Charming Intent]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="27.0,60.5 65.7,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqe` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `none` (—)
- **Leaves:** `none` (—)
- **ID:** `manifestation:exit_if_interacting`

```text
before   … │ none  ◂ top
after    … │ none  ◂ top
```

## Description

Exits a hex if caster uses something interactable. Useful for charmed items that use righ-click.

## Related patterns (same book page)

- [[Set Charm Cast Sound]] `string | text | null, item entity? → none`
- [[Open Casting Screen]] `none → none`
- [[Clear Stack]] `none → none`

## Sources

- 📖 **Guidebook:** [[Charming Intent]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpExitIfInteracting` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:exit_if_interacting`
