---
aliases: ["Dredging Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Hextra Patterns"
op_id: "hextrapats:dredging"
signature: "ddadaq"
start_direction: "WEST"
input: "any"
output: "any"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Hextra Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dredging Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="74.1,106.0 30.5,106.0 52.3,68.3 74.1,106.0 95.9,68.3 117.7,106.0 139.5,68.3 117.7,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="74.1,106.0 30.5,106.0 52.3,68.3 74.1,106.0 95.9,68.3 117.7,106.0 139.5,68.3 117.7,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="74.1" y1="106.0" x2="60.2" y2="106.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.1,106.0 60.2,101.2 60.2,110.8" fill="#fc77be"/><circle cx="74.1" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddadaq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hextrapats:dredging`

```text
before   … │ any  ◂ top
after    … │ any  ◂ top
```

## Description

Pulls the [[Iotas|iota]] at the bottom of the [[The Stack|stack]] to the top of the stack.

## Related patterns (same book page)

- [[Capsizing Gambit]] `any → any`
- [[Capsizing Gambit II]] `any → any`
- [[Dredging Gambit II]] `any → any`
- [[Angler's Gambit]] `many → many`

## Sources

- 📖 **Guidebook:** [[Hextra Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpFloat` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:dredging`
