---
aliases: ["Manifest Relay"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Casting Devices"]
mod: "HexIntent"
op_id: "manifestation:link_intent_relay"
signature: "edeweqaq"
start_direction: "SOUTH_WEST"
input: "vec, vec, vec, num?"
output: "none"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Manifest Relay (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifest Relay

◇ **Pattern** · [[HexIntent]] · 📖 [[Manifest Relay (Book Entry)]] · Patterns › Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="71" viewBox="0 0 170.0 71.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/></g><polyline class="hexpat-base" points="69.3,22.0 53.5,49.3 22.0,49.3 37.8,22.0 69.3,22.0 100.7,22.0 116.5,49.3 148.0,49.3 132.2,22.0 100.7,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:283;--dur:2.5s" points="69.3,22.0 53.5,49.3 22.0,49.3 37.8,22.0 69.3,22.0 100.7,22.0 116.5,49.3 148.0,49.3 132.2,22.0 100.7,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="22.0" x2="64.2" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,37.0 61.2,29.0 67.2,32.5" fill="#fc77be"/><circle cx="69.3" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeweqaq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec, vec, num?` ([[Number]], [[Vector]])
- **Leaves:** `none` (—)
- **ID:** `manifestation:link_intent_relay`

```text
before   … │ vec │ vec │ vec │ num?  ◂ top
after    … │ none                    ◂ top
```

## Description

Takes a destination vector for where the relay will go, then a face vector to choose which side to manifest it on, and then a target vector for the thing to link the relay to.

Optionally you can specify a power up to 15 as the final argument to emit a redstone signal when the relay is activated.
Activating that relay will proxy the caster's interaction to the target as if they were interacting with it themselves.

Be aware that some things require the presence of a person to work fully, so the dream of opening chests from across the world will have to stay a dream.

## Sources

- 📖 **Guidebook:** [[Manifest Relay (Book Entry)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpLinkIntentRelay` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:link_intent_relay`
