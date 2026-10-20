---
aliases: ["Gate's Dismissal"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Movement"]
mod: "Hexal"
op_id: "hexal:gate/unmark"
signature: "edeqqaq"
start_direction: "EAST"
input: "gate, entity"
output: ""
cost: ""
iotas_in: ["[[Entity]]", "[[Gate]]"]
iotas_out: []
entry: "[[Gates]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Gate's Dismissal

✦ **Spell** · [[Hexal]] · 📖 [[Gates]] · Patterns › Great Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 65.7,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 65.7,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="97.4" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="106.2,27.0 97.4,31.3 97.4,22.8" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeqqaq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `gate, entity` ([[Entity]], [[Gate]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:gate/unmark`

```text
before   … │ gate │ entity  ◂ top
after    …                  ◂ top
```

## Description

Consumes a gate [[Iotas|iota]] and an entity. Unmarks that entity so that if it was marked with [[Gate's Opening]] it will no longer be teleported the next time that gate is closed.

## Related patterns (same book page)

- [[Gate's Reflection]] `null | vec | vec, entity → gate`
- [[Gate's Opening]] `gate, entity → —`
- [[Marked Distillation]] `gate, entity → bool`
- [[Gate's Closing]] `gate | gate, vec → —`

## Sources

- 📖 **Guidebook:** [[Gates]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Gate's Dismissal](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/great_spells/gates@hexal:gate/unmark)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpUnmarkGate` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:gate/unmark`
