---
aliases: ["Flock's Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/stack]
kind: "Pattern"
family: "Reflection"
topics: ["Stack Manipulation"]
mod: "Hex Casting"
op_id: "hexcasting:stack_len"
signature: "qwaeawqaeaqa"
start_direction: "NORTH_WEST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Flock's Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 95.9,68.3 74.1,30.5 52.3,68.3 74.1,106.0 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:3.6s" points="139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 95.9,68.3 74.1,30.5 52.3,68.3 74.1,106.0 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="68.3" x2="132.5" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="127.5,47.5 136.7,53.8 128.4,58.6" fill="#fc77be"/><circle cx="139.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwaeawqaeaqa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexcasting:stack_len`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Pushes the size of the [[The Stack|stack]] as a number to the top of the stack. (For example, a stack of \[0, 1\] will become \[0, 1, 2\].)

## Example

`0, 1 → 0, 1, 2`. Pairs well with [[Flock's Gambit]]: [[Flock's Reflection]] → [[Flock's Gambit]] packs the *entire* stack into one list.

## Related patterns (same book page)

- [[Novice's Gambit]] `any → —`
- [[Jester's Gambit]] `any, any → any, any`
- [[Rotation Gambit]] `any, any, any → any, any, any`
- [[Rotation Gambit II]] `any, any, any → any, any, any`
- [[Gemini Decomposition]] `any → any, any`
- [[Prospector's Gambit]] `any, any → any, any, any`
- [[Undertaker's Gambit]] `any, any → any, any, any`
- [[Gemini Gambit]] `any, number → many`
- [[Dioscuri Gambit]] `any, any → any, any, any, any`
- [[Fisherman's Gambit]] `number → any`
- [[Fisherman's Gambit II]] `number → any`
- [[Bookkeeper's Gambit]] `many → many`
- [[Swindler's Gambit]] `many, number → many`

## Sources

- 📖 **Guidebook:** [[Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Flock's Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/stackmanip@hexcasting:stack_len)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpStackSize` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:stack_len`
