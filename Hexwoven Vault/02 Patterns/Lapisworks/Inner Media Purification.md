---
aliases: ["Inner Media Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/devices]
kind: "Pattern"
family: "Purification"
topics: ["Casting Devices"]
mod: "Lapisworks"
op_id: "lapisworks:get_mdia"
signature: "wddwqwddweqeee"
start_direction: "NORTH_WEST"
input: "M.T.I"
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Media Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Inner Media Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Media Manipulation]] · Lapisworks

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="53.5,76.6 37.8,49.3 22.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 22.0,131.1 37.8,103.8 53.5,76.6 85.0,76.6 100.7,49.3 132.2,49.3 148.0,76.6 132.2,103.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="53.5,76.6 37.8,49.3 22.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 22.0,131.1 37.8,103.8 53.5,76.6 85.0,76.6 100.7,49.3 132.2,49.3 148.0,76.6 132.2,103.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="76.6" x2="48.5" y2="67.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,61.6 51.5,66.1 45.5,69.6" fill="#fc77be"/><circle cx="53.5" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wddwqwddweqeee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `M.T.I` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `lapisworks:get_mdia`

```text
before   … │ M.T.I  ◂ top
after    … │ num    ◂ top
```

## Description

Fetches the amount of [[Media|media]] (in units of **Amethyst Dust**) contained in the given **Media Transfer Interface**.

A **Media Transfer Interface** is anything that supports media transfer. For example, if I happen upon media-containing entities (such as [[Craft Phial (Book Entry)|Phials]] on the ground or otherwise), media constructs (**wisps**, I could call them), or **blocks that contain media**, it is extremely likely that these two patterns may be used to interface with them.

*These patterns... They feel **alive**, **humming** even. It's as if some**thing**, no some**one** is trapped in them. The hum is so quiet and in my head, it is as if I'm imagining it. **Am*** I just imagining it?**

## Related patterns (same book page)

- [[Deposit Media]] `M.T.I, num → —`
- [[Withdraw Media]] `M.T.I, num → —`

## Sources

- 📖 **Guidebook:** [[Media Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetThatMedia` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_mdia`
