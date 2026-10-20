---
aliases: ["Selection Exaltation"]
tags: [hexcasting/pattern, mod/hex-casting, family/exaltation, topic/lists]
kind: "Pattern"
family: "Exaltation"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:slice"
signature: "qaeaqwded"
start_direction: "NORTH_WEST"
input: "list, num, num"
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

# Selection Exaltation

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="95.9,68.3 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 117.7,30.5 139.5,68.3 117.7,106.0 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:436;--dur:2.8s" points="95.9,68.3 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 117.7,30.5 139.5,68.3 117.7,106.0 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="68.3" x2="88.9" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="83.9,47.5 93.1,53.8 84.8,58.6" fill="#fc77be"/><circle cx="95.9" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqwded` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, num, num` ([[Number]], [[List]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:slice`

```text
before   … │ list │ num │ num  ◂ top
after    … │ list              ◂ top
```

## Description

Remove the two numbers at the top of the [[The Stack|stack]], then take a sublist of the list at the top of the stack between those indices, lower bound inclusive, upper bound exclusive. For example, the 0, 2 sublist of \[0, 1, 2, 3, 4\] would be \[0, 1\].

## Other uses (same pattern, other inputs)

### [[Hexpose]] · 📖 [[Text Iota]]

`text, number, number` → `text`

Pushes the specified slice of the text.

### [[MoreIotas]] · 📖 [[Strings]]

`str, num, num` → `str`

Removes a string and two positive integers from the top of the stack, and returns the substring from the first to the second (inclusive first, exclusive second).

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
- [[Flock's Disintegration]] `list → many`
- [[Derivation Decomposition]] `list → list, any`
- [[Speaker's Decomposition]] `list → list, any`
- [[Locator's Distillation]] `list, any → num`
- [[Retrograde Purification]] `list → list`
- [[Integration Distillation]] `list, any → list`
- [[Vacant Reflection]] `— → list`
- [[Single's Purification]] `any → list`
- [[Flock's Gambit]] `many, num → list`
- [[Speaker's Distillation]] `list, any → list`

## Sources

- 📖 **Guidebook:** [[List Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Selection Exaltation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:slice)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:slice`
