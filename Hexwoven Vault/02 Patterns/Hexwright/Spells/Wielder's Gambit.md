---
aliases: ["Wielder's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright, topic/devices]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic", "Casting Devices"]
mod: "Hexwright"
op_id: "hexwright:wielders_gambit"
signature: "wewqqwqwqqwaeaqa"
start_direction: "NORTH_EAST"
input: "num"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Staff Cores]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Wielder's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[Staff Cores]] · Hex Wright › Casting Tools

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 145.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="20.8,149.2 35.6,123.5 50.4,97.8 80.1,97.8 109.8,97.8 124.6,72.2 109.8,46.5 94.9,20.8 65.3,20.8 35.6,20.8 20.8,46.5 35.6,72.2 50.4,97.8 65.3,72.2 94.9,72.2 80.1,46.5 50.4,46.5 65.3,72.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.8s" points="20.8,149.2 35.6,123.5 50.4,97.8 80.1,97.8 109.8,97.8 124.6,72.2 109.8,46.5 94.9,20.8 65.3,20.8 35.6,20.8 20.8,46.5 35.6,72.2 50.4,97.8 65.3,72.2 94.9,72.2 80.1,46.5 50.4,46.5 65.3,72.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.8" y1="149.2" x2="25.5" y2="141.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="28.9,135.1 28.3,142.6 22.7,139.4" fill="#fc77be"/><circle cx="20.8" cy="149.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wewqqwqwqqwaeaqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:wielders_gambit`

```text
before   … │ num  ◂ top
after    …        ◂ top
```

## Description

Affects staffs with specific cores and has a different effect depending on the core.
[[Staves|Staff]] must be in hand.

## Related patterns (same book page)

- [[Engraver's Gambit]] `any → —`

## Sources

- 📖 **Guidebook:** [[Staff Cores]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `WielderActions$1` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:wielders_gambit`
