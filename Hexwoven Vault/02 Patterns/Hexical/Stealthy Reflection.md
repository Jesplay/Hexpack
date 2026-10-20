---
aliases: ["Stealthy Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/text, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Text, Strings & Chat", "Sensing & Raycasts"]
mod: "Hexical"
op_id: "hexical:sneaking"
signature: "wede"
start_direction: "NORTH_WEST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Telepathy]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Stealthy Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Telepathy]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]] · [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,140.2 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="72.3,140.2 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="140.2" x2="65.5" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,120.0 69.5,126.1 61.4,130.8" fill="#fc77be"/><circle cx="72.3" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wede` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexical:sneaking`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Pushes how many twentieths of a second I've been intending to Sneak, or -1 if I am not.

## Related patterns (same book page)

- [[Telepathic Reflection]] `— → number`
- [[Send Thought]] `any → —`
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
- [[Rolling Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Telepathy]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Stealthy Reflection](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/telepathy@hexical:sneaking)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetKeybind` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:sneaking`
