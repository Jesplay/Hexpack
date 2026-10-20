---
aliases: ["Chorister's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:harmonic_set_state"
signature: "qqaeawaeedqd"
start_direction: "NORTH_EAST"
input: "resonance or null, number, any"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Null]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Harmonic Channels]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Chorister's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[Harmonic Channels]] · Hex Wright › The Resonance Network

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="65.7,94.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="71.9" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,75.6 75.5,85.4 68.2,81.1" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqaeawaeedqd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `resonance or null, number, any` ([[Number]], [[Null]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:harmonic_set_state`

## Description

This is Setting State. Whoever is listening at that instant hears it. Whoever connects to

the harmonic channel later also hears it.

Each harmonic channel has its own state.

Setting that state replaces the current state.

Costs 1 Dust

## Related patterns (same book page)

- [[Chorister's Gambit]] `resonance or null, number, any → —`
- [[Resonance Reflection (Hexwright)]] `— → resonance or null`

## Sources

- 📖 **Guidebook:** [[Harmonic Channels]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `HarmonicActions$Broadcast` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:harmonic_set_state`
