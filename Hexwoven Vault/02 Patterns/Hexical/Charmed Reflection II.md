---
aliases: ["Charmed Reflection II"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:read_charmed_proxy"
signature: "qaqwedeeaqqqqq"
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

# Charmed Reflection II

◇ **Pattern** · [[Hexical]] · 📖 [[Charm (Book Entry)]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3 59.4,145.5 41.9,115.3 59.4,85.0 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 76.9,115.3 94.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:524;--dur:4.2s" points="59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 76.9,115.3 59.4,145.5 41.9,115.3 59.4,85.0 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 76.9,115.3 94.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="85.0" x2="65.0" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,68.4 68.3,77.2 61.7,73.4" fill="#fc77be"/><circle cx="59.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwedeeaqqqqq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexical:read_charmed_proxy`

```text
before   …        ◂ top
after    … │ any  ◂ top
```

## Description

If the item charged features an [[Iotas|iota]] storage by default, such as a [[Focus]], reads an iota from it.

## Related patterns (same book page)

- [[Charm]] `entity, list, num, list of num, list of num → —`
- [[Charmed Gambit]] `any → —`
- [[Charmed Reflection]] `— → any`
- [[Charmed Gambit II]] `any → —`
- [[Discharm]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Charm (Book Entry)]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpProxyReadCharmed` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:read_charmed_proxy`
