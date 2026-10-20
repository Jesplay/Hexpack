---
aliases: ["Terminate Flight"]
tags: [hexcasting/pattern, mod/ephemera, family/other, topic/movement]
kind: "Pattern"
family: "Other"
topics: ["Movement"]
mod: "Ephemera"
op_id: "ephemera:cancelflight"
signature: "awawaawe"
start_direction: "SOUTH_WEST"
input: ""
output: ""
cost: "cost of one tenth of an Amethyst Dust."
iotas_in: []
iotas_out: []
entry: "[[Additional Flight Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Terminate Flight

◇ **Pattern** · [[Ephemera]] · 📖 [[Additional Flight Patterns]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 85.0,60.5 65.7,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 85.0,60.5 65.7,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="40.2" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,78.9 36.5,69.1 43.9,73.3" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awawaawe` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **Cost:** cost of one tenth of an Amethyst Dust.
- **ID:** `ephemera:cancelflight`

## Description

Cancels my current flight ability, at a cost of one tenth of an Amethyst Dust. Casting a new flight spell in the same instant allows me to continue flying, potentially with different parameters.

## Related patterns (same book page)

- [[Aviator's Purification II]] `player → bool | null`
- [[Aviator's Purification III]] `player → num | null`

## Sources

- 📖 **Guidebook:** [[Additional Flight Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Terminate Flight](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/ephemeral_flight@ephemera:cancelflight)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCancelFlight` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:cancelflight`
