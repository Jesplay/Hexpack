---
aliases: ["Wave Location Rfln."]
tags: [hexcasting/pattern, mod/slate-works, family/reflection, topic/circles]
kind: "Pattern"
family: "Reflection"
topics: ["Spell Circles, Loci & Engines"]
mod: "Slate Works"
op_id: "slate_work:wave_position"
signature: "eaqdaadqaeeaa"
start_direction: "SOUTH_WEST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Slate Reflections]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Wave Location Rfln.

◇ **Pattern** · [[Slate Works]] · 📖 [[Slate Reflections]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,54.7 59.4,85.0 24.5,85.0 41.9,115.3 76.9,115.3 59.4,145.5 94.4,145.5 76.9,115.3 111.8,115.3 129.3,85.0 94.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 76.9,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:489;--dur:3.9s" points="76.9,54.7 59.4,85.0 24.5,85.0 41.9,115.3 76.9,115.3 59.4,145.5 94.4,145.5 76.9,115.3 111.8,115.3 129.3,85.0 94.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 76.9,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="54.7" x2="71.3" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,71.4 68.0,62.5 74.6,66.3" fill="#fc77be"/><circle cx="76.9" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqdaadqaeeaa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `slate_work:wave_position`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Adds the current position of the **[[Media|Media]] Wave** to the top of the [[The Stack|stack]].

## Related patterns (same book page)

- [[Wave Facing Rfln]] `— → vector`
- [[Wave Speed Rfln]] `— → number`
- [[Circle Media Rfln]] `— → number`

## Sources

- 📖 **Guidebook:** [[Slate Reflections]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CircleReflection` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:wave_position`
