---
aliases: ["Threading Reflection"]
tags: [hexcasting/pattern, mod/hexcassettes, family/reflection, topic/control-flow]
kind: "Pattern"
family: "Reflection"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexcassettes"
op_id: "hexcassettes:specs"
signature: "qeqwqwqwqwqeqaweqqqqq"
start_direction: "EAST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Delaying Hexes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Threading Reflection

◇ **Pattern** · [[Hexcassettes]] · 📖 [[Delaying Hexes]] · Patterns › Spells

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.8" cy="20.8" r="2"/><circle cx="50.4" cy="20.8" r="2"/><circle cx="80.1" cy="20.8" r="2"/><circle cx="109.8" cy="20.8" r="2"/><circle cx="139.4" cy="20.8" r="2"/><circle cx="5.9" cy="46.5" r="2"/><circle cx="35.6" cy="46.5" r="2"/><circle cx="65.3" cy="46.5" r="2"/><circle cx="94.9" cy="46.5" r="2"/><circle cx="124.6" cy="46.5" r="2"/><circle cx="154.3" cy="46.5" r="2"/><circle cx="20.8" cy="72.2" r="2"/><circle cx="50.4" cy="72.2" r="2"/><circle cx="80.1" cy="72.2" r="2"/><circle cx="109.8" cy="72.2" r="2"/><circle cx="139.4" cy="72.2" r="2"/><circle cx="5.9" cy="97.8" r="2"/><circle cx="35.6" cy="97.8" r="2"/><circle cx="65.3" cy="97.8" r="2"/><circle cx="94.9" cy="97.8" r="2"/><circle cx="124.6" cy="97.8" r="2"/><circle cx="154.3" cy="97.8" r="2"/><circle cx="20.8" cy="123.5" r="2"/><circle cx="50.4" cy="123.5" r="2"/><circle cx="80.1" cy="123.5" r="2"/><circle cx="109.8" cy="123.5" r="2"/><circle cx="139.4" cy="123.5" r="2"/><circle cx="5.9" cy="149.2" r="2"/><circle cx="35.6" cy="149.2" r="2"/><circle cx="65.3" cy="149.2" r="2"/><circle cx="94.9" cy="149.2" r="2"/><circle cx="124.6" cy="149.2" r="2"/><circle cx="154.3" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="35.6,149.2 65.3,149.2 80.1,123.5 109.8,123.5 124.6,97.8 139.4,72.2 124.6,46.5 109.8,20.8 80.1,20.8 50.4,20.8 35.6,46.5 20.8,72.2 35.6,97.8 20.8,123.5 35.6,149.2 50.4,123.5 65.3,97.8 94.9,97.8 109.8,72.2 94.9,46.5 65.3,46.5 50.4,72.2 65.3,97.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:653;--dur:6.0s" points="35.6,149.2 65.3,149.2 80.1,123.5 109.8,123.5 124.6,97.8 139.4,72.2 124.6,46.5 109.8,20.8 80.1,20.8 50.4,20.8 35.6,46.5 20.8,72.2 35.6,97.8 20.8,123.5 35.6,149.2 50.4,123.5 65.3,97.8 94.9,97.8 109.8,72.2 94.9,46.5 65.3,46.5 50.4,72.2 65.3,97.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.6" y1="149.2" x2="45.1" y2="149.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.9,149.2 45.1,152.5 45.1,146.0" fill="#fc77be"/><circle cx="35.6" cy="149.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeqwqwqwqwqeqaweqqqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexcassettes:specs`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Pushes the total number of **Cassettes** that I have available to me in my mind's collection.

## Related patterns (same book page)

- [[Enqueue]] `list, number → text`
- [[Dequeue]] `pattern → —`
- [[Disqueue]] `— → —`
- [[Operating Reflection]] `— → list of text iotas`
- [[Program Distillation]] `player, text → boolean`
- [[Oracle Purification]] `text → null/number`
- [[State Reflection]] `— → null/text`

## Sources

- 📖 **Guidebook:** [[Delaying Hexes]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpecs` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcassettes:specs`
