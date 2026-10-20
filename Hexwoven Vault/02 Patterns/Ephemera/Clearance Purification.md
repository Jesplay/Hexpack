---
aliases: ["Clearance Purification"]
tags: [hexcasting/pattern, mod/ephemera, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Ephemera"
op_id: "ephemera:collisionprobe"
signature: "qaqqqqqdaqa"
start_direction: "NORTH_WEST"
input: "vec"
output: "bool"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Misc. Ephemera Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Clearance Purification

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="123.6,94.0 104.3,60.5 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="123.6,94.0 104.3,60.5 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="94.0" x2="117.5" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="113.0,75.6 121.1,81.1 113.8,85.4" fill="#fc77be"/><circle cx="123.6" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqqdaqa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `ephemera:collisionprobe`

```text
before   … │ vec   ◂ top
after    … │ bool  ◂ top
```

## Description

Accepts a vector, and returns whether that vector is within (or just on the edge of) the block in that space. Seems to involve the boxes I see in my mind's eye.

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Decorator's Gambit]] `item frame, number → —`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[Entropy Reflection II]] `— → num`
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
- 🌐 **Official online docs:** [Clearance Purification](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:collisionprobe)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCollisionProbe` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:collisionprobe`
