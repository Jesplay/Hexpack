---
aliases: ["Compositor's Gambit"]
tags: [hexcasting/spell, mod/hexdebug, family/gambit, topic/debugging]
kind: "Spell"
family: "Gambit"
topics: ["Debugging"]
mod: "HexDebug"
op_id: "hexdebug:splicing/clipboard/write"
signature: "wedqdewqqqqaw"
start_direction: "SOUTH_WEST"
input: "vec, any"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Splicing Table Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Compositor's Gambit

✦ **Spell** · [[HexDebug]] · 📖 [[Splicing Table Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="93.7,24.3 76.3,54.3 59.0,84.4 24.3,84.4 41.6,54.3 24.3,24.3 59.0,24.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 111.0,54.3 128.4,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:486;--dur:3.9s" points="93.7,24.3 76.3,54.3 59.0,84.4 24.3,84.4 41.6,54.3 24.3,24.3 59.0,24.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 111.0,54.3 128.4,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.7" y1="24.3" x2="88.1" y2="33.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.1,40.8 84.8,32.0 91.4,35.8" fill="#fc77be"/><circle cx="93.7" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wedqdewqqqqaw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, any` ([[Vector]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexdebug:splicing/clipboard/write`

```text
before   … │ vec │ any  ◂ top
after    …              ◂ top
```

## Description

Remove a vector and [[Iotas|iota]] from the [[The Stack|stack]], then save the iota into the secondary item in the [[Splicing Table]] at that position.
As with [[Chronicler's Gambit]], I cannot write my own Name using this spell.

## Related patterns (same book page)

- [[Sprocket's Purification]] `vec → num`
- [[Sprocket's Gambit]] `vec, num → —`
- [[Splicer's Decomposition]] `vec → num | null, num | null`
- [[Splicer's Gambit]] `vec, num | null, num | null → —`
- [[Compositor's Purification]] `vec → any`
- [[Producer's Purification]] `vec → bool`
- [[Director's Purification]] `vec → bool`
- [[Projectionist's Purification]] `vec → num`
- [[Projectionist's Gambit]] `vec, num → —`
- [[Shutter's Purification]] `vec → bool`
- [[Projectionist's Purification II]] `vec → num`
- [[Projectionist's Gambit II]] `vec, num → —`
- [[Shutter's Purification II]] `vec → bool`

## Sources

- 📖 **Guidebook:** [[Splicing Table Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Compositor's Gambit](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/splicing_table@hexdebug:splicing/clipboard/write)
- 🧩 **Mod page:** [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpWriteClipboard` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexdebug:splicing/clipboard/write`
