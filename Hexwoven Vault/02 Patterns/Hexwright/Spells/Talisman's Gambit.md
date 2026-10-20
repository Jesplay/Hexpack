---
aliases: ["Talisman's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:talismans_binding"
signature: "aqede"
start_direction: "NORTH_WEST"
input: "num, num"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Talismans]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Talisman's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[Talismans]] · Hex Wright › Casting Tools

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,66.6 72.3,29.8 51.0,66.6 72.3,103.4 51.0,140.2 29.8,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="93.5,66.6 72.3,29.8 51.0,66.6 72.3,103.4 51.0,140.2 29.8,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="66.6" x2="86.7" y2="54.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="81.9,46.3 90.8,52.5 82.7,57.1" fill="#fc77be"/><circle cx="93.5" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqede` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:talismans_binding`

```text
before   … │ num │ num  ◂ top
after    …              ◂ top
```

## Description

Sets the conditions of the Talisman. First number is the trigger, second is the context.
Costs 5 Dust

## Sources

- 📖 **Guidebook:** [[Talismans]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `TalismanActions$1` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:talismans_binding`
