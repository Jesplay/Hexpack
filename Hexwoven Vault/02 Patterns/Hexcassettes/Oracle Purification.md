---
aliases: ["Oracle Purification"]
tags: [hexcasting/pattern, mod/hexcassettes, family/purification, topic/control-flow]
kind: "Pattern"
family: "Purification"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexcassettes"
op_id: "hexcassettes:foretell"
signature: "eqeweweweweqedwqeeeeedww"
start_direction: "WEST"
input: "text"
output: "null/number"
cost: ""
iotas_in: ["[[String]]"]
iotas_out: ["[[Number]]", "[[Null]]"]
entry: "[[Delaying Hexes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Oracle Purification

◇ **Pattern** · [[Hexcassettes]] · 📖 [[Delaying Hexes]] · Patterns › Spells

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.8" cy="20.8" r="2"/><circle cx="50.4" cy="20.8" r="2"/><circle cx="80.1" cy="20.8" r="2"/><circle cx="109.8" cy="20.8" r="2"/><circle cx="139.4" cy="20.8" r="2"/><circle cx="5.9" cy="46.5" r="2"/><circle cx="35.6" cy="46.5" r="2"/><circle cx="65.3" cy="46.5" r="2"/><circle cx="94.9" cy="46.5" r="2"/><circle cx="124.6" cy="46.5" r="2"/><circle cx="154.3" cy="46.5" r="2"/><circle cx="20.8" cy="72.2" r="2"/><circle cx="50.4" cy="72.2" r="2"/><circle cx="80.1" cy="72.2" r="2"/><circle cx="109.8" cy="72.2" r="2"/><circle cx="139.4" cy="72.2" r="2"/><circle cx="5.9" cy="97.8" r="2"/><circle cx="35.6" cy="97.8" r="2"/><circle cx="65.3" cy="97.8" r="2"/><circle cx="94.9" cy="97.8" r="2"/><circle cx="124.6" cy="97.8" r="2"/><circle cx="154.3" cy="97.8" r="2"/><circle cx="20.8" cy="123.5" r="2"/><circle cx="50.4" cy="123.5" r="2"/><circle cx="80.1" cy="123.5" r="2"/><circle cx="109.8" cy="123.5" r="2"/><circle cx="139.4" cy="123.5" r="2"/><circle cx="5.9" cy="149.2" r="2"/><circle cx="35.6" cy="149.2" r="2"/><circle cx="65.3" cy="149.2" r="2"/><circle cx="94.9" cy="149.2" r="2"/><circle cx="124.6" cy="149.2" r="2"/><circle cx="154.3" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="124.6,149.2 94.9,149.2 80.1,123.5 50.4,123.5 35.6,97.8 20.8,72.2 35.6,46.5 50.4,20.8 80.1,20.8 109.8,20.8 124.6,46.5 139.4,72.2 124.6,97.8 139.4,123.5 124.6,149.2 109.8,123.5 94.9,97.8 65.3,97.8 50.4,72.2 65.3,46.5 94.9,46.5 109.8,72.2 94.9,97.8 80.1,72.2 65.3,46.5 50.4,20.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:742;--dur:6.0s" points="124.6,149.2 94.9,149.2 80.1,123.5 50.4,123.5 35.6,97.8 20.8,72.2 35.6,46.5 50.4,20.8 80.1,20.8 109.8,20.8 124.6,46.5 139.4,72.2 124.6,97.8 139.4,123.5 124.6,149.2 109.8,123.5 94.9,97.8 65.3,97.8 50.4,72.2 65.3,46.5 94.9,46.5 109.8,72.2 94.9,97.8 80.1,72.2 65.3,46.5 50.4,20.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="124.6" y1="149.2" x2="115.1" y2="149.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="108.3,149.2 115.1,146.0 115.1,152.5" fill="#fc77be"/><circle cx="124.6" cy="149.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqeweweweweqedwqeeeeedww` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `text` ([[String]])
- **Leaves:** `null/number` ([[Number]], [[Null]])
- **ID:** `hexcassettes:foretell`

```text
before   … │ text         ◂ top
after    … │ null/number  ◂ top
```

## Description

Pushes the twentieths of a second until the **Cassette** with the given label fires. Pushes Null if there is no such **Cassette**.

## Related patterns (same book page)

- [[Enqueue]] `list, number → text`
- [[Dequeue]] `pattern → —`
- [[Disqueue]] `— → —`
- [[Threading Reflection]] `— → number`
- [[Operating Reflection]] `— → list of text iotas`
- [[Program Distillation]] `player, text → boolean`
- [[State Reflection]] `— → null/text`

## Sources

- 📖 **Guidebook:** [[Delaying Hexes]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpForetell` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcassettes:foretell`
