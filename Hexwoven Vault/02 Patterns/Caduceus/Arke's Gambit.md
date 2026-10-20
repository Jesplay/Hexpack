---
aliases: ["Arke's Gambit"]
tags: [hexcasting/pattern, mod/caduceus, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Caduceus"
op_id: "caduceus:eval/control"
signature: "waqdeq"
start_direction: "WEST"
input: "[pattern] | pattern"
output: "many"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Delimited Jumps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Arke's Gambit

◇ **Pattern** · [[Caduceus]] · 📖 [[Delimited Jumps]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="136.1,29.8 93.5,29.8 51.0,29.8 72.3,66.6 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="136.1,29.8 93.5,29.8 51.0,29.8 72.3,66.6 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="136.1" y1="29.8" x2="122.5" y2="29.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.7,29.8 122.5,25.1 122.5,34.4" fill="#fc77be"/><circle cx="136.1" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqdeq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern] | pattern` ([[List]], [[Pattern]])
- **Leaves:** `many` (—)
- **ID:** `caduceus:eval/control`

## Description

Cast a pattern or list of patterns from the [[The Stack|stack]] similarly to [[Iris' Gambit]]. Must be drawn within [[Thetis' Gambit]].

All of the patterns that would have been drawn between the end of the pattern list and the end of the enclosing [[Thetis' Gambit]] are moved into a Call [[Iotas|iota]], which is pushed to the stack instead of a Jump iota.

When the Call iota is executed, the captured patterns will be cast as if by [[Hermes' Gambit]]. If the Call iota is not executed, the rest of the patterns within the enclosing [[Thetis' Gambit]] will be skipped.

## Related patterns (same book page)

- [[Thetis' Gambit]] `[pattern] | pattern → many`
- [[Thetis' Gambit II]] `[pattern] | pattern, any → many`
- [[Arke's Gambit II]] `[pattern] | pattern, any → many`

## Sources

- 📖 **Guidebook:** [[Delimited Jumps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Arke's Gambit](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/delimcc@caduceus:eval/control)
- 🧩 **Mod page:** [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpControl` is not a `SpellAction`
- 🔧 **Pattern ID:** `caduceus:eval/control`
