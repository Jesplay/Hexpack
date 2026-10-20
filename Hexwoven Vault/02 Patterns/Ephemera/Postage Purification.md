---
aliases: ["Postage Purification"]
tags: [hexcasting/pattern, mod/ephemera, family/purification, topic/text]
kind: "Pattern"
family: "Purification"
topics: ["Text, Strings & Chat"]
mod: "Ephemera"
op_id: "ephemera:getrevealcost"
signature: "qdeaaqqqqq"
start_direction: "EAST"
input: "Player"
output: "num"
cost: "cost to send them a message via Postman's Gambit, between 1/10 of a dust and 10 dust."
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Messaging]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Postage Purification

◇ **Pattern** · [[Ephemera]] · 📖 [[Messaging]] · Patterns › Spells

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,66.6 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 136.1,103.4 114.8,66.6 72.3,66.6 51.0,103.4 72.3,140.2 114.8,140.2 136.1,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="29.8,66.6 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 136.1,103.4 114.8,66.6 72.3,66.6 51.0,103.4 72.3,140.2 114.8,140.2 136.1,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="66.6" x2="43.4" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="53.2,66.6 43.4,71.3 43.4,61.9" fill="#fc77be"/><circle cx="29.8" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qdeaaqqqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `Player` ([[Entity]])
- **Leaves:** `num` ([[Number]])
- **Cost:** cost to send them a message via Postman's Gambit, between 1/10 of a dust and 10 dust.
- **ID:** `ephemera:getrevealcost`

```text
before   … │ Player  ◂ top
after    … │ num     ◂ top
```

## Description

Accepts a player and returns how much amethyst dust it would cost to send them a message via [[Postman's Gambit]], between 1/10 of a dust and 10 dust.

## Related patterns (same book page)

- [[Postman's Gambit]] `Player, any → —`
- [[Postman's Gambit II]] `— → —`

## Sources

- 📖 **Guidebook:** [[Messaging]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Postage Purification](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/reveal_other@ephemera:getrevealcost)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetTransmitCost` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getrevealcost`
