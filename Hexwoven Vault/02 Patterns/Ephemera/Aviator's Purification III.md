---
aliases: ["Aviator's Purification III"]
tags: [hexcasting/pattern, mod/ephemera, family/purification, topic/movement]
kind: "Pattern"
family: "Purification"
topics: ["Movement"]
mod: "Ephemera"
op_id: "ephemera:getflight/remaining"
signature: "dwdwdewqdedd"
start_direction: "NORTH_EAST"
input: "player"
output: "num | null"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]", "[[Null]]"]
entry: "[[Additional Flight Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Aviator's Purification III

◇ **Pattern** · [[Ephemera]] · 📖 [[Additional Flight Patterns]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="41.6,54.3 59.0,24.3 76.3,54.3 93.7,84.4 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 111.0,54.3 128.4,24.3 145.7,54.3 128.4,84.4 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:451;--dur:3.6s" points="41.6,54.3 59.0,24.3 76.3,54.3 93.7,84.4 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 111.0,54.3 128.4,24.3 145.7,54.3 128.4,84.4 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.6" y1="54.3" x2="47.2" y2="44.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.2,37.8 50.5,46.6 43.9,42.8" fill="#fc77be"/><circle cx="41.6" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwdwdewqdedd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player` ([[Entity]])
- **Leaves:** `num | null` ([[Number]], [[Null]])
- **ID:** `ephemera:getflight/remaining`

## Description

Returns how close the target is to the end of their flight spell, represented as either their distance to the boundary or how many seconds remain.

## Related patterns (same book page)

- [[Terminate Flight]] `— → —`
- [[Aviator's Purification II]] `player → bool | null`

## Sources

- 📖 **Guidebook:** [[Additional Flight Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Aviator's Purification III](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/ephemeral_flight@ephemera:getflight/remaining)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpFlightRemaining` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getflight/remaining`
