---
aliases: ["Inversion Purification"]
tags: [hexcasting/pattern, mod/hierophantics, family/purification, topic/minds]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:invert_trigger"
signature: "aawddeeqqqqqaw"
start_direction: "NORTH_EAST"
input: "trigger"
output: "trigger"
cost: ""
iotas_in: ["[[Trigger]]"]
iotas_out: ["[[Trigger]]"]
entry: "[[Hierophantic Triggers]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Inversion Purification

◇ **Pattern** · [[Hierophantics]] · 📖 [[Hierophantic Triggers]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="41.6,54.3 59.0,24.3 24.3,24.3 41.6,54.3 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:520;--dur:4.2s" points="41.6,54.3 59.0,24.3 24.3,24.3 41.6,54.3 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.6" y1="54.3" x2="47.2" y2="44.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.2,37.8 50.5,46.6 43.9,42.8" fill="#fc77be"/><circle cx="41.6" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawddeeqqqqqaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `trigger` ([[Trigger]])
- **Leaves:** `trigger` ([[Trigger]])
- **ID:** `hierophantics:invert_trigger`

```text
before   … │ trigger  ◂ top
after    … │ trigger  ◂ top
```

## Description

Inverts the threshold on a numerical trigger. For example, 'When velocity exceeds 5.0' will become 'When velocity drops below 5.0'. Has no effect on other types of triggers.

## Related patterns (same book page)

- [[Trigger Reflection Damage]] `— → trigger`
- [[Trigger Reflection Damage Type]] `— → trigger`
- [[Trigger Purification Health]] `num → trigger`
- [[Trigger Purification Breath]] `num → trigger`
- [[Trigger Purification Hunger]] `num → trigger`
- [[Trigger Purification Velocity]] `num → trigger`
- [[Trigger Purification Fall]] `num → trigger`
- [[Trigger Reflection Drop]] `— → trigger`
- [[Trigger Reflection Attack]] `— → trigger`
- [[Trigger Reflection Break]] `— → trigger`
- [[Trigger Reflection Jump]] `— → trigger`
- [[Trigger Reflection Teleport]] `— → trigger`

## Sources

- 📖 **Guidebook:** [[Hierophantic Triggers]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpInvertTrigger` is not a `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:invert_trigger`
