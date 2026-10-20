---
aliases: ["Telepathic Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/text, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Text, Strings & Chat", "Sensing & Raycasts"]
mod: "Hexical"
op_id: "hexical:get_telepathy"
signature: "wqqadaw"
start_direction: "EAST"
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

# Telepathic Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Telepathy]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]] · [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5 95.9,68.3 74.1,30.5 52.3,68.3 30.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5 95.9,68.3 74.1,30.5 52.3,68.3 30.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="106.0" x2="44.5" y2="106.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="54.5,106.0 44.5,110.8 44.5,101.2" fill="#fc77be"/><circle cx="30.5" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqadaw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexical:get_telepathy`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

By focusing on a particular image, I can detect that concentration via this pattern. Pushes how many twentieths of a second I've been intending to hexical.telepathy, or -1 if I am not.

## Related patterns (same book page)

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
- [[Stealthy Reflection]] `— → number`
- [[Rolling Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Telepathy]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Telepathic Reflection](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/telepathy@hexical:get_telepathy)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetKeybind` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:get_telepathy`
