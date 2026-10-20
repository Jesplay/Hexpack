---
aliases: ["Payload Purification"]
tags: [hexcasting/pattern, mod/hierophantics, family/purification, topic/minds]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:get_mind_hex"
signature: "ddeqaa"
start_direction: "SOUTH_EAST"
input: "mind"
output: "[pattern] | null"
cost: ""
iotas_in: ["[[Mind]]"]
iotas_out: ["[[List]]", "[[Pattern]]", "[[Null]]"]
entry: "[[Hierophantic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Payload Purification

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Patterns]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,78.3 85.0,121.6 35.0,121.6 60.0,78.3 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="60.0,78.3 85.0,121.6 35.0,121.6 60.0,78.3 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="78.3" x2="68.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="73.7,102.1 63.2,94.9 72.8,89.4" fill="#fc77be"/><circle cx="60.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddeqaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mind` ([[Mind]])
- **Leaves:** `[pattern] | null` ([[List]], [[Pattern]], [[Null]])
- **ID:** `hierophantics:get_mind_hex`

## Description

Pushes the **Hex** stored in the provided mind, or **Null** if there isn't one.

## Related patterns (same book page)

- [[Hierophant's Reflection]] `— → [mind]`
- [[Stabilize Minds]] `— → —`
- [[Payload Gambit]] `mind, [pattern] | null → —`
- [[Trigger Gambit]] `mind, trigger | null → —`
- [[Trigger Purification]] `mind → trigger | null`
- [[Mute Mind]] `mind, bool → —`
- [[Free Mind]] `mind → —`

## Sources

- 📖 **Guidebook:** [[Hierophantic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetMindHex` is not a `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:get_mind_hex`
