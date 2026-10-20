---
aliases: ["Locator's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/lists]
kind: "Pattern"
family: "Distillation"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:index_of"
signature: "dedqde"
start_direction: "EAST"
input: "list, any"
output: "num"
cost: ""
iotas_in: ["[[List]]", "[[Any Iota]]"]
iotas_out: ["[[Number]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Locator's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,85.0 119.4,85.0 92.3,132.0 38.0,132.0 65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:380;--dur:2.0s" points="65.1,85.0 119.4,85.0 92.3,132.0 38.0,132.0 65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="85.0" x2="82.5" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="95.0,85.0 82.5,91.0 82.5,79.0" fill="#fc77be"/><circle cx="65.1" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedqde` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, any` ([[List]], [[Any Iota]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:index_of`

```text
before   … │ list │ any  ◂ top
after    … │ num         ◂ top
```

## Description

Remove the [[Iotas|iota]] at the top of the [[The Stack|stack]], then replace the list at the top with the first index of that iota within the list (starting from 0). Replaces the list with -1 if the iota doesn't exist in the list.

## Other uses (same pattern, other inputs)

### [[HexThings]] · 📖 [[Dictionary Manipulation]]

`dict, key` → `any`

Pushes the value present at the provided key, or [[Nullary Reflection|Null]] if no value was present.

### [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]]

`set, (num|vec|entity)` → `set, bool`

Remove the iota top of the stack, then return True if the set contains that iota, otherwise return False. The iota's type must match the set's type.

### [[MoreIotas]] · 📖 [[Strings]]

`str, str` → `num`

Removes two strings from the top of the stack, and returns the first index of the first string where the second string is a substring.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
- [[Flock's Disintegration]] `list → many`
- [[Derivation Decomposition]] `list → list, any`
- [[Speaker's Decomposition]] `list → list, any`
- [[Selection Exaltation]] `list, num, num → list`
- [[Retrograde Purification]] `list → list`
- [[Integration Distillation]] `list, any → list`
- [[Vacant Reflection]] `— → list`
- [[Single's Purification]] `any → list`
- [[Flock's Gambit]] `many, num → list`
- [[Speaker's Distillation]] `list, any → list`

## Sources

- 📖 **Guidebook:** [[List Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Locator's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:index_of)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:index_of`
