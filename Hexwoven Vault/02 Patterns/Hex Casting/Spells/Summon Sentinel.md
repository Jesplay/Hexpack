---
aliases: ["Summon Sentinel"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/sentinels]
kind: "Spell"
family: "Spell"
topics: ["Sentinels"]
mod: "Hex Casting"
op_id: "hexcasting:sentinel/create"
signature: "waeawae"
start_direction: "EAST"
input: "vector"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Sentinels]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Summon Sentinel

✦ **Spell** · [[Hex Casting]] · 📖 [[Sentinels]] · Patterns › Spells

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,121.6 85.0,121.6 135.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="35.0,121.6 85.0,121.6 135.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="121.6" x2="51.0" y2="121.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,121.6 51.0,127.1 51.0,116.1" fill="#fc77be"/><circle cx="35.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waeawae` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexcasting:sentinel/create`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Summons my [[Sentinels|sentinel]] at the given position. Costs about one [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Banish Sentinel]] `— → —`
- [[Locate Sentinel]] `— → vector`
- [[Wayfind Sentinel]] `vector → vector`

## Sources

- 📖 **Guidebook:** [[Sentinels]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Summon Sentinel](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/sentinels@hexcasting:sentinel/create)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpCreateSentinel` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:sentinel/create`
