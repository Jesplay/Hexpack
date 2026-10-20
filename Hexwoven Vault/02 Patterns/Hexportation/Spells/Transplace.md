---
aliases: ["Transplace"]
tags: [hexcasting/spell, mod/hexportation, family/spell, topic/devices, topic/storage]
kind: "Spell"
family: "Spell"
topics: ["Casting Devices", "Reading & Writing"]
mod: "Hexportation"
op_id: "hexportation:transplace"
signature: "aqaawdaqqqaqeaqaa"
start_direction: "SOUTH_EAST"
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

# Transplace

✦ **Spell** · [[Hexportation]] · 📖 [[Conduits]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="22.0,49.3 37.8,76.6 53.5,49.3 37.8,22.0 22.0,49.3 53.5,49.3 85.0,49.3 69.3,76.6 100.7,76.6 116.5,49.3 100.7,22.0 69.3,22.0 85.0,49.3 116.5,49.3 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="22.0,49.3 37.8,76.6 53.5,49.3 37.8,22.0 22.0,49.3 53.5,49.3 85.0,49.3 69.3,76.6 100.7,76.6 116.5,49.3 100.7,22.0 69.3,22.0 85.0,49.3 116.5,49.3 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="49.3" x2="27.1" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,64.3 24.1,59.8 30.1,56.3" fill="#fc77be"/><circle cx="22.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaawdaqqqaqeaqaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `conduit` ([[Conduit]])
- **Leaves:** `nothing` (—)
- **ID:** `hexportation:transplace`

```text
before   … │ conduit  ◂ top
after    …            ◂ top
```

## Description

Will transport any and all entities standing within the source position of the conduit to the sink position of said conduit.
Free if used in a [[Spell Circles|spell circle]], otherwise 1 shard per victim.

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
- [[Spit]] `conduit → —`

## Sources

- 📖 **Guidebook:** [[Conduits]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexportation on Modrinth](https://modrinth.com/mod/AsqM28gQ)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpTransplace` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexportation:transplace`
