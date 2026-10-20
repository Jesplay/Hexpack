---
aliases: ["Entropy Reflection II"]
tags: [hexcasting/pattern, mod/ephemera, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Ephemera"
op_id: "ephemera:gaussianrand"
signature: "eeeeq"
start_direction: "NORTH_EAST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Misc. Ephemera Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Entropy Reflection II

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="66.6" x2="36.6" y2="54.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,46.3 40.6,57.1 32.5,52.5" fill="#fc77be"/><circle cx="29.8" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `ephemera:gaussianrand`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Returns a random number with a gaussian distribution, with a mean of zero and a standard deviation of one.

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Decorator's Gambit]] `item frame, number → —`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[Clearance Purification]] `vec → bool`
- [[Flock's Reflection II]] `— → num`
- [[Flock's Purification]] `— → num`
- [[State Purification]] `vec → [string]`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[Ride Wisp]] `wisp → —`
- [[Dismount]] `— → —`
- [[Equestrian Purification]] `entity → [entity]`
- [[Equestrian Purification II]] `entity → entity`

## Sources

- 📖 **Guidebook:** [[Misc. Ephemera Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Entropy Reflection II](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:gaussianrand)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGaussianRand` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:gaussianrand`
