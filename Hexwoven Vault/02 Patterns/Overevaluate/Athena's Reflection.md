---
aliases: ["Athena's Reflection"]
tags: [hexcasting/pattern, mod/overevaluate, family/reflection, topic/control-flow]
kind: "Pattern"
family: "Reflection"
topics: ["Control Flow & Meta-evaluation"]
mod: "Overevaluate"
op_id: "overevaluate:athena_log"
signature: "dweaqqqqd"
start_direction: "SOUTH_EAST"
input: ""
output: "garbage/text"
cost: ""
iotas_in: []
iotas_out: ["[[Garbage]]", "[[String]]"]
entry: "[[Advanced Metaevaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Athena's Reflection

◇ **Pattern** · [[Overevaluate]] · 📖 [[Advanced Metaevaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 123.6,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 123.6,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="27.0" x2="129.8" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="134.3,45.4 126.1,39.9 133.5,35.6" fill="#fc77be"/><circle cx="123.6" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dweaqqqqd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `garbage/text` ([[Garbage]], [[String]])
- **ID:** `overevaluate:athena_log`

```text
before   …                 ◂ top
after    … │ garbage/text  ◂ top
```

## Description

Pushes the last [[Mishaps|mishap]] caught by [[Athena's Gambit]] as *text* if the appropriate powers are active in this world.

## Related patterns (same book page)

- [[Athena's Gambit]] `list of patterns → boolean`
- [[Athena's Revelation]] `— → —`
- [[Apep's Gambit]] `pattern/list, list → any`
- [[Tutu's Gambit]] `— → —`
- [[Janus' Gambit]] `— → —`
- [[Ma'at's Gambit]] `boolean, any → —`
- [[Nephthys' Gambit]] `pattern/list → varies`
- [[Atalanta's Gambit]] `— → —`
- [[Castor's Gambit]] `any → —`
- [[Pollux's Gambit]] `any → —`
- [[Sisyphus' Gambit]] `list → —`
- [[Themis' Gambit]] `pattern/list, list → list`

## Sources

- 📖 **Guidebook:** [[Advanced Metaevaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Athena's Reflection](https://overevaluate.hexxy.media/v/latest/main/en_us/#metaevals@overevaluate:athena_log)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpAthenaLog` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:athena_log`
