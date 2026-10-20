---
aliases: ["Speaker's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/lists]
kind: "Pattern"
family: "Distillation"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:construct"
signature: "ddewedd"
start_direction: "SOUTH_EAST"
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

# Speaker's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="46.4,27.0 65.7,60.5 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="46.4,27.0 65.7,60.5 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="27.0" x2="52.5" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.0,45.4 48.9,39.9 56.2,35.6" fill="#fc77be"/><circle cx="46.4" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddewedd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, any` ([[List]], [[Any Iota]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:construct`

```text
before   … │ list │ any  ◂ top
after    … │ list        ◂ top
```

## Description

Remove the top [[Iotas|iota]], then add it as the first element to the list at the top of the [[The Stack|stack]].

## Other uses (same pattern, other inputs)

### [[Caduceus]] · 📖 [[Jump Manipulation]]

`jump, jump` → `jump`

Remove the Jump iota at the top of the stack, then add its top frame to the top of the Jump iota at the top of the stack.

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
- [[Integration Distillation]] `list, any → list`
- [[Vacant Reflection]] `— → list`
- [[Single's Purification]] `any → list`
- [[Flock's Gambit]] `many, num → list`

## Sources

- 📖 **Guidebook:** [[List Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Speaker's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:construct)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:construct`
