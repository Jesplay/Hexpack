---
aliases: ["Popularity Reflection"]
tags: [hexcasting/pattern, mod/hexal, family/reflection, topic/wisps]
kind: "Pattern"
family: "Reflection"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:link/num"
signature: "qeeeeedww"
start_direction: "WEST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Links]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Popularity Reflection

◇ **Pattern** · [[Hexal]] · 📖 [[Links]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="148.0,49.3 116.5,49.3 100.7,76.6 69.3,76.6 53.5,49.3 69.3,22.0 100.7,22.0 116.5,49.3 85.0,49.3 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:2.8s" points="148.0,49.3 116.5,49.3 100.7,76.6 69.3,76.6 53.5,49.3 69.3,22.0 100.7,22.0 116.5,49.3 85.0,49.3 53.5,49.3 22.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="148.0" y1="49.3" x2="137.9" y2="49.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="130.6,49.3 137.9,45.8 137.9,52.8" fill="#fc77be"/><circle cx="148.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeeeeedww` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:link/num`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Adds the number of links to the wisp to the [[The Stack|stack]].

## Related patterns (same book page)

- [[Link]] `linkable → —`
- [[Link Others]] `linkable, linkable → —`
- [[Unlink]] `num → —`
- [[Unlink Others]] `linkable, linkable → —`
- [[Phonebook Purification]] `num → linkable`
- [[Recognition Purification]] `linkable → num`
- [[Send Iota]] `num, any → —`
- [[Recitation Reflection]] `— → any`
- [[Postmaster's Reflection]] `— → num`
- [[Unclogging Gambit]] `— → —`
- [[Open Transmit]] `num → —`
- [[Close Transmit]] `— → —`

## Sources

- 📖 **Guidebook:** [[Links]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Popularity Reflection](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/links@hexal:link/num)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNumLinked` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:link/num`
