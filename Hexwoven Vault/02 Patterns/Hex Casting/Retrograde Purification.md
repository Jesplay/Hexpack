---
aliases: ["Retrograde Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/lists]
kind: "Pattern"
family: "Purification"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:reverse"
signature: "qqqaede"
start_direction: "EAST"
input: "list"
output: "list"
cost: ""
iotas_in: ["[[List]]"]
iotas_out: ["[[List]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Retrograde Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,121.6 110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 85.0,78.3 60.0,121.6 35.0,78.3 60.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="60.0,121.6 110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 85.0,78.3 60.0,121.6 35.0,78.3 60.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="121.6" x2="76.0" y2="121.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="87.5,121.6 76.0,127.1 76.0,116.1" fill="#fc77be"/><circle cx="60.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqaede` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:reverse`

```text
before   … │ list  ◂ top
after    … │ list  ◂ top
```

## Description

Reverse the list at the top of the [[The Stack|stack]].

## Other uses (same pattern, other inputs)

### [[Hexpose]] · 📖 [[Text Iota]]

`text` → `text`

Reverses the text.

### [[MoreIotas]] · 📖 [[Matrices]]

`mat` → `mat`

Accepts an n by m matrix and returns an m by n matrix where every entry has been flipped along the diagonal.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
- [[Flock's Disintegration]] `list → many`
- [[Derivation Decomposition]] `list → list, any`
- [[Speaker's Decomposition]] `list → list, any`
- [[Locator's Distillation]] `list, any → num`
- [[Selection Exaltation]] `list, num, num → list`
- [[Integration Distillation]] `list, any → list`
- [[Vacant Reflection]] `— → list`
- [[Single's Purification]] `any → list`
- [[Flock's Gambit]] `many, num → list`
- [[Speaker's Distillation]] `list, any → list`

## Sources

- 📖 **Guidebook:** [[List Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Retrograde Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:reverse)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:reverse`
