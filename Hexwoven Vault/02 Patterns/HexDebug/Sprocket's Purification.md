---
aliases: ["Sprocket's Purification"]
tags: [hexcasting/pattern, mod/hexdebug, family/purification, topic/debugging]
kind: "Pattern"
family: "Purification"
topics: ["Debugging"]
mod: "HexDebug"
op_id: "hexdebug:splicing/view_index/read"
signature: "wqaeaqwdwaqaw"
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

# Sprocket's Purification

◇ **Pattern** · [[HexDebug]] · 📖 [[Splicing Table Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="93.7,84.4 76.3,54.3 59.0,24.3 24.3,24.3 41.6,54.3 24.3,84.4 59.0,84.4 76.3,54.3 93.7,24.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3 111.0,54.3 93.7,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:486;--dur:3.9s" points="93.7,84.4 76.3,54.3 59.0,24.3 24.3,24.3 41.6,54.3 24.3,84.4 59.0,84.4 76.3,54.3 93.7,24.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3 111.0,54.3 93.7,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.7" y1="84.4" x2="88.1" y2="74.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.1,67.9 91.4,72.9 84.8,76.7" fill="#fc77be"/><circle cx="93.7" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaeaqwdwaqaw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexdebug:splicing/view_index/read`

```text
before   … │ vec  ◂ top
after    … │ num  ◂ top
```

## Description

Remove a vector from the [[The Stack|stack]], then push the index of the leftmost [[Iotas|iota]] currently visible in the [[Splicing Table]] at that position.

## Related patterns (same book page)

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
- [[Projectionist's Gambit II]] `vec, num → —`
- [[Shutter's Purification II]] `vec → bool`

## Sources

- 📖 **Guidebook:** [[Splicing Table Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sprocket's Purification](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/splicing_table@hexdebug:splicing/view_index/read)
- 🧩 **Mod page:** [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadViewIndex` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexdebug:splicing/view_index/read`
