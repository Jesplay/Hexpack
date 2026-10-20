---
aliases: ["Resonance Reflection"]
tags: [hexcasting/pattern, mod/hexintent, family/reflection, topic/storage]
kind: "Pattern"
family: "Reflection"
topics: ["Reading & Writing"]
mod: "HexIntent"
op_id: "manifestation:memory_reflection"
signature: "qwawqwaqw"
start_direction: "SOUTH_WEST"
input: "none"
output: "memory"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Memory Crystal]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Resonance Reflection

◇ **Pattern** · [[HexIntent]] · 📖 [[Memory Crystal]] · Items

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="54.7" x2="36.3" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="32.3,71.4 33.0,62.5 39.7,66.3" fill="#fc77be"/><circle cx="41.9" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwawqwaqw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `none` (—)
- **Leaves:** `memory` (—)
- **ID:** `manifestation:memory_reflection`

```text
before   … │ none    ◂ top
after    … │ memory  ◂ top
```

## Description

Returns the held or embedded Memory Crystal's unique memory identifier. Favours the off-hand first.

## Related patterns (same book page)

- [[Resonance Purification]] `memory → [pattern] | null`
- [[Resonance Gambit]] `memory, any → none`

## Sources

- 📖 **Guidebook:** [[Memory Crystal]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMemoryReflection` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:memory_reflection`
