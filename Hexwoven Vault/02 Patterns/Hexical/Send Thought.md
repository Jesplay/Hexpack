---
aliases: ["Send Thought"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/text, topic/sensing]
kind: "Pattern"
family: "Other"
topics: ["Text, Strings & Chat", "Sensing & Raycasts"]
mod: "Hexical"
op_id: "hexical:send_telepathy"
signature: "qqqqwaqa"
start_direction: "EAST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Telepathy]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Send Thought

◇ **Pattern** · [[Hexical]] · 📖 [[Telepathy]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]] · [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5 74.1,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 52.3,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:392;--dur:2.5s" points="74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5 74.1,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 52.3,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="74.1" y1="106.0" x2="88.1" y2="106.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="98.1,106.0 88.1,110.8 88.1,101.2" fill="#fc77be"/><circle cx="74.1" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqwaqa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:send_telepathy`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

Momentarily displays an [[Iotas|iota]] above my hotbar. If cast repeatedly, each cast overwrites the previous instantly.

## Related patterns (same book page)

- [[Telepathic Reflection]] `— → number`
- [[Shout Thought]] `any → —`
- [[Hallucinate Pling]] `— → —`
- [[Hallucinate Click]] `— → —`
- [[Offensive Reflection]] `— → number`
- [[Manipulative Reflection]] `— → number`
- [[Charge Reflection]] `— → number`
- [[Retreat Reflection]] `— → number`
- [[Dodge Reflection]] `— → number`
- [[Evade Reflection]] `— → number`
- [[Leaping Reflection]] `— → number`
- [[Stealthy Reflection]] `— → number`
- [[Rolling Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Telepathy]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Send Thought](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/telepathy@hexical:send_telepathy)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSendTelepathy` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:send_telepathy`
