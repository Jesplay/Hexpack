---
aliases: ["Gearbox Reflection"]
tags: [hexcasting/pattern, mod/hexic, family/reflection, topic/circles]
kind: "Pattern"
family: "Reflection"
topics: ["Spell Circles, Loci & Engines"]
mod: "Hexic"
op_id: "hexic:engine/pos"
signature: "eedaddawdd"
start_direction: "SOUTH_EAST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Casting Engine Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Gearbox Reflection

◇ **Pattern** · [[Hexic]] · 📖 [[Casting Engine Patterns]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 72.3,103.4 29.8,103.4 51.0,66.6 72.3,103.4 93.5,66.6 114.8,29.8 136.1,66.6 93.5,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 72.3,103.4 29.8,103.4 51.0,66.6 72.3,103.4 93.5,66.6 114.8,29.8 136.1,66.6 93.5,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="66.6" x2="100.3" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.2,86.8 96.3,80.7 104.4,76.0" fill="#fc77be"/><circle cx="93.5" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eedaddawdd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexic:engine/pos`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Returns the position of the engine casting the current **Hex**. If any of these patterns are used outside a casting engine, [[Mishaps|mishaps]] rather spectacularly.

## Related patterns (same book page)

- [[Engineer's Distraction]] `number → —`
- [[Engineer's Departure]] `— → —`
- [[Engineer's Resignation]] `— → ...`

## Sources

- 📖 **Guidebook:** [[Casting Engine Patterns]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexic:engine/pos`
