---
aliases: ["Infiltration Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/sentinels]
kind: "Pattern"
family: "Reflection"
topics: ["Sentinels"]
mod: "Hexical"
op_id: "hexical:get_lesser_sentinels"
signature: "dqddq"
start_direction: "WEST"
input: ""
output: "list of vectors"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]", "[[List]]"]
entry: "[[Lesser Sentinels]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Infiltration Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Lesser Sentinels]] · Patterns › Spells

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,132.0 38.0,132.0 65.1,85.0 38.0,38.0 92.3,38.0 65.1,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="92.3,132.0 38.0,132.0 65.1,85.0 38.0,38.0 92.3,38.0 65.1,85.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="132.0" x2="74.9" y2="132.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.4,132.0 74.9,126.0 74.9,138.0" fill="#fc77be"/><circle cx="92.3" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqddq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `list of vectors` ([[Vector]], [[List]])
- **ID:** `hexical:get_lesser_sentinels`

```text
before   …                    ◂ top
after    … │ list of vectors  ◂ top
```

## Description

Returns a list of vectors corresponding to the positions of all my **Lesser Sentinels**.

## Related patterns (same book page)

- [[Deploy Sentinels]] `list of vectors → —`

## Sources

- 📖 **Guidebook:** [[Lesser Sentinels]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Infiltration Reflection](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/lesser_sentinels@hexical:get_lesser_sentinels)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpLesserSentinelGet` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:get_lesser_sentinels`
