---
aliases: ["Wave Facing Rfln."]
tags: [hexcasting/pattern, mod/slate-works, family/reflection, topic/circles]
kind: "Pattern"
family: "Reflection"
topics: ["Spell Circles, Loci & Engines"]
mod: "Slate Works"
op_id: "slate_work:wave_normal"
signature: "eaqdaadqaeewa"
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

# Wave Facing Rfln.

◇ **Pattern** · [[Slate Works]] · 📖 [[Slate Reflections]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="131" height="170" viewBox="0 0 130.5 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="65.3,72.2 50.4,97.8 20.8,97.8 35.6,123.5 65.3,123.5 50.4,149.2 80.1,149.2 65.3,123.5 94.9,123.5 109.8,97.8 80.1,97.8 65.3,72.2 80.1,46.5 94.9,20.8 65.3,20.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:415;--dur:3.9s" points="65.3,72.2 50.4,97.8 20.8,97.8 35.6,123.5 65.3,123.5 50.4,149.2 80.1,149.2 65.3,123.5 94.9,123.5 109.8,97.8 80.1,97.8 65.3,72.2 80.1,46.5 94.9,20.8 65.3,20.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.3" y1="72.2" x2="60.5" y2="80.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.1,86.3 57.7,78.7 63.3,82.0" fill="#fc77be"/><circle cx="65.3" cy="72.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqdaadqaeewa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `slate_work:wave_normal`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Adds the current facing of the **[[Media|Media]] Wave** to the top of the [[The Stack|stack]]. If the current block does not have a facing, it returns a vector of \[0,0,0\].

## Related patterns (same book page)

- [[Wave Location Rfln]] `— → vector`
- [[Wave Speed Rfln]] `— → number`
- [[Circle Media Rfln]] `— → number`

## Sources

- 📖 **Guidebook:** [[Slate Reflections]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CircleReflection` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:wave_normal`
