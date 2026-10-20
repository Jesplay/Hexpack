---
aliases: ["Refuel Lamp"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/devices]
kind: "Spell"
family: "Spell"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:recharge_lamp"
signature: "qaqwawqwqqwqwqwqwqwqq"
start_direction: "EAST"
input: "number"
output: ""
cost: "Costs just the media I'm giving the Lamp."
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[On Genies]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Refuel Lamp

✦ **Spell** · [[Hexical]] · 📖 [[On Genies]] · Genie Lamp

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="53.5,76.6 85.0,76.6 100.7,49.3 69.3,49.3 53.5,76.6 37.8,103.8 69.3,103.8 100.7,103.8 116.5,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="53.5,76.6 85.0,76.6 100.7,49.3 69.3,49.3 53.5,76.6 37.8,103.8 69.3,103.8 100.7,103.8 116.5,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="76.6" x2="63.6" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.8,76.6 63.6,80.0 63.6,73.1" fill="#fc77be"/><circle cx="53.5" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwawqwqqwqwqwqwqwqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number` ([[Number]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs just the media I'm giving the Lamp.
- **ID:** `hexical:recharge_lamp`

```text
before   … │ number  ◂ top
after    …           ◂ top
```

## Description

Refuels the **Lamp** in my offhand by the given amount in [[Amethyst|Amethyst Dust]]. Costs just the **[[Media|media]]** I'm giving the **Lamp**.

The **Lamp** seems capable of holding a near endless amount of **media**. If I fuel it past its current bounds, it simply expands its storage reservior to accommodate. Truly a marvel of technology.

Upon closer inspiration, the genie within the **Lamp** seems to just be an extremely minimal, almost artificial network of thought and **media**. I hesitate to call it a creature, more of a *wisp*. Perhaps if I replaced it with a more complex, *sentient* mind, I can induce some great change. The mind must be of upmost quality to survive the process and still retain a shred of talent; a villager that has spent its life honing a craft to perfection will do quite nicely.

## Related patterns (same book page)

- [[Promote Lamp]] `entity → —`

## Sources

- 📖 **Guidebook:** [[On Genies]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Refuel Lamp](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#lamp/genie_lore@hexical:recharge_lamp)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpRechargeLamp` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:recharge_lamp`
