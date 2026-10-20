---
aliases: ["Derivation Decomposition II"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/decomposition, topic/lists]
kind: "Pattern"
family: "Decomposition"
topics: ["Lists"]
mod: "Hextra Patterns"
op_id: "hextrapats:peek/back"
signature: "qaeaqe"
start_direction: "NORTH_WEST"
input: "list"
output: "list, any"
cost: ""
iotas_in: ["[[List]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Hextra Lists]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Derivation Decomposition II

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="95.9,68.3 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 139.5,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="95.9,68.3 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 139.5,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="68.3" x2="88.9" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="83.9,47.5 93.1,53.8 84.8,58.6" fill="#fc77be"/><circle cx="95.9" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqe` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `list, any` ([[List]], [[Any Iota]])
- **ID:** `hextrapats:peek/back`

```text
before   … │ list        ◂ top
after    … │ list │ any  ◂ top
```

## Description

Gets the [[Iotas|iota]] at the end of the list without removing it from the list.

## Related patterns (same book page)

- [[Speaker's Decomposition II]] `list → list, any`
- [[Viewer's Gambit]] `list, num → list, any`
- [[Separation Gambit]] `list, num → list, list`
- [[Elimination Gambit]] `list, any → list, bool`
- [[Elimination Gambit II]] `list, any → list, num`
- [[Swindler's Distillation]] `list, num → list`
- [[Shuffling Purification]] `list → list`
- [[Birdkeeper's Gambit]] `many → many, list`

## Sources

- 📖 **Guidebook:** [[Hextra Lists]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPeek` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:peek/back`
