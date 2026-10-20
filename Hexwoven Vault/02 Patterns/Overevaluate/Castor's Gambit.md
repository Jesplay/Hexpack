---
aliases: ["Castor's Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Overevaluate"
op_id: "overevaluate:castor"
signature: "adadee"
start_direction: "NORTH_WEST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Advanced Metaevaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Castor's Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Advanced Metaevaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,106.0 117.7,68.3 95.9,106.0 74.1,68.3 52.3,106.0 30.5,68.3 52.3,30.5 95.9,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="139.5,106.0 117.7,68.3 95.9,106.0 74.1,68.3 52.3,106.0 30.5,68.3 52.3,30.5 95.9,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="106.0" x2="132.5" y2="93.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,85.3 136.7,91.5 128.4,96.3" fill="#fc77be"/><circle cx="139.5" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adadee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `overevaluate:castor`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

Should only be run within a [[Thoth's Gambit|Thoth's]]. Details on the next page. Beware of triggering the Delve Too Deep [[Mishaps|mishap]].

This pattern pops an [[Iotas|iota]] and schedules a new iteration of [[Thoth's Gambit|Thoth's]] to come right after the current one, with the popped iota being the top of the [[The Stack|stack]]. It may be helpful to imagine it as the iota being added to the front of the list that I provided for [[Thoth's Gambit|Thoth's]] to iterate over.

*Do not go where the path may lead, go instead where there is no path... and leave a trail.*

## Related patterns (same book page)

- [[Athena's Gambit]] `list of patterns → boolean`
- [[Athena's Revelation]] `— → —`
- [[Athena's Reflection]] `— → garbage/text`
- [[Apep's Gambit]] `pattern/list, list → any`
- [[Tutu's Gambit]] `— → —`
- [[Janus' Gambit]] `— → —`
- [[Ma'at's Gambit]] `boolean, any → —`
- [[Nephthys' Gambit]] `pattern/list → varies`
- [[Atalanta's Gambit]] `— → —`
- [[Pollux's Gambit]] `any → —`
- [[Sisyphus' Gambit]] `list → —`
- [[Themis' Gambit]] `pattern/list, list → list`

## Sources

- 📖 **Guidebook:** [[Advanced Metaevaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Castor's Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#metaevals@overevaluate:castor)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCastor` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:castor`
