---
aliases: ["Emitter's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:emitters_reflection"
signature: "qqqqqwddadaddqd"
start_direction: "SOUTH_WEST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Harmonic Emitter]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Emitter's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[Harmonic Emitter]] · Hex Wright › The Resonance Network

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 46.4,94.0 65.7,127.4 104.3,127.4 123.6,94.0 104.3,60.5 65.7,60.5 27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 104.3,60.5 85.0,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:618;--dur:4.5s" points="65.7,60.5 46.4,94.0 65.7,127.4 104.3,127.4 123.6,94.0 104.3,60.5 65.7,60.5 27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 104.3,60.5 85.0,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="59.5" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,78.9 55.8,69.1 63.2,73.3" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwddadaddqd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexwright:emitters_reflection`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Returns the vector location of the emitter. Only works inside an Emitter's cast.

## Related patterns (same book page)

- [[Emitter's Gambit]] `vector, number, [pattern] | null → —`

## Sources

- 📖 **Guidebook:** [[Harmonic Emitter]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `HarmonicEmitterActions$1` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:emitters_reflection`
