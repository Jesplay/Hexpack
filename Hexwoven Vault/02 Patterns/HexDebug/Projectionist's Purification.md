---
aliases: ["Projectionist's Purification"]
tags: [hexcasting/pattern, mod/hexdebug, family/purification, topic/debugging]
kind: "Pattern"
family: "Purification"
topics: ["Debugging"]
mod: "HexDebug"
op_id: "hexdebug:splicing/list/spellbook_index/read"
signature: "wqaeaqwedqddq"
start_direction: "NORTH_WEST"
input: "vec"
output: "num"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Number]]"]
entry: "[[Splicing Table Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Projectionist's Purification

◇ **Pattern** · [[HexDebug]] · 📖 [[Splicing Table Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="104.3,94.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0 123.6,60.5 104.3,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="104.3,94.0 85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 143.0,94.0 104.3,94.0 123.6,60.5 104.3,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="94.0" x2="98.1" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,75.6 101.8,81.1 94.5,85.4" fill="#fc77be"/><circle cx="104.3" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaeaqwedqddq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexdebug:splicing/list/spellbook_index/read`

```text
before   … │ vec  ◂ top
after    … │ num  ◂ top
```

## Description

Remove a vector from the [[The Stack|stack]], then push the current page number (starting at 1) of the [[Spellbook]] in the main slot of the [[Splicing Table]] or [[Focal Frame]] at that position.

## Related patterns (same book page)

- [[Sprocket's Purification]] `vec → num`
- [[Sprocket's Gambit]] `vec, num → —`
- [[Splicer's Decomposition]] `vec → num | null, num | null`
- [[Splicer's Gambit]] `vec, num | null, num | null → —`
- [[Compositor's Purification]] `vec → any`
- [[Compositor's Gambit]] `vec, any → —`
- [[Producer's Purification]] `vec → bool`
- [[Director's Purification]] `vec → bool`
- [[Projectionist's Gambit]] `vec, num → —`
- [[Shutter's Purification]] `vec → bool`
- [[Projectionist's Purification II]] `vec → num`
- [[Projectionist's Gambit II]] `vec, num → —`
- [[Shutter's Purification II]] `vec → bool`

## Sources

- 📖 **Guidebook:** [[Splicing Table Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Projectionist's Purification](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/splicing_table@hexdebug:splicing/list/spellbook_index/read)
- 🧩 **Mod page:** [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadSpellbookIndex` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexdebug:splicing/list/spellbook_index/read`
