---
aliases: ["Withdraw Media"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/devices]
kind: "Spell"
family: "Spell"
topics: ["Casting Devices"]
mod: "Lapisworks"
op_id: "lapisworks:withdraw"
signature: "qaqwwdwdw"
start_direction: "NORTH_EAST"
input: "M.T.I, num"
output: ""
cost: "Costs 10%% of the media withdrawn."
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Media Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Withdraw Media

✦ **Spell** · [[Lapisworks]] · 📖 [[Media Manipulation]] · Lapisworks

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3 94.4,145.5 59.4,145.5 24.5,145.5 41.9,115.3 59.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3 94.4,145.5 59.4,145.5 24.5,145.5 41.9,115.3 59.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="85.0" x2="65.0" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,68.4 68.3,77.2 61.7,73.4" fill="#fc77be"/><circle cx="59.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwwdwdw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `M.T.I, num` ([[Number]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 10%% of the media withdrawn.
- **ID:** `lapisworks:withdraw`

```text
before   … │ M.T.I │ num  ◂ top
after    …                ◂ top
```

## Description

Withdraws, from the given **[[Media|Media]] Transfer Interface**, into the **Media-containing item** in my other hand, as much of the dust specified as possible. Costs 10%% of the media withdrawn.

## Related patterns (same book page)

- [[Deposit Media]] `M.T.I, num → —`
- [[Inner Media Purification]] `M.T.I → num`

## Sources

- 📖 **Guidebook:** [[Media Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `Withdraw` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:withdraw`
