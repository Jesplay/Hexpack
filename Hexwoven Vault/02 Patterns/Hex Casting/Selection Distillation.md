---
aliases: ["Selection Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/lists]
kind: "Pattern"
family: "Distillation"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:index"
signature: "deeed"
start_direction: "NORTH_WEST"
input: "list, number"
output: "any"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Selection Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0 92.3,132.0 38.0,132.0 65.1,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0 92.3,132.0 38.0,132.0 65.1,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="85.0" x2="56.4" y2="70.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.2,59.1 61.6,67.0 51.3,72.9" fill="#fc77be"/><circle cx="65.1" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeed` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, number` ([[Number]], [[List]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexcasting:index`

```text
before   … │ list │ number  ◂ top
after    … │ any            ◂ top
```

## Description

Remove the number at the top of the [[The Stack|stack]], then replace the list at the top with the nth element of that list (where n is the number you removed). Replaces the list with **Null** if the number is out of bounds.

## Other uses (same pattern, other inputs)

### [[PhLib]] · 📖 [[Maps]]

`map<k, v>, k` → `v?`

Destroys the map, returning only the value associated with the given key. O(1), unlike scanning a list.

### [[Hexpose]] · 📖 [[Text Iota]]

`text, number` → `text`

Pushes the nth glyph in the text.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Flock's Disintegration]] `list → many`
- [[Derivation Decomposition]] `list → list, any`
- [[Speaker's Decomposition]] `list → list, any`
- [[Locator's Distillation]] `list, any → num`
- [[Selection Exaltation]] `list, num, num → list`
- [[Retrograde Purification]] `list → list`
- [[Integration Distillation]] `list, any → list`
- [[Vacant Reflection]] `— → list`
- [[Single's Purification]] `any → list`
- [[Flock's Gambit]] `many, num → list`
- [[Speaker's Distillation]] `list, any → list`

## Sources

- 📖 **Guidebook:** [[List Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Selection Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:index)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:index`
