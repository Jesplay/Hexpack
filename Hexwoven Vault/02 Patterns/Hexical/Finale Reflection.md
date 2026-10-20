---
aliases: ["Finale Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:lamp_finale"
signature: "aaddaddad"
start_direction: "EAST"
input: ""
output: "boolean"
cost: ""
iotas_in: []
iotas_out: ["[[Boolean]]"]
entry: "[[Hand Lamps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Finale Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Hand Lamps]] · Genie Lamp

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,121.6 110.0,121.6 85.0,78.3 60.0,121.6 35.0,78.3 85.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:500;--dur:2.8s" points="60.0,121.6 110.0,121.6 85.0,78.3 60.0,121.6 35.0,78.3 85.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="121.6" x2="76.0" y2="121.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="87.5,121.6 76.0,127.1 76.0,116.1" fill="#fc77be"/><circle cx="60.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaddaddad` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexical:lamp_finale`

```text
before   …            ◂ top
after    … │ boolean  ◂ top
```

## Description

The instant I let go of my **Hand Lamp**, it casts an additional time. I can use this pattern to recognize whether a cast is that finale, and react accordingly.

## Related patterns (same book page)

- [[Wish]] `list of patterns → —`
- [[Genie Reflection Spatial]] `— → vector`
- [[Genie Reflection Rotational]] `— → vector`
- [[Genie Reflection Kinetic]] `— → vector`
- [[Genie Reflection Temporal]] `— → number`
- [[Genie Reflection Media]] `— → number`
- [[Genie Gambit]] `any → —`
- [[Genie Reflection Memory]] `— → any`

## Sources

- 📖 **Guidebook:** [[Hand Lamps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Finale Reflection](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#lamp/hand_lamps@hexical:lamp_finale)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetFinale` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:lamp_finale`
