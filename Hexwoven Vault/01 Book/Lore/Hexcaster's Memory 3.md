---
aliases: ["Hexcaster's Memory #3"]
tags: [hexbook/entry, mod/oneironaut, category/lore]
mod: "Oneironaut"
category: "Lore"
entry_id: "hexcasting:lore/treatise3"
---

# Hexcaster's Memory #3

> **Category:** [[Lore (Category)|Lore]]  
> **Source mod:** [[Oneironaut]]

While [[Thoth's Gambit]] is perfectly-suited for casting something on many targets at once, using to it cast something on one target many times can get awkward, and short of completely halting the hex, it is not possible to add or remove targets once Thoth has begun executing.
To get around these limitations, one simply needs to use a different sort of loop.

### While Loop

A While loop is a loop that continues executing until some condition is fulfilled, such as an entity's target velocity having been reached. The most common way to accomplish this is creating a pattern list which duplicates and executes the top [[Iotas|iota]] on the [[The Stack|stack]], duplicating it, then executing it. On its own this simply produces an infinite loop which does nothing but [[Mishaps|mishap]].

However, by adding actual functionality and a bit of boolean logic, you can create a proper while loop. The code for the aforementioned infinite loop is as follows:
Introspection
[[Gemini Decomposition]]
[[Hermes' Gambit]]
Retrospection
Gemini Decomposition
Hermes' Gambit

By adding logic that changes whether the inner Hermes' Gambit gets evaluated based on a condition, you can make it stop once that condition is fulfilled.

### Loop Unrolling

A more complex form of loop is an unrolled loop. Like [[Thoth's Gambit]], it's created with a fixed number of iterations which can't easily be added to, but [[Charon's Gambit]] can be used to halt it early if need be. It also has the advantage that when done competently, it only ever produces one or two levels of recursion depth (not counting anything executed in the body of the loop). Loop unrolling is accomplished by creating a pattern list you want to repeat, duplicating it many times, and then combining all those lists into one.

The simplest way to accomplish this is to use [[Gemini Decomposition]] and then [[Additive Distillation|Combination Distillation]], resulting in a doubled pattern list. This method can be repeated to achieve greater powers of 2, and only produces a single level of recursion depth, when the list is executed. A more flexible method is to use [[Gemini Gambit]] to produce *n* copies of the list, then gathering *n-1* instances of the pattern for Combination Distillation into one list and executing it, thus merging all the copies of your original list into one.


**Sources:** 🌐 [Official online docs](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#lore/treatise3)
