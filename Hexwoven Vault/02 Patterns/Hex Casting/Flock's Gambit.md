---
aliases: ["Flock's Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/lists]
kind: "Pattern"
family: "Gambit"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:last_n_list"
signature: "ewdqdwe"
start_direction: "SOUTH_WEST"
input: "many, num"
output: "list"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[List]]"]
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Flock's Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,68.3 117.7,106.0 74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 117.7,30.5 139.5,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="139.5,68.3 117.7,106.0 74.1,106.0 30.5,106.0 52.3,68.3 30.5,30.5 74.1,30.5 117.7,30.5 139.5,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="68.3" x2="132.5" y2="80.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,89.0 128.4,77.9 136.7,82.7" fill="#fc77be"/><circle cx="139.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewdqdwe` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many, num` ([[Number]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:last_n_list`

## Description

Remove num elements from the [[The Stack|stack]], then add them to a list at the top of the stack.

## Example

`a, b, c, 2 → a, [b, c]`. Packs the top *n* [[Iotas|iotas]] into a list, e.g. to feed several values into [[Thoth's Gambit]] or save them all with [[Scribe's Gambit]].

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
- [[Speaker's Distillation]] `list, any → list`

## Sources

- 📖 **Guidebook:** [[List Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Flock's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:last_n_list)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpLastNToList` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:last_n_list`
