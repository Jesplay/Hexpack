---
aliases: ["Williams' Reflection II"]
tags: [hexcasting/pattern, mod/oneironaut, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Oneironaut"
op_id: "oneironaut:readrodramremote"
signature: "qwaqawewewaqawewddw"
start_direction: "NORTH_EAST"
input: ""
output: "non-list"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Reverberation Rod]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Williams' Reflection II

◇ **Pattern** · [[Oneironaut]] · 📖 [[Reverberation Rod]] · Items

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="106" viewBox="0 0 170.0 106.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/><circle cx="18.6" cy="87.6" r="2"/><circle cx="45.2" cy="87.6" r="2"/><circle cx="71.7" cy="87.6" r="2"/><circle cx="98.3" cy="87.6" r="2"/><circle cx="124.8" cy="87.6" r="2"/><circle cx="151.4" cy="87.6" r="2"/></g><polyline class="hexpat-base" points="45.2,87.6 58.4,64.6 45.2,41.6 31.9,18.6 18.6,41.6 31.9,64.6 45.2,41.6 58.4,18.6 85.0,18.6 111.6,18.6 124.8,41.6 138.1,64.6 151.4,41.6 138.1,18.6 124.8,41.6 111.6,64.6 85.0,64.6 58.4,64.6 71.7,41.6 85.0,64.6 98.3,87.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:531;--dur:5.6s" points="45.2,87.6 58.4,64.6 45.2,41.6 31.9,18.6 18.6,41.6 31.9,64.6 45.2,41.6 58.4,18.6 85.0,18.6 111.6,18.6 124.8,41.6 138.1,64.6 151.4,41.6 138.1,18.6 124.8,41.6 111.6,64.6 85.0,64.6 58.4,64.6 71.7,41.6 85.0,64.6 98.3,87.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.2" y1="87.6" x2="49.4" y2="80.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.5,75.0 51.9,81.7 46.9,78.8" fill="#fc77be"/><circle cx="45.2" cy="87.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwaqawewewaqawewddw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `non-list` (—)
- **ID:** `oneironaut:readrodramremote`

```text
before   …             ◂ top
after    … │ non-list  ◂ top
```

## Description

As [[Williams' Reflection]], but can be cast from non-rod sources while a rod is in use.

## Related patterns (same book page)

- [[Conduct Rod]] `entity, list of patterns → —`
- [[Baton Reflection]] `— → vector`
- [[Baton Reflection II]] `— → vector`
- [[Metronome Reflection]] `— → number`
- [[Metronome Gambit]] `number → —`
- [[Finale]] `— → —`
- [[Encore]] `number → —`
- [[Williams' Gambit]] `non-list → —`
- [[Williams' Reflection]] `— → non-list`
- [[Conductor's Reflection]] `— → bool`
- [[Williams' Gambit II]] `non-list → —`
- [[Conductor's Reflection II]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Reverberation Rod]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Williams' Reflection II](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#items/reverberation_rod@oneironaut:readrodramremote)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpAccessRAMRemote` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:readrodramremote`
