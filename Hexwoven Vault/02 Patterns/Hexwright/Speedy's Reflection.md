---
aliases: ["Speedy's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright, topic/movement]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic", "Movement"]
mod: "Hexwright"
op_id: "hexwright:speedys_reflection"
signature: "aqadd"
start_direction: "SOUTH_WEST"
input: ""
output: "[num]"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]", "[[List]]"]
entry: "[[Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Speedy's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[Patterns]] · Hex Wright › Flying Devices

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,85.0 38.0,132.0 92.3,132.0 119.4,85.0 65.1,85.0 92.3,38.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="65.1,85.0 38.0,132.0 92.3,132.0 119.4,85.0 65.1,85.0 92.3,38.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="85.0" x2="56.4" y2="100.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.2,110.9 51.3,97.1 61.6,103.0" fill="#fc77be"/><circle cx="65.1" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqadd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[num]` ([[Number]], [[List]])
- **ID:** `hexwright:speedys_reflection`

```text
before   …          ◂ top
after    … │ [num]  ◂ top
```

## Description

Your vehicle's three limits, these are flying device constants and won't change mid flight.
Free to cast

## Related patterns (same book page)

- [[Conveyance's Reflection]] `— → [state]`
- [[Locomotion's Reflection]] `— → num, num`
- [[Vessel's Dismissal]] `bool → —`
- [[Vessel's Reflection]] `— → entity`

## Sources

- 📖 **Guidebook:** [[Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `FlightActions$2` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:speedys_reflection`
