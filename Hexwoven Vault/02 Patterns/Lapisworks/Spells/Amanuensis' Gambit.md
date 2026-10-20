---
aliases: ["Amanuensis' Gambit"]
tags: [hexcasting/spell, mod/lapisworks, family/gambit, topic/storage]
kind: "Spell"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Lapisworks"
op_id: "lapisworks:write_spechand"
signature: "deeeed"
start_direction: "EAST"
input: "any, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Specific Read-write]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Amanuensis' Gambit

✦ **Spell** · [[Lapisworks]] · 📖 [[Specific Read-write]] · Lapisworks

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="85.0,78.3 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeed` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any, num` ([[Number]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:write_spechand`

```text
before   … │ any │ num  ◂ top
after    …              ◂ top
```

## Description

Specific hand version of [[Scribe's Gambit]]. Writes the given [[Iotas|Iota]] to the Iota holder in the specified hand.

## Related patterns (same book page)

- [[Secretary's Purification]] `num → any`
- [[Reviewer's Purification]] `num → bool`
- [[Sanctioner's Purification]] `num → bool`

## Sources

- 📖 **Guidebook:** [[Specific Read-write]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `WriteToHand` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:write_spechand`
