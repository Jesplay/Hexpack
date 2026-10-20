---
aliases: ["Saturn's Gambit"]
tags: [hexcasting/pattern, mod/hexintent, family/gambit, topic/devices]
kind: "Pattern"
family: "Gambit"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:saturns_gambit"
signature: "deaqqqded"
start_direction: "SOUTH_EAST"
input: "pattern"
output: "none"
cost: ""
iotas_in: ["[[Pattern]]"]
iotas_out: []
entry: "[[Splinters]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Saturn's Gambit

◇ **Pattern** · [[HexIntent]] · 📖 [[Splinters]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,29.8 114.8,66.6 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 136.1,103.4 114.8,140.2 93.5,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="93.5,29.8 114.8,66.6 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 136.1,103.4 114.8,140.2 93.5,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="29.8" x2="100.3" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.2,50.0 96.3,43.9 104.4,39.2" fill="#fc77be"/><circle cx="93.5" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deaqqqded` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern` ([[Pattern]])
- **Leaves:** `none` (—)
- **ID:** `manifestation:saturns_gambit`

```text
before   … │ pattern  ◂ top
after    … │ none     ◂ top
```

## Description

Like [[Hermes' Gambit]] but executes it asynchronously, splitting up large lists of work over multiple ticks that execute synchronously.

It borrows the technique from Splinters but is not a Splinter.

## Related patterns (same book page)

- [[Manifest Splinter]] `vec, num, list → none`
- [[Renew Splinter]] `vec, num → none`
- [[Get Splinter Location]] `— → vec`
- [[Destroy Current Splinter]] `— → none`
- [[Destroy Splinters]] `— → none`

## Sources

- 📖 **Guidebook:** [[Splinters]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSaturnsGambit` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:saturns_gambit`
