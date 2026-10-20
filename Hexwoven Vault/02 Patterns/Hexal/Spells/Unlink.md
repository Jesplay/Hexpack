---
aliases: ["Unlink"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/wisps]
kind: "Spell"
family: "Spell"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:link/unlink"
signature: "qdeddqeeeeedwqdeddw"
start_direction: "WEST"
input: "num"
output: ""
cost: "costs two Amethyst Dust."
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Links]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Unlink

✦ **Spell** · [[Hexal]] · 📖 [[Links]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="72" viewBox="0 0 170.0 72.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.1" r="2"/><circle cx="27.6" cy="16.1" r="2"/><circle cx="50.5" cy="16.1" r="2"/><circle cx="73.5" cy="16.1" r="2"/><circle cx="96.5" cy="16.1" r="2"/><circle cx="119.5" cy="16.1" r="2"/><circle cx="142.4" cy="16.1" r="2"/><circle cx="165.4" cy="16.1" r="2"/><circle cx="16.1" cy="36.0" r="2"/><circle cx="39.1" cy="36.0" r="2"/><circle cx="62.0" cy="36.0" r="2"/><circle cx="85.0" cy="36.0" r="2"/><circle cx="108.0" cy="36.0" r="2"/><circle cx="130.9" cy="36.0" r="2"/><circle cx="153.9" cy="36.0" r="2"/><circle cx="4.6" cy="55.9" r="2"/><circle cx="27.6" cy="55.9" r="2"/><circle cx="50.5" cy="55.9" r="2"/><circle cx="73.5" cy="55.9" r="2"/><circle cx="96.5" cy="55.9" r="2"/><circle cx="119.5" cy="55.9" r="2"/><circle cx="142.4" cy="55.9" r="2"/><circle cx="165.4" cy="55.9" r="2"/></g><polyline class="hexpat-base" points="153.9,36.0 130.9,36.0 119.5,55.9 108.0,36.0 119.5,16.1 130.9,36.0 108.0,36.0 96.5,55.9 73.5,55.9 62.0,36.0 73.5,16.1 96.5,16.1 108.0,36.0 85.0,36.0 62.0,36.0 50.5,55.9 39.1,36.0 50.5,16.1 62.0,36.0 39.1,36.0 16.1,36.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:459;--dur:5.6s" points="153.9,36.0 130.9,36.0 119.5,55.9 108.0,36.0 119.5,16.1 130.9,36.0 108.0,36.0 96.5,55.9 73.5,55.9 62.0,36.0 73.5,16.1 96.5,16.1 108.0,36.0 85.0,36.0 62.0,36.0 50.5,55.9 39.1,36.0 50.5,16.1 62.0,36.0 39.1,36.0 16.1,36.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="153.9" y1="36.0" x2="146.6" y2="36.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="141.3,36.0 146.6,33.4 146.6,38.5" fill="#fc77be"/><circle cx="153.9" cy="36.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qdeddqeeeeedwqdeddw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `nothing` (—)
- **Cost:** costs two Amethyst Dust.
- **ID:** `hexal:link/unlink`

```text
before   … │ num  ◂ top
after    …        ◂ top
```

## Description

Removes the link at the given index from the caster's links. This costs two [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Link]] `linkable → —`
- [[Link Others]] `linkable, linkable → —`
- [[Unlink Others]] `linkable, linkable → —`
- [[Phonebook Purification]] `num → linkable`
- [[Recognition Purification]] `linkable → num`
- [[Popularity Reflection]] `— → num`
- [[Send Iota]] `num, any → —`
- [[Recitation Reflection]] `— → any`
- [[Postmaster's Reflection]] `— → num`
- [[Unclogging Gambit]] `— → —`
- [[Open Transmit]] `num → —`
- [[Close Transmit]] `— → —`

## Sources

- 📖 **Guidebook:** [[Links]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Unlink](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/links@hexal:link/unlink)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpUnlink` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:link/unlink`
