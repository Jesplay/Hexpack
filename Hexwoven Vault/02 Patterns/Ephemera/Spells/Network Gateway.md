---
aliases: ["Network Gateway"]
tags: [hexcasting/spell, mod/ephemera, family/spell, topic/wisps]
kind: "Spell"
family: "Spell"
topics: ["Wisps & Links"]
mod: "Ephemera"
op_id: "ephemera:networktp"
signature: "qqqqqwdeddwdawqqqwaq"
start_direction: "NORTH_WEST"
input: "entity, linkable, vec"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Linkable]]"]
iotas_out: []
entry: "[[Link Amenities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Network Gateway

✦ **Spell** · [[Ephemera]] · 📖 [[Link Amenities]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="72" viewBox="0 0 170.0 72.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.1" r="2"/><circle cx="27.6" cy="16.1" r="2"/><circle cx="50.5" cy="16.1" r="2"/><circle cx="73.5" cy="16.1" r="2"/><circle cx="96.5" cy="16.1" r="2"/><circle cx="119.5" cy="16.1" r="2"/><circle cx="142.4" cy="16.1" r="2"/><circle cx="165.4" cy="16.1" r="2"/><circle cx="16.1" cy="36.0" r="2"/><circle cx="39.1" cy="36.0" r="2"/><circle cx="62.0" cy="36.0" r="2"/><circle cx="85.0" cy="36.0" r="2"/><circle cx="108.0" cy="36.0" r="2"/><circle cx="130.9" cy="36.0" r="2"/><circle cx="153.9" cy="36.0" r="2"/><circle cx="4.6" cy="55.9" r="2"/><circle cx="27.6" cy="55.9" r="2"/><circle cx="50.5" cy="55.9" r="2"/><circle cx="73.5" cy="55.9" r="2"/><circle cx="96.5" cy="55.9" r="2"/><circle cx="119.5" cy="55.9" r="2"/><circle cx="142.4" cy="55.9" r="2"/><circle cx="165.4" cy="55.9" r="2"/></g><polyline class="hexpat-base" points="62.0,36.0 50.5,16.1 27.6,16.1 16.1,36.0 27.6,55.9 50.5,55.9 62.0,36.0 73.5,16.1 85.0,36.0 73.5,55.9 62.0,36.0 85.0,36.0 108.0,36.0 96.5,55.9 119.5,55.9 142.4,55.9 153.9,36.0 142.4,16.1 119.5,16.1 96.5,16.1 108.0,36.0 130.9,36.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:482;--dur:5.9s" points="62.0,36.0 50.5,16.1 27.6,16.1 16.1,36.0 27.6,55.9 50.5,55.9 62.0,36.0 73.5,16.1 85.0,36.0 73.5,55.9 62.0,36.0 85.0,36.0 108.0,36.0 96.5,55.9 119.5,55.9 142.4,55.9 153.9,36.0 142.4,16.1 119.5,16.1 96.5,16.1 108.0,36.0 130.9,36.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="62.0" y1="36.0" x2="58.4" y2="29.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.7,25.0 60.5,28.3 56.2,30.9" fill="#fc77be"/><circle cx="62.0" cy="36.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwdeddwdawqqqwaq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, linkable, vec` ([[Vector]], [[Entity]], [[Linkable]])
- **Leaves:** `nothing` (—)
- **ID:** `ephemera:networktp`

```text
before   … │ entity │ linkable │ vec  ◂ top
after    …                            ◂ top
```

## Description

Accepts an entity to teleport, a linkable to use for access to its network, and a destination. If the entity is within 8 blocks of the linkable, and there is a network node within 4 blocks of the destination, the entity will be teleported. Cost varies based on what entity is teleported.

## Related patterns (same book page)

- [[Network Distillation]] `linkable, num → [linkable]`
- [[Routing Distillation]] `linkable, iota → vec`

## Sources

- 📖 **Guidebook:** [[Link Amenities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Network Gateway](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/network@ephemera:networktp)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpNetworkTeleport` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `ephemera:networktp`
