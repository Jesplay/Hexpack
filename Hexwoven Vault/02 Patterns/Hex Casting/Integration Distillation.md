---
aliases: ["Integration Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/lists]
kind: "Pattern"
family: "Distillation"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:append"
signature: "edqde"
start_direction: "SOUTH_WEST"
input: "list, any"
output: "list"
cost: ""
iotas_in: ["[[List]]", "[[Any Iota]]"]
iotas_out: ["[[List]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Integration Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="119.4,85.0 92.3,132.0 38.0,132.0 65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="119.4,85.0 92.3,132.0 38.0,132.0 65.1,85.0 38.0,38.0 92.3,38.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="119.4" y1="85.0" x2="110.7" y2="100.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="104.5,110.9 105.6,97.1 115.9,103.0" fill="#fc77be"/><circle cx="119.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edqde` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, any` ([[List]], [[Any Iota]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:append`

```text
before   … │ list │ any  ◂ top
after    … │ list        ◂ top
```

## Description

Remove the top of the [[The Stack|stack]], then add it to the end of the list at the top of the stack.

## Other uses (same pattern, other inputs)

### [[Hextra Patterns]] · 📖 [[Hash Set Manipulation]]

`set, (num|vec|entity)` → `set`

Remove the iota at the top of the stack, then add it to the set. The iota's type must match the set's type.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
- [[Flock's Disintegration]] `list → many`
- [[Derivation Decomposition]] `list → list, any`
- [[Speaker's Decomposition]] `list → list, any`
- [[Locator's Distillation]] `list, any → num`
- [[Selection Exaltation]] `list, num, num → list`
- [[Retrograde Purification]] `list → list`
- [[Vacant Reflection]] `— → list`
- [[Single's Purification]] `any → list`
- [[Flock's Gambit]] `many, num → list`
- [[Speaker's Distillation]] `list, any → list`

## Sources

- 📖 **Guidebook:** [[List Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Integration Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:append)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:append`
