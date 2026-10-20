---
aliases: ["Redirect"]
tags: [hexcasting/spell, mod/hexwright, family/spell, topic/hexwright, topic/devices]
kind: "Spell"
family: "Spell"
topics: ["Hex Wright Magic", "Casting Devices"]
mod: "Hexwright"
op_id: "hexwright:redirect"
signature: "wawqwawaed"
start_direction: "SOUTH_WEST"
input: "entity, vec"
output: ""
cost: "Costs 1/20 dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Warding Box]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Redirect

✦ **Spell** · [[Hexwright]] · 📖 [[Warding Box]] · Hex Wright › Casting Tools

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 85.0,60.5 65.7,94.0 46.4,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="65.7,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 85.0,60.5 65.7,94.0 46.4,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="27.0" x2="59.5" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,45.4 55.8,35.6 63.2,39.9" fill="#fc77be"/><circle cx="65.7" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawqwawaed` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, vec` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 1/20 dust.
- **ID:** `hexwright:redirect`

```text
before   … │ entity │ vec  ◂ top
after    …                 ◂ top
```

## Description

Moves an entity from it's current position to the position specified.

Costs 1/20 dust.

Both the entity and the destination of redirect must be within the Warding Box's cuboid. Despite the box having an ambit of the cuboid +4 blocks, redirect only works within the cuboid itself.

## Related patterns (same book page)

- [[Watcher's Gambit]] `vec, entity/item type → —`
- [[Warden's Reflection]] `vec → [vec,vec]`
- [[Warden's Gambit]] `[vec,vec] → —`

## Sources

- 📖 **Guidebook:** [[Warding Box]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `WardingBoxActions$4` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:redirect`
