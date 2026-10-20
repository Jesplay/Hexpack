---
aliases: ["Postman's Gambit"]
tags: [hexcasting/spell, mod/ephemera, family/gambit, topic/text]
kind: "Spell"
family: "Gambit"
topics: ["Text, Strings & Chat"]
mod: "Ephemera"
op_id: "ephemera:revealtoother"
signature: "qde"
start_direction: "EAST"
input: "Player, any"
output: ""
cost: ""
iotas_in: ["[[Entity]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Messaging]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Postman's Gambit

✦ **Spell** · [[Ephemera]] · 📖 [[Messaging]] · Patterns › Spells

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:1.6s" points="35.0,78.3 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="51.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,78.3 51.0,83.8 51.0,72.8" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qde` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `Player, any` ([[Entity]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `ephemera:revealtoother`

```text
before   … │ Player │ any  ◂ top
after    …                 ◂ top
```

## Description

Accepts a player and any [[Iotas|iota]] less than 256 characters long, and shows that iota to them as if they had cast Reveal, also telling them who sent the message. Cost is inversely proportional to the length of time since they were last targeted by this spell.

## Related patterns (same book page)

- [[Postage Purification]] `Player → num`
- [[Postman's Gambit II]] `— → —`

## Sources

- 📖 **Guidebook:** [[Messaging]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Postman's Gambit](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/reveal_other@ephemera:revealtoother)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPrintToOther` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `ephemera:revealtoother`
