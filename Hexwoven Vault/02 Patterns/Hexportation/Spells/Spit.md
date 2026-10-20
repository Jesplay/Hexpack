---
aliases: ["Spit"]
tags: [hexcasting/spell, mod/hexportation, family/spell, topic/devices, topic/storage]
kind: "Spell"
family: "Spell"
topics: ["Casting Devices", "Reading & Writing"]
mod: "Hexportation"
op_id: "hexportation:spit"
signature: "daqqqaqeaqaa"
start_direction: "EAST"
input: "conduit"
output: ""
cost: ""
iotas_in: ["[[Conduit]]"]
iotas_out: []
entry: "[[Conduits]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Spit

✦ **Spell** · [[Hexportation]] · 📖 [[Conduits]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="27.0,60.5 65.7,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 104.3,60.5 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `daqqqaqeaqaa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `conduit` ([[Conduit]])
- **Leaves:** `nothing` (—)
- **ID:** `hexportation:spit`

```text
before   … │ conduit  ◂ top
after    …            ◂ top
```

## Description

Will take the contents of the first filled slot of the source inventory and drop it at the conduit's target. Can also place a buckets worth of fluid in the same way.
Free if used in a [[Spell Circles|Spell Circle]]

## Related patterns (same book page)

- [[Create Conduit]] `vec, vec, vec, vec → conduit`
- [[Send]] `conduit, num → —`
- [[Source Data]] `conduit → [view, ...]`
- [[Sink Data]] `conduit → [view, ...]`
- [[Metered Send]] `conduit, [num, ...] → —`
- [[View Prfn. Amount]] `view → num`
- [[View Prfn. Capacity]] `view → num`
- [[View Prfn. Name]] `view → string`
- [[Slurp]] `conduit → —`
- [[Transplace]] `conduit → —`

## Sources

- 📖 **Guidebook:** [[Conduits]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexportation on Modrinth](https://modrinth.com/mod/AsqM28gQ)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSpit` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexportation:spit`
