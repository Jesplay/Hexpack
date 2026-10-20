---
aliases: ["Tuner's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:transducers_gambit"
signature: "qqqqqwddadada"
start_direction: "SOUTH_WEST"
input: "vector, number | null"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Null]]"]
iotas_out: []
entry: "[[Harmonic Transducer]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Tuner's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[Harmonic Transducer]] · Hex Wright › The Resonance Network

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="59.0,54.3 41.6,84.4 59.0,114.4 93.7,114.4 111.0,84.4 93.7,54.3 59.0,54.3 24.3,54.3 41.6,24.3 59.0,54.3 76.3,24.3 93.7,54.3 111.0,24.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:486;--dur:3.9s" points="59.0,54.3 41.6,84.4 59.0,114.4 93.7,114.4 111.0,84.4 93.7,54.3 59.0,54.3 24.3,54.3 41.6,24.3 59.0,54.3 76.3,24.3 93.7,54.3 111.0,24.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.0" y1="54.3" x2="53.4" y2="63.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.4,70.9 50.1,62.0 56.7,65.9" fill="#fc77be"/><circle cx="59.0" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwddadada` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number | null` ([[Number]], [[Vector]], [[Null]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:transducers_gambit`

## Description

Tells the block at that position which harmonic channel to transmit on. Null resets channel.
Costs 1 Dust

## Sources

- 📖 **Guidebook:** [[Harmonic Transducer]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `HarmonicTransducerActions$1` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:transducers_gambit`
