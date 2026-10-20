---
aliases: ["How to Read This Section"]
tags: [hexbook/entry, mod/hex-casting, category/patterns]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/readers_guide"
---

# How to Read This Section

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

I've divided all the valid patterns I've found into sections based on what they do, more or less. I've written down the stroke order of the patterns as well, if I managed to find it in my studies, with the start of the pattern marked with a red dot.

If an action is cast by multiple patterns, as is the case with some, I'll write them all side-by-side.

For a few patterns, however, I was not able to find the stroke order, just the shape. I suspect the order to draw them in are out there, locked away in the ancient libraries and dungeons of the world.

In such cases I just draw the pattern without any information on the order to draw it in.

I also write the types of [[Iotas|iota]] that the action will consume or modify, a "→", and the types of iota the action will create.

For example, "**vector, number** → **vector**" means the action will remove a vector and a number from the top of the [[The Stack|stack]], and then add a vector; or, put another way, will remove a number from the stack, and then modify the vector at the top of the stack. (The number needs to be on the top of the stack, with the vector right below it.)

"→ **entity**" means it'll just push an entity. "**entity, vector** →" means it removes an entity and a vector, and doesn't push anything.

Finally, if I find the little dot marking the stroke order too slow or confusing, I can press **Control/Command** to display a gradient, where the start of the pattern is darkest and the end is lightest. This works on scrolls and when casting, too!


**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readers_guide) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
