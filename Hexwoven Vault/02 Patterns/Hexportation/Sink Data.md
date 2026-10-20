---
aliases: ["Sink Data"]
tags: [hexcasting/pattern, mod/hexportation, family/other, topic/devices, topic/storage]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices", "Reading & Writing"]
mod: "Hexportation"
op_id: "hexportation:get_sink_inv"
signature: "deeedeqeeeeedw"
start_direction: "NORTH_WEST"
input: "conduit"
output: "[view, ...]"
cost: ""
iotas_in: ["[[Conduit]]"]
iotas_out: ["[[List]]"]
entry: "[[Conduits]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sink Data

◇ **Pattern** · [[Hexportation]] · 📖 [[Conduits]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="41.6,54.3 24.3,24.3 59.0,24.3 76.3,54.3 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:520;--dur:4.2s" points="41.6,54.3 24.3,24.3 59.0,24.3 76.3,54.3 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.6" y1="54.3" x2="36.1" y2="44.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="32.1,37.8 39.4,42.8 32.8,46.6" fill="#fc77be"/><circle cx="41.6" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeedeqeeeeedw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `conduit` ([[Conduit]])
- **Leaves:** `[view, ...]` ([[List]])
- **ID:** `hexportation:get_sink_inv`

## Description

Returns a list of Views that reperesent each slot in the conduit's sink storage.

## Related patterns (same book page)

- [[Create Conduit]] `vec, vec, vec, vec → conduit`
- [[Send]] `conduit, num → —`
- [[Source Data]] `conduit → [view, ...]`
- [[Metered Send]] `conduit, [num, ...] → —`
- [[View Prfn. Amount]] `view → num`
- [[View Prfn. Capacity]] `view → num`
- [[View Prfn. Name]] `view → string`
- [[Slurp]] `conduit → —`
- [[Spit]] `conduit → —`
- [[Transplace]] `conduit → —`

## Sources

- 📖 **Guidebook:** [[Conduits]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexportation on Modrinth](https://modrinth.com/mod/AsqM28gQ)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetInvData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexportation:get_sink_inv`
