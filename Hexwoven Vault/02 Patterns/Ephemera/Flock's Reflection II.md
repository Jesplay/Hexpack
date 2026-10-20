---
aliases: ["Flock's Reflection II"]
tags: [hexcasting/pattern, mod/ephemera, family/reflection, topic/stack]
kind: "Pattern"
family: "Reflection"
topics: ["Stack Manipulation"]
mod: "Ephemera"
op_id: "ephemera:getstacksizedeep"
signature: "qwaeawqaqded"
start_direction: "NORTH_WEST"
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

# Flock's Reflection II

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 95.9,68.3 74.1,106.0 52.3,68.3 74.1,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:3.6s" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 95.9,68.3 74.1,106.0 52.3,68.3 74.1,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="68.3" x2="132.5" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,47.5 136.7,53.8 128.4,58.6" fill="#fc77be"/><circle cx="139.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwaeawqaqded` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `ephemera:getstacksizedeep`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Returns the total size of all [[Iotas|iotas]] on my [[The Stack|stack]], including the contents of lists and similar. If this number ever reaches 1024, my stack shall come crashing down into garbage.

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Decorator's Gambit]] `item frame, number → —`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[Entropy Reflection II]] `— → num`
- [[Clearance Purification]] `vec → bool`
- [[Flock's Purification]] `— → num`
- [[State Purification]] `vec → [string]`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[Ride Wisp]] `wisp → —`
- [[Dismount]] `— → —`
- [[Equestrian Purification]] `entity → [entity]`
- [[Equestrian Purification II]] `entity → entity`

## Sources

- 📖 **Guidebook:** [[Misc. Ephemera Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Flock's Reflection II](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:getstacksizedeep)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpStackSizeDeep` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getstacksizedeep`
