---
aliases: ["Flock's Disintegration"]
tags: [hexcasting/pattern, mod/hex-casting, family/disintegration, topic/lists]
kind: "Pattern"
family: "Disintegration"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:splat"
signature: "qwaeawq"
start_direction: "NORTH_WEST"
input: "list"
output: "many"
cost: ""
iotas_in: ["[[List]]"]
iotas_out: []
entry: "[[List Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Flock's Disintegration

◇ **Pattern** · [[Hex Casting]] · 📖 [[List Manipulation]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✷ Disintegration
> This pattern takes **one** iota and spills out **many**: its name marks it as a **Disintegration**. See [[Pattern Naming Guide]] · [[Patterns by Family#Disintegration|all Disintegration patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="68.3" x2="132.5" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,47.5 136.7,53.8 128.4,58.6" fill="#fc77be"/><circle cx="139.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwaeawq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `many` (—)
- **ID:** `hexcasting:splat`

## Description

Remove the list at the top of the [[The Stack|stack]], then push its contents to the stack.

## Example

`[a, b, c] → a, b, c`. The opposite of [[Flock's Gambit]]: unpack a list read from a [[Focus]] back onto the stack.

## Other uses (same pattern, other inputs)

### [[PhLib]] · 📖 [[Maps]]

`map<k, v>` → `k, v, k, v...`

Disintegrates the map, returning the key-value pairs in the order I inserted them.

### [[Caduceus]] · 📖 [[Jump Manipulation]]

`jump` → `many`

Remove the Jump iota at the top of the stack, then, for each of its frames, push a new Jump iota containing only that frame to the stack.

## Related patterns (same book page)

- [[Surgeon's Exaltation]] `list, num, any → list`
- [[Excisor's Distillation]] `list, num → list`
- [[Selection Distillation]] `list, number → any`
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
- 🌐 **Official online docs:** [Flock's Disintegration](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists@hexcasting:splat)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSplat` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:splat`
