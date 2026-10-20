---
aliases: ["Clear Stack"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices, topic/control-flow]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices", "Control Flow & Meta-evaluation"]
mod: "HexIntent"
op_id: "manifestation:clear_stack"
signature: "aqaeawqqwa"
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

# Clear Stack

◇ **Pattern** · [[HexIntent]] · 📖 [[Charming Intent]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,66.6 72.3,103.4 114.8,103.4 136.1,66.6 93.5,66.6 72.3,29.8 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 136.1,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="93.5,66.6 72.3,103.4 114.8,103.4 136.1,66.6 93.5,66.6 72.3,29.8 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 136.1,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="66.6" x2="86.7" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="81.9,86.8 82.7,76.0 90.8,80.7" fill="#fc77be"/><circle cx="93.5" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaeawqqwa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `none` (—)
- **Leaves:** `none` (—)
- **ID:** `manifestation:clear_stack`

```text
before   … │ none  ◂ top
after    … │ none  ◂ top
```

## Description

Clears the caster's persisted [[Staves|staff]] [[The Stack|stack]] (the shift, right-click stack). Must be cast by a person.

## Related patterns (same book page)

- [[Set Charm Cast Sound]] `string | text | null, item entity? → none`
- [[Exit If Interacting]] `none → none`
- [[Open Casting Screen]] `none → none`

## Sources

- 📖 **Guidebook:** [[Charming Intent]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpClearStack` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:clear_stack`
