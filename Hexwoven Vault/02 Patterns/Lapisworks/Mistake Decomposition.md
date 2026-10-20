---
aliases: ["Mistake Decomposition"]
tags: [hexcasting/pattern, mod/lapisworks, family/decomposition, topic/minds, topic/enchanting]
kind: "Pattern"
family: "Decomposition"
topics: ["Minds & Hierophantics", "Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:get_amalgam_err"
signature: "eqaqqaqqadqeqaqqaqqad"
start_direction: "NORTH_WEST"
input: "amalgam"
output: "amalgam, num"
cost: ""
iotas_in: ["[[Amalgam]]"]
iotas_out: ["[[Number]]", "[[Amalgam]]"]
entry: "[[Amalgamations]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Mistake Decomposition

◇ **Pattern** · [[Lapisworks]] · 📖 [[Amalgamations]] · Lapisworks › Interop › Chariot

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]] · [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="88" height="170" viewBox="0 0 87.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.0" cy="18.0" r="2"/><circle cx="43.8" cy="18.0" r="2"/><circle cx="69.6" cy="18.0" r="2"/><circle cx="5.2" cy="40.4" r="2"/><circle cx="30.9" cy="40.4" r="2"/><circle cx="56.7" cy="40.4" r="2"/><circle cx="82.5" cy="40.4" r="2"/><circle cx="18.0" cy="62.7" r="2"/><circle cx="43.8" cy="62.7" r="2"/><circle cx="69.6" cy="62.7" r="2"/><circle cx="5.2" cy="85.0" r="2"/><circle cx="30.9" cy="85.0" r="2"/><circle cx="56.7" cy="85.0" r="2"/><circle cx="82.5" cy="85.0" r="2"/><circle cx="18.0" cy="107.3" r="2"/><circle cx="43.8" cy="107.3" r="2"/><circle cx="69.6" cy="107.3" r="2"/><circle cx="5.2" cy="129.6" r="2"/><circle cx="30.9" cy="129.6" r="2"/><circle cx="56.7" cy="129.6" r="2"/><circle cx="82.5" cy="129.6" r="2"/><circle cx="18.0" cy="152.0" r="2"/><circle cx="43.8" cy="152.0" r="2"/><circle cx="69.6" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="56.7,85.0 43.8,62.7 56.7,40.4 43.8,18.0 30.9,40.4 43.8,62.7 69.6,62.7 56.7,40.4 30.9,40.4 18.0,62.7 43.8,62.7 30.9,85.0 43.8,107.3 30.9,129.6 43.8,152.0 56.7,129.6 43.8,107.3 18.0,107.3 30.9,129.6 56.7,129.6 69.6,107.3 43.8,107.3 56.7,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:6.0s" points="56.7,85.0 43.8,62.7 56.7,40.4 43.8,18.0 30.9,40.4 43.8,62.7 69.6,62.7 56.7,40.4 30.9,40.4 18.0,62.7 43.8,62.7 30.9,85.0 43.8,107.3 30.9,129.6 43.8,152.0 56.7,129.6 43.8,107.3 18.0,107.3 30.9,129.6 56.7,129.6 69.6,107.3 43.8,107.3 56.7,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="56.7" y1="85.0" x2="52.6" y2="77.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.6,72.7 55.0,76.4 50.1,79.3" fill="#fc77be"/><circle cx="56.7" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqaqqaqqadqeqaqqaqqad` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `amalgam` ([[Amalgam]])
- **Leaves:** `amalgam, num` ([[Number]], [[Amalgam]])
- **ID:** `lapisworks:get_amalgam_err`

```text
before   … │ amalgam        ◂ top
after    … │ amalgam │ num  ◂ top
```

## Description

Pushes the error of the provided amalgamation.

## Related patterns (same book page)

- [[Amalgamation Purification]] `vec → amalgam`
- [[The Mind's Decomp]] `amalgam → amalgam, bool`
- [[Loudener's Decomposition]] `amalgam → amalgam, int`
- [[Loudener's Distillation]] `amalgam, num → amalgam`
- [[Fisherman's Decomposition]] `amalgam → amalgam, num`
- [[Fisherman's Distillation]] `amalgam, num → amalgam`
- [[My Mind's Decomposition]] `— → num`
- [[Amalgamation Gambit]] `num → —`
- [[Etched Decomposition]] `amalgam → amalgam, [pattern]`
- [[Etched Distillation]] `amalgam, [pattern] → amalgam`

## Sources

- 📖 **Guidebook:** [[Amalgamations]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetAmalgamErr` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_amalgam_err`
