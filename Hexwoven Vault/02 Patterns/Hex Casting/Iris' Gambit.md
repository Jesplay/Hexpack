---
aliases: ["Iris' Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:eval/cc"
signature: "qwaqde"
start_direction: "NORTH_WEST"
input: "evaluatable"
output: "many"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Iris' Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 95.9,68.3 74.1,106.0 30.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 95.9,68.3 74.1,106.0 30.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="68.3" x2="132.5" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,47.5 136.7,53.8 128.4,58.6" fill="#fc77be"/><circle cx="139.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwaqde` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `evaluatable` ([[List]], [[Pattern]])
- **Leaves:** `many` (—)
- **ID:** `hexcasting:eval/cc`

## Description

Cast something evaluatable from the [[The Stack|stack]] exactly like [[Hermes' Gambit]], except that a unique "Jump" [[Iotas|iota]] is pushed to the stack beforehand.

When the "Jump"-iota is executed, it'll jump directly to the end of the evaluatable, skipping any remaining actions.

While this may seem redundant given [[Charon's Gambit]] exists, this allows you to exit nested [[Hermes' Gambit|Hermes']] invocations in a controlled way, where Charon only allows you to exit one.

The "Jump" iota will apparently stay on the stack even after execution is finished... better not think about the implications of that.

## Example

Runs code like [[Hermes' Gambit]], but first pushes a **Jump** iota. Running that Jump later (with Hermes') skips straight to the end of this code, even from inside nested Hermes' calls. Think "return" for hexes; [[Charon's Gambit]] can only exit one level. More in [[Gambits Explained]].

## Related patterns (same book page)

- [[Hermes' Gambit]] `evaluatable → many`
- [[Thoth's Gambit]] `evaluatable, list → list`
- [[Charon's Gambit]] `— → —`
- [[Thanatos' Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Iris' Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/meta@hexcasting:eval/cc)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEvalBreakable` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:eval/cc`
