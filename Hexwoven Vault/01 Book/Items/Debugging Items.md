---
aliases: ["Debugging Items", "Debugger", "Evaluator"]
tags: [hexbook/entry, mod/hexdebug, category/items]
mod: "HexDebug"
category: "Items"
entry_id: "hexcasting:items/debugging"
---

# Debugging Items

> **Category:** [[Items (Category)|Items]]  
> **Source mod:** [[HexDebug]]

I have read vague descriptions of a strange object used by the ancients to help discover issues in complex **Hexes**. I believe I have managed to reproduce this item; I call it the **Debugger**.

It seems to work in a similar way to [[Casting Items|Artifacts]]. However, I can also use it to cast a **Hex** right from an item (like a [[Focus]]) in my off-hand.

Unfortunately, the **Debugger** is not very useful on its own; to get the most out of it, I will need to [set up](https://github.com/object-Object/HexDebug/wiki/Setting-up-VSCode-with-HexDebug) some sort of [external tool](https://marketplace.visualstudio.com/items?itemName=object-Object.hex-casting).

**Debugger**: shaped crafting

|   |   |   |
|---|---|---|
|   | `C` | `C` |
|   | `U` | `C` |
| `L` |   |   |

`C` = Charged Amethyst · `U` = Artifact · `L` = Gold Ingot

Thwack!

I can also switch between different "step modes" of the **Debugger** by sneak-scrolling (like with a [[Spellbook]] or [[Abacus]]).

Complementing the **Debugger**, I've also created a new type of [[Staves|staff]]: the **Evaluator**.

While debugging a **Hex**, patterns drawn with this staff will be cast using the current [[The Stack|stack]] and [[Reading and Writing|ravenmind]] of the **Debugger**.

Clearing the grid seems to reset the **Debugger** to just before the first pattern was drawn with the **Evaluator**. It will not, of course, undo any effects performed by spells on the world.

**Evaluator**: shaped crafting

|   |   |   |
|---|---|---|
|   | `C` | `C` |
|   | `U` | `C` |
| `L` |   |   |

`C` = Charged Amethyst · `U` = Block of Slate · `L` = Block of Slate

Thwonk!


**Sources:** 🌐 [Official online docs](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#items/debugging) · 🧩 [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
