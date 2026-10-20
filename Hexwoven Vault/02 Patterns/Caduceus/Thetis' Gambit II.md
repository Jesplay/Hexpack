---
aliases: ["Thetis' Gambit II"]
tags: [hexcasting/pattern, mod/caduceus, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Caduceus"
op_id: "caduceus:eval/prompt_at"
signature: "qedqaw"
start_direction: "NORTH_WEST"
input: "[pattern] | pattern, any"
output: "many"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Delimited Jumps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Thetis' Gambit II

◇ **Pattern** · [[Caduceus]] · 📖 [[Delimited Jumps]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="136.1,140.2 114.8,103.4 72.3,103.4 51.0,66.6 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="136.1,140.2 114.8,103.4 72.3,103.4 51.0,66.6 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="136.1" y1="140.2" x2="129.3" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="124.4,120.0 133.3,126.1 125.2,130.8" fill="#fc77be"/><circle cx="136.1" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qedqaw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern] | pattern, any` ([[List]], [[Pattern]], [[Any Iota]])
- **Leaves:** `many` (—)
- **ID:** `caduceus:eval/prompt_at`

## Description

Like [[Thetis' Gambit]], but also sets the [[Jump Manipulation|jump tag]] of the cast to the given [[Iotas|iota]]. If the iota is Null, this is equivalent to [[Thetis' Gambit]].

## Related patterns (same book page)

- [[Thetis' Gambit]] `[pattern] | pattern → many`
- [[Arke's Gambit]] `[pattern] | pattern → many`
- [[Arke's Gambit II]] `[pattern] | pattern, any → many`

## Sources

- 📖 **Guidebook:** [[Delimited Jumps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Thetis' Gambit II](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/delimcc@caduceus:eval/prompt_at)
- 🧩 **Mod page:** [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPromptAt` is not a `SpellAction`
- 🔧 **Pattern ID:** `caduceus:eval/prompt_at`
