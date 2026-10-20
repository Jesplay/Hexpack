---
aliases: ["Arke's Gambit II"]
tags: [hexcasting/pattern, mod/caduceus, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Caduceus"
op_id: "caduceus:eval/control_at"
signature: "eqaedw"
start_direction: "NORTH_EAST"
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

# Arke's Gambit II

◇ **Pattern** · [[Caduceus]] · 📖 [[Delimited Jumps]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 93.5,103.4 114.8,66.6 72.3,66.6 51.0,29.8 93.5,29.8 136.1,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="29.8,140.2 51.0,103.4 93.5,103.4 114.8,66.6 72.3,66.6 51.0,29.8 93.5,29.8 136.1,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqaedw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern] | pattern, any` ([[List]], [[Pattern]], [[Any Iota]])
- **Leaves:** `many` (—)
- **ID:** `caduceus:eval/control_at`

## Description

Like [[Arke's Gambit]], but only patterns within an enclosing [[Thetis' Gambit II]] with a [[Jump Manipulation|jump tag]] matching the given [[Iotas|iota]] are captured. If the iota is Null, this is equivalent to [[Arke's Gambit]].

## Related patterns (same book page)

- [[Thetis' Gambit]] `[pattern] | pattern → many`
- [[Thetis' Gambit II]] `[pattern] | pattern, any → many`
- [[Arke's Gambit]] `[pattern] | pattern → many`

## Sources

- 📖 **Guidebook:** [[Delimited Jumps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Arke's Gambit II](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/delimcc@caduceus:eval/control_at)
- 🧩 **Mod page:** [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpControlAt` is not a `SpellAction`
- 🔧 **Pattern ID:** `caduceus:eval/control_at`
