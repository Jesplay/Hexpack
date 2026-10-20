---
aliases: ["Marked Distillation"]
tags: [hexcasting/pattern, mod/hexal, family/distillation, topic/movement]
kind: "Pattern"
family: "Distillation"
topics: ["Movement"]
mod: "Hexal"
op_id: "hexal:gate/mark/get"
signature: "edwwdeeede"
start_direction: "EAST"
input: "gate, entity"
output: "bool"
cost: ""
iotas_in: ["[[Entity]]", "[[Gate]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Gates]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Marked Distillation

◇ **Pattern** · [[Hexal]] · 📖 [[Gates]] · Patterns › Great Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="97.4" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="106.2,27.0 97.4,31.3 97.4,22.8" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edwwdeeede` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `gate, entity` ([[Entity]], [[Gate]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexal:gate/mark/get`

```text
before   … │ gate │ entity  ◂ top
after    … │ bool           ◂ top
```

## Description

Consumes a gate [[Iotas|iota]] and an entity, returns whether that entity is marked by that gate.

## Related patterns (same book page)

- [[Gate's Reflection]] `null | vec | vec, entity → gate`
- [[Gate's Opening]] `gate, entity → —`
- [[Gate's Dismissal]] `gate, entity → —`
- [[Gate's Closing]] `gate | gate, vec → —`

## Sources

- 📖 **Guidebook:** [[Gates]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Marked Distillation](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/great_spells/gates@hexal:gate/mark/get)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetMarkedGate` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:gate/mark/get`
