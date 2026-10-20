---
aliases: ["Charmed Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:read_charmed"
signature: "waqqqqqeaqeaeaeaeaeq"
start_direction: "NORTH_EAST"
input: ""
output: "any"
cost: ""
iotas_in: []
iotas_out: ["[[Any Iota]]"]
entry: "[[Charm (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Charmed Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Charm (Book Entry)]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,115.3 76.9,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 111.8,24.5 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 111.8,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:734;--dur:5.9s" points="59.4,115.3 76.9,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 111.8,24.5 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 111.8,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="115.3" x2="65.0" y2="105.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,98.6 68.3,107.5 61.7,103.7" fill="#fc77be"/><circle cx="59.4" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqqqqqeaqeaeaeaeaeq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexical:read_charmed`

```text
before   …        ◂ top
after    … │ any  ◂ top
```

## Description

Pushes the stored [[Iotas|iota]] from the charmed item’s internal storage.

## Related patterns (same book page)

- [[Charm]] `entity, list, num, list of num, list of num → —`
- [[Charmed Gambit]] `any → —`
- [[Charmed Gambit II]] `any → —`
- [[Charmed Reflection II]] `— → any`
- [[Discharm]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Charm (Book Entry)]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadCharmed` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:read_charmed`
