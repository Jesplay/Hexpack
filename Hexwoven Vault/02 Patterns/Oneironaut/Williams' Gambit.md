---
aliases: ["Williams' Gambit"]
tags: [hexcasting/pattern, mod/oneironaut, family/gambit, topic/devices]
kind: "Pattern"
family: "Gambit"
topics: ["Casting Devices"]
mod: "Oneironaut"
op_id: "oneironaut:writerodram"
signature: "eqqwqqqwaaw"
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

# Williams' Gambit

◇ **Pattern** · [[Oneironaut]] · 📖 [[Reverberation Rod]] · Items

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="143.0,127.4 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 104.3,60.5 85.0,94.0 65.7,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="143.0,127.4 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 104.3,60.5 85.0,94.0 65.7,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="143.0" y1="127.4" x2="136.8" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="132.3,109.0 140.5,114.6 133.1,118.8" fill="#fc77be"/><circle cx="143.0" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqwqqqwaaw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `non-list` (—)
- **Leaves:** `nothing` (—)
- **ID:** `oneironaut:writerodram`

```text
before   … │ non-list  ◂ top
after    …             ◂ top
```

## Description

Accepts a non-list [[Iotas|iota]], and stores it inside the current cast loop. It will be lost when the cast loop ends.

## Related patterns (same book page)

- [[Conduct Rod]] `entity, list of patterns → —`
- [[Baton Reflection]] `— → vector`
- [[Baton Reflection II]] `— → vector`
- [[Metronome Reflection]] `— → number`
- [[Metronome Gambit]] `number → —`
- [[Finale]] `— → —`
- [[Encore]] `number → —`
- [[Williams' Reflection]] `— → non-list`
- [[Conductor's Reflection]] `— → bool`
- [[Williams' Gambit II]] `non-list → —`
- [[Williams' Reflection II]] `— → non-list`
- [[Conductor's Reflection II]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Reverberation Rod]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Williams' Gambit](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#items/reverberation_rod@oneironaut:writerodram)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpAccessRAM` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:writerodram`
