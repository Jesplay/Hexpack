---
aliases: ["Speaker's Decomposition II"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/decomposition, topic/lists]
kind: "Pattern"
family: "Decomposition"
topics: ["Lists"]
mod: "Hextra Patterns"
op_id: "hextrapats:peek/front"
signature: "aaqwqaaq"
start_direction: "SOUTH_WEST"
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

# Speaker's Decomposition II

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="79" viewBox="0 0 170.0 78.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/></g><polyline class="hexpat-base" points="128.4,24.3 111.0,54.3 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3 41.6,54.3 76.3,54.3 59.0,24.3 24.3,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:312;--dur:2.5s" points="128.4,24.3 111.0,54.3 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3 41.6,54.3 76.3,54.3 59.0,24.3 24.3,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="128.4" y1="24.3" x2="122.8" y2="33.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="118.8,40.8 119.5,32.0 126.1,35.8" fill="#fc77be"/><circle cx="128.4" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqwqaaq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `list, any` ([[List]], [[Any Iota]])
- **ID:** `hextrapats:peek/front`

```text
before   … │ list        ◂ top
after    … │ list │ any  ◂ top
```

## Description

Gets the [[Iotas|iota]] at the front of the list without removing it from the list.

## Related patterns (same book page)

- [[Derivation Decomposition II]] `list → list, any`
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
- 🔧 **Pattern ID:** `hextrapats:peek/front`
