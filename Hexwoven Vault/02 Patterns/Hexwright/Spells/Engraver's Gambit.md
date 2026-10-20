---
aliases: ["Engraver's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright, topic/devices]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic", "Casting Devices"]
mod: "Hexwright"
op_id: "hexwright:engravers_gambit"
signature: "deeeed"
start_direction: "EAST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Staff Cores]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Engraver's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[Staff Cores]] · Hex Wright › Casting Tools

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="85.0,78.3 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeed` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:engravers_gambit`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

[[Scribe's Gambit]] for staffs and weapons. Null erases it.
[[Scribe's Reflection]] still reads them.

## Related patterns (same book page)

- [[Wielder's Gambit]] `num → —`

## Sources

- 📖 **Guidebook:** [[Staff Cores]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `EngraverActions$1` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:engravers_gambit`
