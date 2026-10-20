---
aliases: ["Williams' Gambit II"]
tags: [hexcasting/pattern, mod/oneironaut, family/gambit, topic/devices]
kind: "Pattern"
family: "Gambit"
topics: ["Casting Devices"]
mod: "Oneironaut"
op_id: "oneironaut:writerodramremote"
signature: "ewdedwqwqwdedwqwaaw"
start_direction: "NORTH_WEST"
input: "non-list"
output: ""
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

# Williams' Gambit II

◇ **Pattern** · [[Oneironaut]] · 📖 [[Reverberation Rod]] · Items

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="106" viewBox="0 0 170.0 106.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/><circle cx="18.6" cy="87.6" r="2"/><circle cx="45.2" cy="87.6" r="2"/><circle cx="71.7" cy="87.6" r="2"/><circle cx="98.3" cy="87.6" r="2"/><circle cx="124.8" cy="87.6" r="2"/><circle cx="151.4" cy="87.6" r="2"/></g><polyline class="hexpat-base" points="124.8,87.6 111.6,64.6 124.8,41.6 138.1,18.6 151.4,41.6 138.1,64.6 124.8,41.6 111.6,18.6 85.0,18.6 58.4,18.6 45.2,41.6 31.9,64.6 18.6,41.6 31.9,18.6 45.2,41.6 58.4,64.6 85.0,64.6 111.6,64.6 98.3,41.6 85.0,64.6 71.7,87.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:531;--dur:5.6s" points="124.8,87.6 111.6,64.6 124.8,41.6 138.1,18.6 151.4,41.6 138.1,64.6 124.8,41.6 111.6,18.6 85.0,18.6 58.4,18.6 45.2,41.6 31.9,64.6 18.6,41.6 31.9,18.6 45.2,41.6 58.4,64.6 85.0,64.6 111.6,64.6 98.3,41.6 85.0,64.6 71.7,87.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="124.8" y1="87.6" x2="120.6" y2="80.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="117.5,75.0 123.1,78.8 118.1,81.7" fill="#fc77be"/><circle cx="124.8" cy="87.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewdedwqwqwdedwqwaaw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `non-list` (—)
- **Leaves:** `nothing` (—)
- **ID:** `oneironaut:writerodramremote`

```text
before   … │ non-list  ◂ top
after    …             ◂ top
```

## Description

As [[Williams' Gambit]], but can be cast from non-rod sources while a rod is in use.

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
- [[Williams' Reflection II]] `— → non-list`
- [[Conductor's Reflection II]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Reverberation Rod]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Williams' Gambit II](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#items/reverberation_rod@oneironaut:writerodramremote)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpAccessRAMRemote` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:writerodramremote`
