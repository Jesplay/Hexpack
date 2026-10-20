---
aliases: ["Trigger Purification"]
tags: [hexcasting/pattern, mod/hierophantics, family/purification, topic/minds]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:get_mind_trigger"
signature: "ddewaa"
start_direction: "SOUTH_EAST"
input: "mind"
output: "trigger | null"
cost: ""
iotas_in: ["[[Mind]]"]
iotas_out: ["[[Null]]", "[[Trigger]]"]
entry: "[[Hierophantic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Trigger Purification

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Patterns]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="52.3,68.3 74.1,106.0 30.5,106.0 52.3,68.3 95.9,68.3 139.5,68.3 117.7,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="52.3,68.3 74.1,106.0 30.5,106.0 52.3,68.3 95.9,68.3 139.5,68.3 117.7,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.3" y1="68.3" x2="59.3" y2="80.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="64.3,89.0 55.1,82.7 63.4,77.9" fill="#fc77be"/><circle cx="52.3" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddewaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mind` ([[Mind]])
- **Leaves:** `trigger | null` ([[Null]], [[Trigger]])
- **ID:** `hierophantics:get_mind_trigger`

## Description

Pushes the trigger stored in the provided mind, or **Null** if there isn't one.

## Related patterns (same book page)

- [[Hierophant's Reflection]] `— → [mind]`
- [[Stabilize Minds]] `— → —`
- [[Payload Gambit]] `mind, [pattern] | null → —`
- [[Payload Purification]] `mind → [pattern] | null`
- [[Trigger Gambit]] `mind, trigger | null → —`
- [[Mute Mind]] `mind, bool → —`
- [[Free Mind]] `mind → —`

## Sources

- 📖 **Guidebook:** [[Hierophantic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetMindTrigger` is not a `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:get_mind_trigger`
