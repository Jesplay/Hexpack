---
aliases: ["Derivation Decomposition"]
tags: [hexcasting/pattern, mod/hex-casting, family/decomposition, topic/lists]
kind: "Pattern"
family: "Decomposition"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:unappend"
signature: "qaeaq"
start_direction: "NORTH_WEST"
input: "list"
output: "list, any"
cost: ""
iotas_in: ["[[List]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Derivation Decomposition

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="119.4,85.0 92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="119.4,85.0 92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="119.4" y1="85.0" x2="110.7" y2="70.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="104.5,59.1 115.9,67.0 105.6,72.9" fill="#fc77be"/><circle cx="119.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `list, any` ([[List]], [[Any Iota]])
- **ID:** `hexcasting:unappend`

```text
before   … │ list        ◂ top
after    … │ list │ any  ◂ top
```

## Description

Remove the [[Iotas|iota]] on the end of the list at the top of the [[The Stack|stack]], and add it to the top of the stack.

## Other uses (same pattern, other inputs)

### [[PhLib]] · 📖 [[Maps]]

`map<k,v>` → `map<k,v>, (k,v | nulls)`

Shaves off the element I added most recently from a map. If used on an empty map, returns two nulls.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
- [[Flock's Disintegration]] `list → many`
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
- 🌐 **Official online docs:** [Derivation Decomposition](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:unappend)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:unappend`
