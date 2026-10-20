---
aliases: ["Speaker's Decomposition"]
tags: [hexcasting/pattern, mod/hex-casting, family/decomposition, topic/lists]
kind: "Pattern"
family: "Decomposition"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:deconstruct"
signature: "aaqwqaa"
start_direction: "SOUTH_WEST"
input: "list"
output: "list, any"
cost: ""
iotas_in: ["[[List]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Speaker's Decomposition

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="123.6,27.0 104.3,60.5 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 65.7,60.5 46.4,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="123.6,27.0 104.3,60.5 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 65.7,60.5 46.4,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="27.0" x2="117.5" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="113.0,45.4 113.8,35.6 121.1,39.9" fill="#fc77be"/><circle cx="123.6" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqwqaa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `list, any` ([[List]], [[Any Iota]])
- **ID:** `hexcasting:deconstruct`

```text
before   … │ list        ◂ top
after    … │ list │ any  ◂ top
```

## Description

Remove the first [[Iotas|iota]] from the list at the top of the [[The Stack|stack]], then push that iota to the stack.

## Other uses (same pattern, other inputs)

### [[PhLib]] · 📖 [[Maps]]

`map<k,v>` → `map<k,v>, (k,v | nulls)`

Shaves off the element I added least recently from a map. If used on an empty map, returns two nulls.

### [[Caduceus]] · 📖 [[Jump Manipulation]]

`jump` → `jump, jump`

Remove the top frame from the Jump iota at the top of the stack, then push a new Jump iota containing only that frame to the stack.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
- [[Flock's Disintegration]] `list → many`
- [[Derivation Decomposition]] `list → list, any`
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
- 🌐 **Official online docs:** [Speaker's Decomposition](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:deconstruct)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:deconstruct`
