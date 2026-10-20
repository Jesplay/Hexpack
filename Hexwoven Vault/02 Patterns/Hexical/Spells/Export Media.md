---
aliases: ["Export Media"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/circles]
kind: "Spell"
family: "Spell"
topics: ["Spell Circles, Loci & Engines"]
mod: "Hexical"
op_id: "hexical:create_dust"
signature: "eaqwedqdqddqqwae"
start_direction: "SOUTH_WEST"
input: "vector, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Circle Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Export Media

✦ **Spell** · [[Hexical]] · 📖 [[Circle Spells]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 76.9,85.0 111.8,85.0 129.3,115.3 94.4,115.3 76.9,145.5 59.4,115.3 24.5,115.3 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:594;--dur:4.8s" points="76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 76.9,85.0 111.8,85.0 129.3,115.3 94.4,115.3 76.9,145.5 59.4,115.3 24.5,115.3 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="24.5" x2="71.3" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,41.1 68.0,32.2 74.6,36.1" fill="#fc77be"/><circle cx="76.9" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwedqdqddqqwae` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:create_dust`

```text
before   … │ vector │ number  ◂ top
after    …                    ◂ top
```

## Description

Exports **[[Media|media]]** out of an [[Impeti|Impetus]] at a rate of about eleven [[Amethyst|Amethyst Dust]] spent per ten [[Amethyst|Amethyst Dust]] created. Takes a location and a number of up to a [[The Stack|stack]] of [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Displace]] `entity, vector → —`
- [[Appendage]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Circle Spells]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpCreateDust` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:create_dust`
