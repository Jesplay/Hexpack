---
aliases: ["Wave Speed Rfln."]
tags: [hexcasting/pattern, mod/slate-works, family/reflection, topic/circles]
kind: "Pattern"
family: "Reflection"
topics: ["Spell Circles, Loci & Engines"]
mod: "Slate Works"
op_id: "slate_work:wave_speed"
signature: "eaqdaadqaeewq"
start_direction: "SOUTH_WEST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Slate Reflections]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Wave Speed Rfln.

◇ **Pattern** · [[Slate Works]] · 📖 [[Slate Reflections]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="113" height="170" viewBox="0 0 113.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="95.4" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="108.2" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="95.4" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="108.2" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="95.4" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="108.2" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/><circle cx="95.4" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="56.7,85.0 43.8,107.3 18.0,107.3 30.9,129.6 56.7,129.6 43.8,152.0 69.6,152.0 56.7,129.6 82.5,129.6 95.4,107.3 69.6,107.3 56.7,85.0 69.6,62.7 82.5,40.4 69.6,18.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:361;--dur:3.9s" points="56.7,85.0 43.8,107.3 18.0,107.3 30.9,129.6 56.7,129.6 43.8,152.0 69.6,152.0 56.7,129.6 82.5,129.6 95.4,107.3 69.6,107.3 56.7,85.0 69.6,62.7 82.5,40.4 69.6,18.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="56.7" y1="85.0" x2="52.6" y2="92.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.6,97.3 50.1,90.7 55.0,93.6" fill="#fc77be"/><circle cx="56.7" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqdaadqaeewq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `slate_work:wave_speed`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Adds the current speed of the **[[Media|Media]] Wave** to the top of the [[The Stack|stack]]. This is measured in how many 20ths of a second the **Media Wave** waits until going to the next [[Slates|Slate.]]

## Related patterns (same book page)

- [[Wave Location Rfln]] `— → vector`
- [[Wave Facing Rfln]] `— → vector`
- [[Circle Media Rfln]] `— → number`

## Sources

- 📖 **Guidebook:** [[Slate Reflections]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CircleReflection` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:wave_speed`
