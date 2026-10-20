---
aliases: ["Program Distillation"]
tags: [hexcasting/pattern, mod/hexcassettes, family/distillation, topic/control-flow]
kind: "Pattern"
family: "Distillation"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexcassettes"
op_id: "hexcassettes:inspect"
signature: "eqeweweweweqedwqeeeee"
start_direction: "WEST"
input: "player, text"
output: "boolean"
cost: ""
iotas_in: ["[[Entity]]", "[[String]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Delaying Hexes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Program Distillation

◇ **Pattern** · [[Hexcassettes]] · 📖 [[Delaying Hexes]] · Patterns › Spells

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.8" cy="20.8" r="2"/><circle cx="50.4" cy="20.8" r="2"/><circle cx="80.1" cy="20.8" r="2"/><circle cx="109.8" cy="20.8" r="2"/><circle cx="139.4" cy="20.8" r="2"/><circle cx="5.9" cy="46.5" r="2"/><circle cx="35.6" cy="46.5" r="2"/><circle cx="65.3" cy="46.5" r="2"/><circle cx="94.9" cy="46.5" r="2"/><circle cx="124.6" cy="46.5" r="2"/><circle cx="154.3" cy="46.5" r="2"/><circle cx="20.8" cy="72.2" r="2"/><circle cx="50.4" cy="72.2" r="2"/><circle cx="80.1" cy="72.2" r="2"/><circle cx="109.8" cy="72.2" r="2"/><circle cx="139.4" cy="72.2" r="2"/><circle cx="5.9" cy="97.8" r="2"/><circle cx="35.6" cy="97.8" r="2"/><circle cx="65.3" cy="97.8" r="2"/><circle cx="94.9" cy="97.8" r="2"/><circle cx="124.6" cy="97.8" r="2"/><circle cx="154.3" cy="97.8" r="2"/><circle cx="20.8" cy="123.5" r="2"/><circle cx="50.4" cy="123.5" r="2"/><circle cx="80.1" cy="123.5" r="2"/><circle cx="109.8" cy="123.5" r="2"/><circle cx="139.4" cy="123.5" r="2"/><circle cx="5.9" cy="149.2" r="2"/><circle cx="35.6" cy="149.2" r="2"/><circle cx="65.3" cy="149.2" r="2"/><circle cx="94.9" cy="149.2" r="2"/><circle cx="124.6" cy="149.2" r="2"/><circle cx="154.3" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="124.6,149.2 94.9,149.2 80.1,123.5 50.4,123.5 35.6,97.8 20.8,72.2 35.6,46.5 50.4,20.8 80.1,20.8 109.8,20.8 124.6,46.5 139.4,72.2 124.6,97.8 139.4,123.5 124.6,149.2 109.8,123.5 94.9,97.8 65.3,97.8 50.4,72.2 65.3,46.5 94.9,46.5 109.8,72.2 94.9,97.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:653;--dur:6.0s" points="124.6,149.2 94.9,149.2 80.1,123.5 50.4,123.5 35.6,97.8 20.8,72.2 35.6,46.5 50.4,20.8 80.1,20.8 109.8,20.8 124.6,46.5 139.4,72.2 124.6,97.8 139.4,123.5 124.6,149.2 109.8,123.5 94.9,97.8 65.3,97.8 50.4,72.2 65.3,46.5 94.9,46.5 109.8,72.2 94.9,97.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="124.6" y1="149.2" x2="115.1" y2="149.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="108.3,149.2 115.1,146.0 115.1,152.5" fill="#fc77be"/><circle cx="124.6" cy="149.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqeweweweweqedwqeeeee` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player, text` ([[Entity]], [[String]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexcassettes:inspect`

```text
before   … │ player │ text  ◂ top
after    … │ boolean        ◂ top
```

## Description

Pushes whether a given player has a **Cassette** with a certain label.

## Related patterns (same book page)

- [[Enqueue]] `list, number → text`
- [[Dequeue]] `pattern → —`
- [[Disqueue]] `— → —`
- [[Threading Reflection]] `— → number`
- [[Operating Reflection]] `— → list of text iotas`
- [[Oracle Purification]] `text → null/number`
- [[State Reflection]] `— → null/text`

## Sources

- 📖 **Guidebook:** [[Delaying Hexes]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpInspect` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcassettes:inspect`
