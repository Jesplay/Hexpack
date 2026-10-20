---
aliases: ["Shutter's Purification"]
tags: [hexcasting/pattern, mod/hexdebug, family/purification, topic/debugging]
kind: "Pattern"
family: "Purification"
topics: ["Debugging"]
mod: "HexDebug"
op_id: "hexdebug:splicing/list/spellbook_index/readable"
signature: "wqaeaqwedqddqw"
start_direction: "NORTH_WEST"
input: "vec"
output: "bool"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Splicing Table Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Shutter's Purification

◇ **Pattern** · [[HexDebug]] · 📖 [[Splicing Table Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="104.3,127.4 85.0,94.0 65.7,60.5 27.0,60.5 46.4,94.0 27.0,127.4 65.7,127.4 85.0,94.0 104.3,60.5 143.0,60.5 123.6,94.0 143.0,127.4 104.3,127.4 123.6,94.0 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:580;--dur:4.2s" points="104.3,127.4 85.0,94.0 65.7,60.5 27.0,60.5 46.4,94.0 27.0,127.4 65.7,127.4 85.0,94.0 104.3,60.5 143.0,60.5 123.6,94.0 143.0,127.4 104.3,127.4 123.6,94.0 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="127.4" x2="98.1" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,109.0 101.8,114.6 94.5,118.8" fill="#fc77be"/><circle cx="104.3" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaeaqwedqddqw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexdebug:splicing/list/spellbook_index/readable`

```text
before   … │ vec   ◂ top
after    … │ bool  ◂ top
```

## Description

Replace the vector at the top of the [[The Stack|stack]] with **True** if there is a [[Splicing Table]] or [[Focal Frame]] at that position with a [[Spellbook]] in its main slot that contains at least one page, or **False** otherwise.

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
- [[Projectionist's Purification II]] `vec → num`
- [[Projectionist's Gambit II]] `vec, num → —`
- [[Shutter's Purification II]] `vec → bool`

## Sources

- 📖 **Guidebook:** [[Splicing Table Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Shutter's Purification](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/splicing_table@hexdebug:splicing/list/spellbook_index/readable)
- 🧩 **Mod page:** [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadableSpellbookIndex` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexdebug:splicing/list/spellbook_index/readable`
