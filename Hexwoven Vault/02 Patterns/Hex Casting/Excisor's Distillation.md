---
aliases: ["Excisor's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/lists]
kind: "Pattern"
family: "Distillation"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:remove_from"
signature: "edqdewaqa"
start_direction: "SOUTH_WEST"
input: "list, num"
output: "list"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[List]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Excisor's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="95.9,68.3 74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 95.9,68.3 117.7,106.0 139.5,68.3 117.7,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:436;--dur:2.8s" points="95.9,68.3 74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 95.9,68.3 117.7,106.0 139.5,68.3 117.7,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="68.3" x2="88.9" y2="80.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="83.9,89.0 84.8,77.9 93.1,82.7" fill="#fc77be"/><circle cx="95.9" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edqdewaqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, num` ([[Number]], [[List]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:remove_from`

```text
before   … │ list │ num  ◂ top
after    … │ list        ◂ top
```

## Description

Remove the number at the top of the [[The Stack|stack]], then remove the nth element of the list at the top of the stack (where n is the number you removed).

## Other uses (same pattern, other inputs)

### [[PhLib]] · 📖 [[Maps]]

`map<k, v>, k` → `map<k, v>`

Throws out the map's association for the given key. If there is no matching association, the map is returned unchanged.

### [[HexThings]] · 📖 [[Dictionary Manipulation]]

`dict, key` → `dict, any`

Removes the key-value pair from the Dictionary, pushing the previous value, or [[Nullary Reflection|Null]] if no value was present.

### [[Hexpose]] · 📖 [[Text Iota]]

`text, number` → `text`

Removes the nth glyph in the text.

### [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]]

`set, (num|vec|entity)` → `set`

Remove the iota from top of the stack, then remove it set. The iota's type must match the set's type.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
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
- 🌐 **Official online docs:** [Excisor's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:remove_from)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:remove_from`
