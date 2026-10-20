---
aliases: ["Hierophant's Reflection"]
tags: [hexcasting/pattern, mod/hierophantics, family/reflection, topic/minds]
kind: "Pattern"
family: "Reflection"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:get_minds"
signature: "qaqqaeawaea"
start_direction: "NORTH_EAST"
input: ""
output: "[mind]"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Mind]]"]
entry: "[[Hierophantic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hierophant's Reflection

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Patterns]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6 85.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:600;--dur:3.4s" points="85.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6 85.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="121.6" x2="93.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="98.7,97.8 97.8,110.5 88.2,105.0" fill="#fc77be"/><circle cx="85.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqaeawaea` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[mind]` ([[List]], [[Mind]])
- **ID:** `hierophantics:get_minds`

```text
before   …           ◂ top
after    … │ [mind]  ◂ top
```

## Description

Pushes a list of [[Iotas|iotas]] representing all my embedded minds.

## Related patterns (same book page)

- [[Stabilize Minds]] `— → —`
- [[Payload Gambit]] `mind, [pattern] | null → —`
- [[Payload Purification]] `mind → [pattern] | null`
- [[Trigger Gambit]] `mind, trigger | null → —`
- [[Trigger Purification]] `mind → trigger | null`
- [[Mute Mind]] `mind, bool → —`
- [[Free Mind]] `mind → —`

## Sources

- 📖 **Guidebook:** [[Hierophantic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetMinds` is not a `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:get_minds`
