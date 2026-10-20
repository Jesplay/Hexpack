---
aliases: ["Williams' Reflection"]
tags: [hexcasting/pattern, mod/oneironaut, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Oneironaut"
op_id: "oneironaut:readrodram"
signature: "qeeweeewddw"
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

# Williams' Reflection

◇ **Pattern** · [[Oneironaut]] · 📖 [[Reverberation Rod]] · Items

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,127.4 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 65.7,60.5 85.0,94.0 104.3,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="27.0,127.4 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 65.7,60.5 85.0,94.0 104.3,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="127.4" x2="33.2" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,109.0 36.9,118.8 29.5,114.6" fill="#fc77be"/><circle cx="27.0" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeeweeewddw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `non-list` (—)
- **ID:** `oneironaut:readrodram`

```text
before   …             ◂ top
after    … │ non-list  ◂ top
```

## Description

Returns the [[Iotas|iota]] stored in the current cast loop. Defaults to NULL.

## Related patterns (same book page)

- [[Conduct Rod]] `entity, list of patterns → —`
- [[Baton Reflection]] `— → vector`
- [[Baton Reflection II]] `— → vector`
- [[Metronome Reflection]] `— → number`
- [[Metronome Gambit]] `number → —`
- [[Finale]] `— → —`
- [[Encore]] `number → —`
- [[Williams' Gambit]] `non-list → —`
- [[Conductor's Reflection]] `— → bool`
- [[Williams' Gambit II]] `non-list → —`
- [[Williams' Reflection II]] `— → non-list`
- [[Conductor's Reflection II]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Reverberation Rod]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Williams' Reflection](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#items/reverberation_rod@oneironaut:readrodram)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpAccessRAM` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:readrodram`
