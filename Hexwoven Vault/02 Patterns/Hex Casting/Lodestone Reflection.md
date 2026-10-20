---
aliases: ["Lodestone Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/circles]
kind: "Pattern"
family: "Reflection"
topics: ["Spell Circles, Loci & Engines"]
mod: "Hex Casting"
op_id: "hexcasting:circle/impetus_dir"
signature: "eaqwqaewede"
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

# Lodestone Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Spell Circle Patterns]] · Patterns

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,85.0 59.4,115.3 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 94.4,115.3 76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="76.9,85.0 59.4,115.3 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 94.4,115.3 76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="85.0" x2="71.3" y2="94.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,101.6 68.0,92.8 74.6,96.6" fill="#fc77be"/><circle cx="76.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaewede` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:circle/impetus_dir`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Returns the direction the [[Impeti|Impetus]] of this [[Spell Circles|spell circle]] is facing as a unit vector.

## Related patterns (same book page)

- [[Waystone Reflection]] `— → vector`
- [[Lesser Fold Reflection]] `— → vector`
- [[Greater Fold Reflection]] `— → vector`

## Sources

- 📖 **Guidebook:** [[Spell Circle Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Lodestone Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/circle@hexcasting:circle/impetus_dir)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpImpetusDir` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:circle/impetus_dir`
