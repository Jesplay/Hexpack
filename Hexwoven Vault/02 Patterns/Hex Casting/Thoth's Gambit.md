---
aliases: ["Thoth's Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:for_each"
signature: "dadad"
start_direction: "NORTH_EAST"
input: "evaluatable, list"
output: "list"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: ["[[List]]"]
entry: "[[Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Thoth's Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:232;--dur:1.7s" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,42.1 36.9,51.9 29.5,47.7" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dadad` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `evaluatable, list` ([[List]], [[Pattern]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:for_each`

```text
before   … │ evaluatable │ list  ◂ top
after    … │ list                ◂ top
```

## Description

Remove something evaluatable and a list from the [[The Stack|stack]], then cast the evaluatable over each element of the list.

More specifically, for each element in the provided list, it will:
- Create a new stack, with everything on the current stack plus that element
- Cast the evaluatable [[Iotas|iota]]
- Save all the iotas remaining on the stack to a list

Then, after all is said and done, pushes the list of saved iotas onto the main stack.

No wonder all the practitioners of this art go mad.

## Example

Runs a list of patterns **once for every element** of a list. Each run starts from a **copy of the current stack plus that element**, and everything left afterwards is collected into the result list.

Double every number: code `[Numerical Reflection 2, Multiplicative Distillation]`, list `[1, 2, 3]` → **Thoth's Gambit** → `[2, 4, 6]`.

Typical use: [[Zone Distillation Any]] for nearby entities, then Thoth's to do something to each of them. Keep the stack otherwise empty, or leftovers end up in every result. More in [[Gambits Explained]].

## Related patterns (same book page)

- [[Hermes' Gambit]] `evaluatable → many`
- [[Iris' Gambit]] `evaluatable → many`
- [[Charon's Gambit]] `— → —`
- [[Thanatos' Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Thoth's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/meta@hexcasting:for_each)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpForEach` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:for_each`
