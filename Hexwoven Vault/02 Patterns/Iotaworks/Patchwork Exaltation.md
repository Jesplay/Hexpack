---
aliases: ["Patchwork Exaltation"]
tags: [hexcasting/pattern, mod/iotaworks, family/exaltation, topic/data-structures]
kind: "Pattern"
family: "Exaltation"
topics: ["Sets, Maps & Special Iotas"]
mod: "Iotaworks"
op_id: "iotaworks:metatable"
signature: "deaqqwqqqeaeqqqeadedaqaaee"
start_direction: "SOUTH_EAST"
input: "iota, iota, vec, property"
output: "patchwork"
cost: ""
iotas_in: ["[[Vector]]", "[[Property]]", "[[Any Iota]]"]
iotas_out: ["[[Patchwork]]"]
entry: "[[Iota Patching]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Patchwork Exaltation

◇ **Pattern** · [[Iotaworks]] · 📖 [[Iota Patching]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="139" height="170" viewBox="0 0 139.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="95.4" cy="18.0" r="2"/><circle cx="121.1" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="108.2" cy="40.4" r="2"/><circle cx="134.0" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="95.4" cy="62.7" r="2"/><circle cx="121.1" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="108.2" cy="85.0" r="2"/><circle cx="134.0" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="95.4" cy="107.3" r="2"/><circle cx="121.1" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="108.2" cy="129.6" r="2"/><circle cx="134.0" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/><circle cx="95.4" cy="152.0" r="2"/><circle cx="121.1" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="82.5,40.4 95.4,62.7 69.6,62.7 56.7,40.4 43.8,62.7 56.7,85.0 82.5,85.0 108.2,85.0 121.1,62.7 108.2,40.4 82.5,40.4 69.6,18.0 56.7,40.4 30.9,40.4 18.0,62.7 30.9,85.0 56.7,85.0 69.6,107.3 82.5,85.0 95.4,107.3 82.5,129.6 69.6,107.3 56.7,129.6 69.6,152.0 82.5,129.6 56.7,129.6 43.8,107.3 56.7,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:696;--dur:6.0s" points="82.5,40.4 95.4,62.7 69.6,62.7 56.7,40.4 43.8,62.7 56.7,85.0 82.5,85.0 108.2,85.0 121.1,62.7 108.2,40.4 82.5,40.4 69.6,18.0 56.7,40.4 30.9,40.4 18.0,62.7 30.9,85.0 56.7,85.0 69.6,107.3 82.5,85.0 95.4,107.3 82.5,129.6 69.6,107.3 56.7,129.6 69.6,152.0 82.5,129.6 56.7,129.6 43.8,107.3 56.7,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="82.5" y1="40.4" x2="86.6" y2="47.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="89.6,52.6 84.1,48.9 89.1,46.1" fill="#fc77be"/><circle cx="82.5" cy="40.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deaqqwqqqeaeqqqeadedaqaaee` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `iota, iota, vec, property` ([[Vector]], [[Property]], [[Any Iota]])
- **Leaves:** `patchwork` ([[Patchwork]])
- **ID:** `iotaworks:metatable`

```text
before   … │ iota │ iota │ vec │ property  ◂ top
after    … │ patchwork                     ◂ top
```

## Description

Constructs a Patchwork [[Iotas|iota]] from the userdata (iota stored inside), display iota (visible to my eyes), color, and an overload map.

These **overload maps** resemble a technique I've seen before — the concept of redefining patterns on a fundamental level resembles the moon cultists' (little time to explain) [metatables](https://www.lua.org/pil/13.html), which allow associating a table with another table to redefine its operations. When my 'metamethods' are invoked, the userdata iota is pushed to the top of the [[The Stack|stack]]. All other information about the patchwork disintegrates: I'll need to recreate it from scratch (maybe I should use a

'constructor' macro?) if I want to return a new iota of the same type.

Important note: unlike these 'metatables' I've heard about, patchwork *replaces* the behavior of the underlying iota. For example, operators such as &lt;ne,waaw&gt; won't work on two patchwork numbers unless I define it myself. However, I feel uneasy relying on this behavior — Nature will be perfectly happy to change this at any moment.

These **overload maps** resemble a technique I've heard of before — the concept of redefining patterns on a fundamental level resembles the moon cultists' (little time to explain) [metatables](https://www.lua.org/pil/13.html), which allow associating a table with another table to redefine its operations. When my 'metamethods' are invoked, the userdata iota is pushed to the top of the stack. All other information about the patchwork disintegrates: I'll need to recreate it from scratch (maybe I should use a 'constructor' macro?) if I want to return a new iota of the same type.

Important note: unlike these 'metatables' I've heard about, patchwork *replaces* the behavior of the underlying iota. For example, operators such as &lt;ne,waaw&gt; won't work on two patchwork numbers unless I define it myself. However, I feel uneasy relying on this behavior — Nature will be perfectly happy to change this at any moment.

## Sources

- 📖 **Guidebook:** [[Iota Patching]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `iotaworks:metatable`
