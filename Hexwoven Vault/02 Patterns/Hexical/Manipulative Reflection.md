---
aliases: ["Manipulative Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/text, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Text, Strings & Chat", "Sensing & Raycasts"]
mod: "Hexical"
op_id: "hexical:right_click"
signature: "edaqq"
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

# Manipulative Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Telepathy]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]] · [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,140.2 29.8,103.4 51.0,66.6 72.3,103.4 93.5,66.6 72.3,29.8 29.8,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="51.0,140.2 29.8,103.4 51.0,66.6 72.3,103.4 93.5,66.6 72.3,29.8 29.8,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="140.2" x2="44.2" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,120.0 48.3,126.1 40.2,130.8" fill="#fc77be"/><circle cx="51.0" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edaqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexical:right_click`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Pushes how many twentieths of a second I've been intending to Right-Click, or -1 if I am not.

## Related patterns (same book page)

- [[Telepathic Reflection]] `— → number`
- [[Send Thought]] `any → —`
- [[Shout Thought]] `any → —`
- [[Hallucinate Pling]] `— → —`
- [[Hallucinate Click]] `— → —`
- [[Offensive Reflection]] `— → number`
- [[Charge Reflection]] `— → number`
- [[Retreat Reflection]] `— → number`
- [[Dodge Reflection]] `— → number`
- [[Evade Reflection]] `— → number`
- [[Leaping Reflection]] `— → number`
- [[Stealthy Reflection]] `— → number`
- [[Rolling Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Telepathy]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Manipulative Reflection](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/telepathy@hexical:right_click)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetKeybind` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:right_click`
