---
aliases: ["Reflecting Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Overevaluate"
op_id: "overevaluate:swap_one_three"
signature: "ddwqaq"
start_direction: "NORTH_EAST"
input: "any, any, any"
output: "any, any, any"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Extra Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Reflecting Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Extra Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="95.9,68.3 117.7,30.5 139.5,68.3 95.9,68.3 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="95.9,68.3 117.7,30.5 139.5,68.3 95.9,68.3 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="68.3" x2="102.9" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.9,47.5 107.0,58.6 98.7,53.8" fill="#fc77be"/><circle cx="95.9" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddwqaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any, any, any` ([[Any Iota]])
- **Leaves:** `any, any, any` ([[Any Iota]])
- **ID:** `overevaluate:swap_one_three`

```text
before   … │ any │ any │ any  ◂ top
after    … │ any │ any │ any  ◂ top
```

## Description

Reflects the order of the top three [[Iotas|iota]], turning A, B, C into C, B, A. Essentially swaps the top and the third-from-the-top iota.

## Related patterns (same book page)

- [[Sekhmet's Gambit]] `— → varies`
- [[Geb's Gambit]] `many → many`
- [[Nut's Gambit]] `many → many`
- [[Bubbling Gambit]] `any, any, any → any, any, any`
- [[Dioscuri Gambit II]] `many, number → many`

## Sources

- 📖 **Guidebook:** [[Extra Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Reflecting Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#extra_stack@overevaluate:swap_one_three)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTwiddling` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:swap_one_three`
