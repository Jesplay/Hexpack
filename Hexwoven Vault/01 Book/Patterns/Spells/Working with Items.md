---
aliases: ["Working with Items"]
tags: [hexbook/entry, mod/hex-casting, category/spells]
mod: "Hex Casting"
category: "Spells"
entry_id: "hexcasting:patterns/spells/itempicking"
---

# Working with Items

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hex Casting]]

Certain spells, such as [[Place Block]], will consume additional items from my inventory. When this happens, the spell will first look for the item to use, and then draw from all such items in my inventory.

This process is called "picking an item."

More specifically:
- First, the spell will search for the first valid item in my hotbar to the right of my [[Staves|staff]], wrapping around at the right-hand side, and starting at the first slot if my [[Staves|staff]] is in my off-hand.
- Second, the spell will draw that item from as far back in my inventory as possible, prioritizing the main inventory over the hotbar.

This way, I can keep a "chooser" item on my hotbar to tell the spell what to use, and fill the rest of my inventory with that item to keep the spell well-stocked.


**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/itempicking) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
