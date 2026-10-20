---
aliases: ["Manifest Dust"]
tags: [hexcasting/spell, mod/hexwright, family/spell, topic/hexwright]
kind: "Spell"
family: "Spell"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:dust_manifest"
signature: "waa"
start_direction: "EAST"
input: "num"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Controlled Dust]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifest Dust

✦ **Spell** · [[Hexwright]] · 📖 [[Controlled Dust]] · Hex Wright › Abilities

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:1.6s" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="51.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,78.3 51.0,83.8 51.0,72.8" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:dust_manifest`

```text
before   … │ num  ◂ top
after    …        ◂ top
```

## Description

Spread dust into the air and seize control of it. Max amount is 500 amethyst dust.

Once dust is released, you never get it back, it's spent.

If you lower the dust amount, the effect is instant as control is dropped for the difference in dust, but if you increase the amount, it takes time to flow from you to the formation. So increasing the amount isn't a way to instantly patch a damaged formation.

## Related patterns (same book page)

- [[Direct Dust]] `vec → —`
- [[Form Dust]] `region → —`
- [[Impel Dust]] `vec → —`
- [[Move Dust]] `vector → —`
- [[Constrain Dust]] `bool → —`
- [[Recall Dust]] `— → —`
- [[Await Dust]] `[pattern] | pattern → —`

## Sources

- 📖 **Guidebook:** [[Controlled Dust]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `DustActions$1` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:dust_manifest`
