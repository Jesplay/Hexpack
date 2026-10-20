---
aliases: ["Hexcaster's Memory #1"]
tags: [hexbook/entry, mod/oneironaut, category/lore]
mod: "Oneironaut"
category: "Lore"
entry_id: "hexcasting:lore/treatise1"
---

# Hexcaster's Memory #1

> **Category:** [[Lore (Category)|Lore]]  
> **Source mod:** [[Oneironaut]]

While it is possible to produce most [[Iotas|iotas]] on the fly, there are cases where one finds it impractical, or simply cannot (such as very large and precise numbers, or entity references), and as such must obtain the desired iota in another way. Reading it from a [[Focus|Focus]] or a similar item works just fine, but in many cases this would require a third hand, which I do not have. Other methods of fetching iotas from external locations have their own limitations as well.

In cases where no external reference method would work well, one must insert the desired iota into the hex itself. This may seem counterintuive to some, what with the conventional wisdom being that a hex is a list of patterns, but one must remember that patterns are just another type of iota, which [[Hermes' Gambit]] (and similar patterns) works particularly well with.

While you can't just shove a number into the list and expect it to be pushed to the [[The Stack|stack]] as if it was a [[Number Literals|Numerical Reflection]] pattern, there are ways around this.

The simplest (and least reliable) is the [[Consideration]] pattern. Similar to its use when casting freehand, anything that comes immediately after it in a list will be escaped and pushed to the stack when it might otherwise be executed, preventing [[Mishaps|mishaps]] due to execution of non-patterns.

However, when you're more than one exeuction deep in a hex (such as with nested conditional logic, or loops), the number of Considerations required to properly escape an iota increases exponentially, making it unreliable when used in anything that could conceivably be executed by another hex.

🔗 [relevant xkcd](https://xkcd.com/1638/)

A far more reliable method of escaping iotas is to use [[Introspection]] and [[Retrospection]]. When a pattern list is being evaluated and an Introspection is encountered, everything up to (but not including) its paired Retrospection is escaped and pushed to the stack in a list, regardless of iota type. You can then work with this list as you see fit, such as using [[Flock's Disintegration]] to push all of its contents to the stack on their own.

If the contents of your list looks like this:

Introspection
Any iota(s)
Retrospection
[[Flock's Disintegration]]

the state of the stack after all that is executed will simply be all the iotas that were contained within Introspection and Retrospection.


**Sources:** 🌐 [Official online docs](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#lore/treatise1)
