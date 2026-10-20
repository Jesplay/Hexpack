---
aliases: ["Capsizing Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Hextra Patterns"
op_id: "hextrapats:capsizing"
signature: "ddadeq"
start_direction: "WEST"
input: "any"
output: "any"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Hextra Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Capsizing Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,66.6 29.8,66.6 51.0,29.8 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="72.3,66.6 29.8,66.6 51.0,29.8 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="66.6" x2="58.7" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.9,66.6 58.7,61.9 58.7,71.3" fill="#fc77be"/><circle cx="72.3" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddadeq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hextrapats:capsizing`

```text
before   … │ any  ◂ top
after    … │ any  ◂ top
```

## Description

Sends the [[Iotas|iota]] on the top of the [[The Stack|stack]] to the bottom of the stack.

## Related patterns (same book page)

- [[Capsizing Gambit II]] `any → any`
- [[Dredging Gambit]] `any → any`
- [[Dredging Gambit II]] `any → any`
- [[Angler's Gambit]] `many → many`

## Sources

- 📖 **Guidebook:** [[Hextra Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSink` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:capsizing`
