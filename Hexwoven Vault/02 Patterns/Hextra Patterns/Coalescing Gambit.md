---
aliases: ["Coalescing Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/logic]
kind: "Pattern"
family: "Gambit"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:nulls/exec"
signature: "wawaadq"
start_direction: "SOUTH_EAST"
input: "any, [pattern] | pattern"
output: "any"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]", "[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Null Coercion]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Coalescing Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Null Coercion]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="35.0" x2="43.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,58.8 38.2,51.6 47.8,46.1" fill="#fc77be"/><circle cx="35.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawaadq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any, [pattern] | pattern` ([[List]], [[Pattern]], [[Any Iota]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hextrapats:nulls/exec`

## Description

If the first argument is not Null, returns it. Otherwise, executes the second argument, much like [[Hermes' Gambit]].

## Related patterns (same book page)

- [[Coalescing Distillation]] `any, any → any`

## Sources

- 📖 **Guidebook:** [[Null Coercion]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNullExecute` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:nulls/exec`
