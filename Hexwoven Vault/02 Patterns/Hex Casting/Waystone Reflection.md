---
aliases: ["Waystone Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/circles]
kind: "Pattern"
family: "Reflection"
topics: ["Spell Circles, Loci & Engines"]
mod: "Hex Casting"
op_id: "hexcasting:circle/impetus_pos"
signature: "eaqwqae"
start_direction: "SOUTH_WEST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Spell Circle Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Waystone Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Spell Circle Patterns]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="85.0,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="78.8" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,45.4 75.1,35.6 82.5,39.9" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqae` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:circle/impetus_pos`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Returns the position of the [[Impeti|Impetus]] of this [[Spell Circles|spell circle]].

## Related patterns (same book page)

- [[Lodestone Reflection]] `— → vector`
- [[Lesser Fold Reflection]] `— → vector`
- [[Greater Fold Reflection]] `— → vector`

## Sources

- 📖 **Guidebook:** [[Spell Circle Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Waystone Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/circle@hexcasting:circle/impetus_pos)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpImpetusPos` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:circle/impetus_pos`
