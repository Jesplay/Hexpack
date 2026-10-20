---
aliases: ["Make Note"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hex Casting"
op_id: "hexcasting:beep"
signature: "adaa"
start_direction: "WEST"
input: "vector, number, number"
output: ""
cost: "Costs a negligible amount of media."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Basic Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Make Note

✦ **Spell** · [[Hex Casting]] · 📖 [[Basic Spells]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="146.5" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="119.4,38.0 65.1,38.0 92.3,85.0 38.0,85.0 65.1,132.0 92.3,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="119.4,38.0 65.1,38.0 92.3,85.0 38.0,85.0 65.1,132.0 92.3,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="119.4" y1="38.0" x2="102.0" y2="38.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="89.6,38.0 102.0,32.0 102.0,44.0" fill="#fc77be"/><circle cx="119.4" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adaa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a negligible amount of media.
- **ID:** `hexcasting:beep`

```text
before   … │ vector │ number │ number  ◂ top
after    …                             ◂ top
```

## Description

Remove a vector and two numbers from the [[The Stack|stack]]. Plays an **instrument** defined by the first number at the given location, with a **note** defined by the second number. Costs a negligible amount of **[[Media|media]]**.

There appear to be 16 different **instruments** and 25 different **notes**. Both are indexed by zero.

These seem to be the same instruments I can produce with a **Note Block**, though the reason for each instrument's number being what it is eludes me.

Either way, I can find the numbers I need to use by inspecting a **Note Block** through a [[Scrying Lens]].

## Related patterns (same book page)

- [[Explosion]] `vector, number → —`
- [[Fireball]] `vector, number → —`
- [[Impulse]] `entity, vector → —`
- [[Blink]] `entity, number → —`

## Sources

- 📖 **Guidebook:** [[Basic Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Make Note](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/basic@hexcasting:beep)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpBeep` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:beep`
