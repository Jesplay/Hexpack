---
aliases: ["Projectionist's Gambit II"]
tags: [hexcasting/spell, mod/hexdebug, family/gambit, topic/debugging]
kind: "Spell"
family: "Gambit"
topics: ["Debugging"]
mod: "HexDebug"
op_id: "hexdebug:splicing/clipboard/spellbook_index/write"
signature: "wedqdewaqddqd"
start_direction: "SOUTH_WEST"
input: "vec, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Splicing Table Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Projectionist's Gambit II

✦ **Spell** · [[HexDebug]] · 📖 [[Splicing Table Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="104.3,27.0 85.0,60.5 65.7,94.0 27.0,94.0 46.4,60.5 27.0,27.0 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="104.3,27.0 85.0,60.5 65.7,94.0 27.0,94.0 46.4,60.5 27.0,27.0 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="27.0" x2="98.1" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,45.4 94.5,35.6 101.8,39.9" fill="#fc77be"/><circle cx="104.3" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wedqdewaqddqd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexdebug:splicing/clipboard/spellbook_index/write`

```text
before   … │ vec │ num  ◂ top
after    …              ◂ top
```

## Description

Like [[Projectionist's Gambit]], but the page number of the [[Spellbook]] in the secondary slot is written instead of the main slot.

## Related patterns (same book page)

- [[Sprocket's Purification]] `vec → num`
- [[Sprocket's Gambit]] `vec, num → —`
- [[Splicer's Decomposition]] `vec → num | null, num | null`
- [[Splicer's Gambit]] `vec, num | null, num | null → —`
- [[Compositor's Purification]] `vec → any`
- [[Compositor's Gambit]] `vec, any → —`
- [[Producer's Purification]] `vec → bool`
- [[Director's Purification]] `vec → bool`
- [[Projectionist's Purification]] `vec → num`
- [[Projectionist's Gambit]] `vec, num → —`
- [[Shutter's Purification]] `vec → bool`
- [[Projectionist's Purification II]] `vec → num`
- [[Shutter's Purification II]] `vec → bool`

## Sources

- 📖 **Guidebook:** [[Splicing Table Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Projectionist's Gambit II](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/splicing_table@hexdebug:splicing/clipboard/spellbook_index/write)
- 🧩 **Mod page:** [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpWriteSpellbookIndex` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexdebug:splicing/clipboard/spellbook_index/write`
