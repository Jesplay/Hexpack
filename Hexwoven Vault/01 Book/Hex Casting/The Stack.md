---
aliases: ["The Stack"]
tags: [hexbook/entry, mod/hex-casting, category/hex-casting]
mod: "Hex Casting"
category: "Hex Casting"
entry_id: "hexcasting:casting/stack"
---

# The Stack

> **Category:** [[Hex Casting (Category)|Hex Casting]]  
> **Source mod:** [[Hex Casting]]

A **Stack**, also known as a "LIFO", is a concept borrowed from computer science. In short, it's a collection of things designed so that you can only interact with the most recently used thing.

Think of a stack of plates, where new plates are added to the top: if you want to interact with a plate halfway down the stack, you have to remove the plates above it in order to get ahold of it.

Because a stack is so simple, there's only so many things you can do with it:
- Adding something to it, known formally as pushing,
- Removing the last added element, known as popping, or
- Examining or modifying the last added element, known as peeking.

We call the last-added element the "top" of the stack, in accordance with the dinner plate analogy.

As an example, if we push 1 to a stack, then push 2, then pop, the top of the stack is now 1.

Actions are (on the most part) restricted to interacting with the casting stack in these ways. They will pop some [[Iotas|iotas]] they're interested in (known as "arguments" or "parameters"), process them, and push some number of results.

Of course, some actions (e.g. [[Mind's Reflection]]) might pop no arguments, and some actions (particularly spells) might push nothing afterwards.

Even more complicated actions can be expressed in terms of pushing, popping, and peeking. For example, [[Jester's Gambit]] swaps the top two items of the stack. This can be thought of as popping two items and pushing them in opposite order. For another, [[Gemini Decomposition]] duplicates the top of the stack-- in other words, it peeks the stack and pushes a copy of what it finds.


**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#casting/stack) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
