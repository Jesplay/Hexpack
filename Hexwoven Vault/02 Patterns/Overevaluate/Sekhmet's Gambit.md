---
aliases: ["Sekhmet's Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Overevaluate"
op_id: "overevaluate:sekhmet"
signature: "qaqdd"
start_direction: "SOUTH_WEST"
input: ""
output: "varies"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Extra Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sekhmet's Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Extra Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="146.5" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,38.0 38.0,85.0 65.1,132.0 92.3,85.0 65.1,38.0 119.4,38.0 92.3,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="65.1,38.0 38.0,85.0 65.1,132.0 92.3,85.0 65.1,38.0 119.4,38.0 92.3,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="38.0" x2="56.4" y2="53.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.2,63.8 51.3,50.1 61.6,56.0" fill="#fc77be"/><circle cx="65.1" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqdd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `varies` (—)
- **ID:** `overevaluate:sekhmet`

## Description

Voids the entire [[The Stack|stack]] except for the top n [[Iotas|iota]], determined by tail length. In the examples above, zero, one, and two iota are left untouched respectively.

This simple gambit serves as a convenient wrapper around a specific combination of [[Flock's Reflection]], [[Flock's Gambit]], [[Selection Exaltation]], and [[Flock's Disintegration]]. It also pairs extremely well with [[Thoth's Gambit]] to leave only the iotas I am interested in on the pseudo-stack.

*Everything is possible, but not everything is necessary.*

## Related patterns (same book page)

- [[Geb's Gambit]] `many → many`
- [[Nut's Gambit]] `many → many`
- [[Reflecting Gambit]] `any, any, any → any, any, any`
- [[Bubbling Gambit]] `any, any, any → any, any, any`
- [[Dioscuri Gambit II]] `many, number → many`

## Sources

- 📖 **Guidebook:** [[Extra Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sekhmet's Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#extra_stack@overevaluate:sekhmet)
- ✏️ **Strokes:** from the book page, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSekhmet` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:sekhmet`
