---
aliases: ["Alter Scale"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/effects]
kind: "Spell"
family: "Spell"
topics: ["Status Effects"]
mod: "Hex Casting"
op_id: "hexcasting:interop/pehkui/set"
signature: "ddwdwwdwwd"
start_direction: "NORTH_EAST"
input: "entity, num"
output: ""
cost: "Costs about 1 Amethyst Shard."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Pehkui]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Alter Scale

✦ **Spell** · [[Hex Casting]] · 📖 [[Pehkui]] · Cross-Mod Compatibility

**Topics:** [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,127.4 85.0,94.0 104.3,127.4 65.7,127.4 27.0,127.4 46.4,94.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,94.0 143.0,127.4 104.3,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="65.7,127.4 85.0,94.0 104.3,127.4 65.7,127.4 27.0,127.4 46.4,94.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,94.0 143.0,127.4 104.3,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="127.4" x2="71.9" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,109.0 75.5,118.8 68.2,114.6" fill="#fc77be"/><circle cx="65.7" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddwdwwdwwd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, num` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about 1 Amethyst Shard.
- **ID:** `hexcasting:interop/pehkui/set`

```text
before   … │ entity │ num  ◂ top
after    …                 ◂ top
```

## Description

Set the scale of the entity, passing in a proportion of their normal size. Costs about 1 **Amethyst Shard**.

## Related patterns (same book page)

- [[Gulliver's Purification]] `entity → num`

## Sources

- 📖 **Guidebook:** [[Pehkui]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Alter Scale](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#interop/pehkui@hexcasting:interop/pehkui/set)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSetScale` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:interop/pehkui/set`
