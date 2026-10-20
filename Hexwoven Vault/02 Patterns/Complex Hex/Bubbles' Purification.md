---
aliases: ["Bubbles' Purification"]
tags: [hexcasting/pattern, mod/complex-hex, family/purification, topic/control-flow]
kind: "Pattern"
family: "Purification"
topics: ["Control Flow & Meta-evaluation"]
mod: "Complex Hex"
op_id: "complexhex:bubble"
signature: "qdqdqdqdqdq"
start_direction: "SOUTH_EAST"
input: "any"
output: "{any}"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Bubbles]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bubbles' Purification

◇ **Pattern** · [[Complex Hex]] · 📖 [[Bubbles]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,24.5 94.4,54.7 129.3,54.7 111.8,85.0 129.3,115.3 94.4,115.3 76.9,145.5 59.4,115.3 24.5,115.3 41.9,85.0 24.5,54.7 59.4,54.7 76.9,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="76.9,24.5 94.4,54.7 129.3,54.7 111.8,85.0 129.3,115.3 94.4,115.3 76.9,145.5 59.4,115.3 24.5,115.3 41.9,85.0 24.5,54.7 59.4,54.7 76.9,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="24.5" x2="82.5" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.5,41.1 79.2,36.1 85.8,32.2" fill="#fc77be"/><circle cx="76.9" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qdqdqdqdqdq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `{any}` ([[Any Iota]])
- **ID:** `complexhex:bubble`

```text
before   … │ any    ◂ top
after    … │ {any}  ◂ top
```

## Description

Pushes a Bubbled [[Iotas|Iota]], which when popped via execution (by Hermes' or the likes), will push the contained iota to the [[The Stack|stack]].

## Sources

- 📖 **Guidebook:** [[Bubbles]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Bubbles' Purification](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/bubble@complexhex:bubble)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpBubbleIota` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:bubble`
