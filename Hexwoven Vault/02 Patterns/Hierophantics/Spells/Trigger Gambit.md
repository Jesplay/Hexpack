---
aliases: ["Trigger Gambit"]
tags: [hexcasting/spell, mod/hierophantics, family/gambit, topic/minds]
kind: "Spell"
family: "Gambit"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:set_mind_trigger"
signature: "ddewdd"
start_direction: "SOUTH_EAST"
input: "mind, trigger | null"
output: ""
cost: "Costs one Amethyst Shard."
iotas_in: ["[[Null]]", "[[Trigger]]", "[[Mind]]"]
iotas_out: []
entry: "[[Hierophantic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Trigger Gambit

✦ **Spell** · [[Hierophantics]] · 📖 [[Hierophantic Patterns]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/></g><polyline class="hexpat-base" points="52.3,30.5 74.1,68.3 30.5,68.3 52.3,30.5 95.9,30.5 139.5,30.5 117.7,68.3 95.9,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="52.3,30.5 74.1,68.3 30.5,68.3 52.3,30.5 95.9,30.5 139.5,30.5 117.7,68.3 95.9,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.3" y1="30.5" x2="59.3" y2="42.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="64.3,51.3 55.1,45.0 63.4,40.2" fill="#fc77be"/><circle cx="52.3" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddewdd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mind, trigger | null` ([[Null]], [[Trigger]], [[Mind]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one Amethyst Shard.
- **ID:** `hierophantics:set_mind_trigger`

## Description

Stores the provided trigger into the mind. If the mind already has a trigger, it will be overwritten. Costs one [[Amethyst|Amethyst Shard]].

## Related patterns (same book page)

- [[Hierophant's Reflection]] `— → [mind]`
- [[Stabilize Minds]] `— → —`
- [[Payload Gambit]] `mind, [pattern] | null → —`
- [[Payload Purification]] `mind → [pattern] | null`
- [[Trigger Purification]] `mind → trigger | null`
- [[Mute Mind]] `mind, bool → —`
- [[Free Mind]] `mind → —`

## Sources

- 📖 **Guidebook:** [[Hierophantic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSetMindTrigger` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:set_mind_trigger`
