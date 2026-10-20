---
aliases: ["Recitation Reflection"]
tags: [hexcasting/pattern, mod/hexal, family/reflection, topic/wisps]
kind: "Pattern"
family: "Reflection"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:link/comm/read"
signature: "weeeeew"
start_direction: "NORTH_EAST"
input: ""
output: "any"
cost: ""
iotas_in: []
iotas_out: ["[[Any Iota]]"]
entry: "[[Links]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Recitation Reflection

◇ **Pattern** · [[Hexal]] · 📖 [[Links]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,106.0 52.3,68.3 74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0 74.1,106.0 52.3,68.3 30.5,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="30.5,106.0 52.3,68.3 74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0 74.1,106.0 52.3,68.3 30.5,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="106.0" x2="37.5" y2="93.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,85.3 41.6,96.3 33.3,91.5" fill="#fc77be"/><circle cx="30.5" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeeew` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexal:link/comm/read`

```text
before   …        ◂ top
after    … │ any  ◂ top
```

## Description

When something receives an [[Iotas|iota]] along a link it is added to a queue. This pattern adds the first iota in the queue to the [[The Stack|stack]], removing it from the queue.

## Related patterns (same book page)

- [[Link]] `linkable → —`
- [[Link Others]] `linkable, linkable → —`
- [[Unlink]] `num → —`
- [[Unlink Others]] `linkable, linkable → —`
- [[Phonebook Purification]] `num → linkable`
- [[Recognition Purification]] `linkable → num`
- [[Popularity Reflection]] `— → num`
- [[Send Iota]] `num, any → —`
- [[Postmaster's Reflection]] `— → num`
- [[Unclogging Gambit]] `— → —`
- [[Open Transmit]] `num → —`
- [[Close Transmit]] `— → —`

## Sources

- 📖 **Guidebook:** [[Links]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Recitation Reflection](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/links@hexal:link/comm/read)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadReceivedIota` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:link/comm/read`
