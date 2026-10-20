---
aliases: ["Baton Reflection"]
tags: [hexcasting/pattern, mod/oneironaut, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Oneironaut"
op_id: "oneironaut:getrodlook"
signature: "qwqqqwqawa"
start_direction: "SOUTH_EAST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Reverberation Rod]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Baton Reflection

◇ **Pattern** · [[Oneironaut]] · 📖 [[Reverberation Rod]] · Items

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 65.7,60.5 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 65.7,60.5 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,78.9 29.5,73.3 36.9,69.1" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwqqqwqawa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `oneironaut:getrodlook`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

When cast with a Reverberation Rod, returns my look vector from when I started the current cast loop.

## Related patterns (same book page)

- [[Conduct Rod]] `entity, list of patterns → —`
- [[Baton Reflection II]] `— → vector`
- [[Metronome Reflection]] `— → number`
- [[Metronome Gambit]] `number → —`
- [[Finale]] `— → —`
- [[Encore]] `number → —`
- [[Williams' Gambit]] `non-list → —`
- [[Williams' Reflection]] `— → non-list`
- [[Conductor's Reflection]] `— → bool`
- [[Williams' Gambit II]] `non-list → —`
- [[Williams' Reflection II]] `— → non-list`
- [[Conductor's Reflection II]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Reverberation Rod]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Baton Reflection](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#items/reverberation_rod@oneironaut:getrodlook)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetInitialRodState` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:getrodlook`
