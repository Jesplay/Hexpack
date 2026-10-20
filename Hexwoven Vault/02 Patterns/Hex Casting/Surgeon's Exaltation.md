---
aliases: ["Surgeon's Exaltation"]
tags: [hexcasting/pattern, mod/hex-casting, family/exaltation, topic/lists]
kind: "Pattern"
family: "Exaltation"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:replace"
signature: "wqaeaqw"
start_direction: "NORTH_WEST"
input: "list, num, any"
output: "list"
cost: ""
iotas_in: ["[[Number]]", "[[List]]", "[[Any Iota]]"]
iotas_out: ["[[List]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Surgeon's Exaltation

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="135.0,121.6 110.0,78.3 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6 85.0,121.6 110.0,78.3 135.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="135.0,121.6 110.0,78.3 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6 85.0,121.6 110.0,78.3 135.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.0" y1="121.6" x2="127.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="121.2,97.8 131.8,105.0 122.2,110.5" fill="#fc77be"/><circle cx="135.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaeaqw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, num, any` ([[Number]], [[List]], [[Any Iota]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:replace`

```text
before   … │ list │ num │ any  ◂ top
after    … │ list              ◂ top
```

## Description

Remove the top [[Iotas|iota]] of the [[The Stack|stack]] and the number at the top, then set the nth element of the list at the top of the stack to that iota (where n is the number you removed). Does nothing if the number is out of bounds.

## Other uses (same pattern, other inputs)

### [[PhLib]] · 📖 [[Maps]]

`map<k, v>, k, v` → `map<k, v>`

Replaces the element associated with **k** with **v**. If **k** is not present in the map, the entry is created and placed at the end of the map.

### [[HexThings]] · 📖 [[Dictionary Manipulation]]

`dict, key, any` → `dict`

Places the iota in the Dictionary at the specified key. Not every iota type is allowed to be used as a key, with notable the exceptions being Garbage and [[Iris' Gambit|Jump]] iota. Attempting to set the value of a key to [[Nullary Reflection|Null]] will remove the key the same way using [[Excisor's Distillation]] would.

### [[Hexpose]] · 📖 [[Text Iota]]

`text, number, text` → `text`

Replaces the glyph at the nth position in the text with another segment of text. Replacement can be longer than one glyph.

## Related patterns (same book page)

- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
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
- 🌐 **Official online docs:** [Surgeon's Exaltation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:replace)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:replace`
