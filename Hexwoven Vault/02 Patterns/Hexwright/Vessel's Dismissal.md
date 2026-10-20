---
aliases: ["Vessel's Dismissal"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/movement]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "Movement"]
mod: "Hexwright"
op_id: "hexwright:vessels_dismissal"
signature: "aqawqded"
start_direction: "SOUTH_WEST"
input: "bool"
output: ""
cost: ""
iotas_in: ["[[Boolean]]"]
iotas_out: []
entry: "[[Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vessel's Dismissal

◇ **Pattern** · [[Hexwright]] · 📖 [[Patterns]] · Hex Wright › Flying Devices

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="104.3,60.5 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="104.3,60.5 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="60.5" x2="98.1" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,78.9 94.5,69.1 101.8,73.3" fill="#fc77be"/><circle cx="104.3" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawqded` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `bool` ([[Boolean]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:vessels_dismissal`

```text
before   … │ bool  ◂ top
after    …         ◂ top
```

## Description

Puts everyone off - the pilot, and every passenger on a carpet. True leaves the vehicle hovering where it stopped; false packs it back into the pilot's inventory.
Costs 1 Amethyst Dust

## Related patterns (same book page)

- [[Conveyance's Reflection]] `— → [state]`
- [[Locomotion's Reflection]] `— → num, num`
- [[Speedy's Reflection]] `— → [num]`
- [[Vessel's Reflection]] `— → entity`

## Sources

- 📖 **Guidebook:** [[Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `FlightActions$4` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:vessels_dismissal`
