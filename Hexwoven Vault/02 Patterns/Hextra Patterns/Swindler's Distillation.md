---
aliases: ["Swindler's Distillation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/lists]
kind: "Pattern"
family: "Distillation"
topics: ["Lists"]
mod: "Hextra Patterns"
op_id: "hextrapats:swindle_list"
signature: "dqdeqaawddea"
start_direction: "WEST"
input: "list, num"
output: "list"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[List]]"]
entry: "[[Hextra Lists]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Swindler's Distillation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 95.9,68.3 139.5,68.3 117.7,30.5 95.9,68.3 74.1,106.0 52.3,68.3 95.9,68.3 117.7,106.0 139.5,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:3.6s" points="74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 95.9,68.3 139.5,68.3 117.7,30.5 95.9,68.3 74.1,106.0 52.3,68.3 95.9,68.3 117.7,106.0 139.5,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="74.1" y1="106.0" x2="60.2" y2="106.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.1,106.0 60.2,101.2 60.2,110.8" fill="#fc77be"/><circle cx="74.1" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqdeqaawddea` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, num` ([[Number]], [[List]])
- **Leaves:** `list` ([[List]])
- **ID:** `hextrapats:swindle_list`

```text
before   … │ list │ num  ◂ top
after    … │ list        ◂ top
```

## Description

Reorders the elements of a list the same way [[Swindler's Gambit]] reorders the [[The Stack|stack]]. Inputting a negative code will reorder the elements at the start instead of the end.

## Related patterns (same book page)

- [[Speaker's Decomposition II]] `list → list, any`
- [[Derivation Decomposition II]] `list → list, any`
- [[Viewer's Gambit]] `list, num → list, any`
- [[Separation Gambit]] `list, num → list, list`
- [[Elimination Gambit]] `list, any → list, bool`
- [[Elimination Gambit II]] `list, any → list, num`
- [[Shuffling Purification]] `list → list`
- [[Birdkeeper's Gambit]] `many → many, list`

## Sources

- 📖 **Guidebook:** [[Hextra Lists]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpListSwindle` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:swindle_list`
