---
aliases: ["Conveyance's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright, topic/movement]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic", "Movement"]
mod: "Hexwright"
op_id: "hexwright:conveyances_reflection"
signature: "aqaeede"
start_direction: "SOUTH_WEST"
input: ""
output: "[state]"
cost: ""
iotas_in: []
iotas_out: ["[[List]]"]
entry: "[[Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conveyance's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[Patterns]] · Hex Wright › Flying Devices

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,103.4 29.8,140.2 72.3,140.2 93.5,103.4 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:340;--dur:2.2s" points="51.0,103.4 29.8,140.2 72.3,140.2 93.5,103.4 51.0,103.4 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="103.4" x2="44.2" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,123.7 40.2,112.9 48.3,117.5" fill="#fc77be"/><circle cx="51.0" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaeede` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[state]` ([[List]])
- **ID:** `hexwright:conveyances_reflection`

```text
before   …            ◂ top
after    … │ [state]  ◂ top
```

## Description

Can only be cast by a broom or carpet. Returns everything the vehicle knows about this moment.
Free to cast

## Related patterns (same book page)

- [[Locomotion's Reflection]] `— → num, num`
- [[Speedy's Reflection]] `— → [num]`
- [[Vessel's Dismissal]] `bool → —`
- [[Vessel's Reflection]] `— → entity`

## Sources

- 📖 **Guidebook:** [[Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `FlightActions$1` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:conveyances_reflection`
