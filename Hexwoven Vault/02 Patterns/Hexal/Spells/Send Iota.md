---
aliases: ["Send Iota"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/wisps]
kind: "Spell"
family: "Spell"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:link/comm/send"
signature: "qqqqqwdeddw"
start_direction: "NORTH_WEST"
input: "num, any"
output: ""
cost: "costs one hundredth of an Amethyst Dust."
iotas_in: ["[[Number]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Links]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Send Iota

✦ **Spell** · [[Hexal]] · 📖 [[Links]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 85.0,49.3 100.7,22.0 116.5,49.3 100.7,76.6 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:378;--dur:3.4s" points="85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 85.0,49.3 100.7,22.0 116.5,49.3 100.7,76.6 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="80.0" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,34.3 83.0,38.8 77.0,42.3" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwdeddw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, any` ([[Number]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **Cost:** costs one hundredth of an Amethyst Dust.
- **ID:** `hexal:link/comm/send`

```text
before   … │ num │ any  ◂ top
after    …              ◂ top
```

## Description

Removes an [[Iotas|iota]] from the [[The Stack|stack]], then removes an index from the stack and sends the iota along the link at the given index.  This costs one hundredth of an [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Link]] `linkable → —`
- [[Link Others]] `linkable, linkable → —`
- [[Unlink]] `num → —`
- [[Unlink Others]] `linkable, linkable → —`
- [[Phonebook Purification]] `num → linkable`
- [[Recognition Purification]] `linkable → num`
- [[Popularity Reflection]] `— → num`
- [[Recitation Reflection]] `— → any`
- [[Postmaster's Reflection]] `— → num`
- [[Unclogging Gambit]] `— → —`
- [[Open Transmit]] `num → —`
- [[Close Transmit]] `— → —`

## Sources

- 📖 **Guidebook:** [[Links]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Send Iota](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/links@hexal:link/comm/send)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSendIota` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:link/comm/send`
