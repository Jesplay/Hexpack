---
aliases: ["Charmed Gambit II"]
tags: [hexcasting/spell, mod/hexical, family/gambit, topic/devices]
kind: "Spell"
family: "Gambit"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:write_charmed_proxy"
signature: "edewqaqqdeeeee"
start_direction: "SOUTH_EAST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Charm (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Charmed Gambit II

✦ **Spell** · [[Hexical]] · 📖 [[Charm (Book Entry)]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,85.0 76.9,115.3 59.4,145.5 41.9,115.3 59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 94.4,85.0 76.9,115.3 41.9,115.3 24.5,85.0 41.9,54.7 76.9,54.7 94.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:524;--dur:4.2s" points="59.4,85.0 76.9,115.3 59.4,145.5 41.9,115.3 59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 94.4,85.0 76.9,115.3 41.9,115.3 24.5,85.0 41.9,54.7 76.9,54.7 94.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="85.0" x2="65.0" y2="94.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,101.6 61.7,96.6 68.3,92.8" fill="#fc77be"/><circle cx="59.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edewqaqqdeeeee` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:write_charmed_proxy`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

If the item charged features an [[Iotas|iota]] storage by default, such as a [[Focus]], writes an iota into it.

## Related patterns (same book page)

- [[Charm]] `entity, list, num, list of num, list of num → —`
- [[Charmed Gambit]] `any → —`
- [[Charmed Reflection]] `— → any`
- [[Charmed Reflection II]] `— → any`
- [[Discharm]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Charm (Book Entry)]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpProxyWriteCharmed` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:write_charmed_proxy`
