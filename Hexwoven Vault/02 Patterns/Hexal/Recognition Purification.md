---
aliases: ["Recognition Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/wisps]
kind: "Pattern"
family: "Purification"
topics: ["Wisps & Links"]
mod: "Hexal"
op_id: "hexal:link/get_index"
signature: "aeqqqqqawwd"
start_direction: "SOUTH_WEST"
input: "linkable"
output: "num"
cost: ""
iotas_in: ["[[Linkable]]"]
iotas_out: ["[[Number]]"]
entry: "[[Links]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Recognition Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Links]] · Patterns › Spells

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="37.8,22.0 22.0,49.3 53.5,49.3 69.3,76.6 100.7,76.6 116.5,49.3 100.7,22.0 69.3,22.0 53.5,49.3 85.0,49.3 116.5,49.3 148.0,49.3 132.2,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:378;--dur:3.4s" points="37.8,22.0 22.0,49.3 53.5,49.3 69.3,76.6 100.7,76.6 116.5,49.3 100.7,22.0 69.3,22.0 53.5,49.3 85.0,49.3 116.5,49.3 148.0,49.3 132.2,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="22.0" x2="32.7" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="29.1,37.0 29.7,29.0 35.7,32.5" fill="#fc77be"/><circle cx="37.8" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeqqqqqawwd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `linkable` ([[Linkable]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexal:link/get_index`

```text
before   … │ linkable  ◂ top
after    … │ num       ◂ top
```

## Description

Removes an entity from the [[The Stack|stack]], and adds the index to the link to that entity to the stack (or -1 if the caster isn't linked to that entity).

## Related patterns (same book page)

- [[Link]] `linkable → —`
- [[Link Others]] `linkable, linkable → —`
- [[Unlink]] `num → —`
- [[Unlink Others]] `linkable, linkable → —`
- [[Phonebook Purification]] `num → linkable`
- [[Popularity Reflection]] `— → num`
- [[Send Iota]] `num, any → —`
- [[Recitation Reflection]] `— → any`
- [[Postmaster's Reflection]] `— → num`
- [[Unclogging Gambit]] `— → —`
- [[Open Transmit]] `num → —`
- [[Close Transmit]] `— → —`

## Sources

- 📖 **Guidebook:** [[Links]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Recognition Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/links@hexal:link/get_index)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetLinkedIndex` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:link/get_index`
