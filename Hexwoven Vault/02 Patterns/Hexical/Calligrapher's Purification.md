---
aliases: ["Calligrapher's Purification"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/control-flow]
kind: "Pattern"
family: "Purification"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexical"
op_id: "hexical:deserialize_pattern"
signature: "wqqqaqwd"
start_direction: "EAST"
input: "list of numbers"
output: "pattern"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[Pattern]]"]
entry: "[[Pattern Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Calligrapher's Purification

◇ **Pattern** · [[Hexical]] · 📖 [[Pattern Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,84.4 59.0,84.4 93.7,84.4 111.0,54.3 93.7,24.3 59.0,24.3 76.3,54.3 111.0,54.3 145.7,54.3 128.4,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:312;--dur:2.5s" points="24.3,84.4 59.0,84.4 93.7,84.4 111.0,54.3 93.7,24.3 59.0,24.3 76.3,54.3 111.0,54.3 145.7,54.3 128.4,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="84.4" x2="35.4" y2="84.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,84.4 35.4,88.2 35.4,80.6" fill="#fc77be"/><circle cx="24.3" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqaqwd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list of numbers` ([[Number]], [[List]])
- **Leaves:** `pattern` ([[Pattern]])
- **ID:** `hexical:deserialize_pattern`

```text
before   … │ list of numbers  ◂ top
after    … │ pattern          ◂ top
```

## Description

Turns a list of numbers into a pattern for my viewing and execution. The inverse of [[Chirographer's Purification]].

The numbers returned by [[Chirographer's Purification]] correspond to individual strokes in the pattern. The diagonal stroke that travels upwards and rightwards is labeled 0 and all the other strokes are labelled incrementally in a clockwise manner.

If I pass in a list of numbers outside the 0-5 range into [[Calligrapher's Purification]], it will apply a [[Modulus Distillation|modulo]] to coerce numbers into the 0-5 range.

These lists have multiple fascinating properties. By adding or subtracting constant values, I can rotate the pattern. I can splice off the beginning and ends to check for pattern prefixes and suffixes. In cyclical patterns, I can rotate the list by moving the first element to the end or vice versa to cycle the starting position. By carefully altering numbers with list manipulation, I can also make some kind of *backwards stroke*.

## Related patterns (same book page)

- [[Glyphmaker's Distillation]] `pattern, number → pattern`
- [[Congruence Distillation]] `pattern, pattern → boolean`
- [[Similarity Distillation]] `pattern, pattern → boolean`
- [[Chirographer's Purification]] `pattern → list of numbers`
- [[Handwriting Distillation]] `pattern → list of vectors`

## Sources

- 📖 **Guidebook:** [[Pattern Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Calligrapher's Purification](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/pattern_manipulation@hexical:deserialize_pattern)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDeserializePattern` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:deserialize_pattern`
