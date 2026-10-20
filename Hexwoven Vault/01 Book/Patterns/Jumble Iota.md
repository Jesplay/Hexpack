---
aliases: ["Jumble Iota"]
tags: [hexbook/entry, mod/overevaluate, category/patterns, topic/data-structures, topic/stack]
mod: "Overevaluate"
category: "Patterns"
entry_id: "hexcasting:jumble"
---

# Jumble Iota

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Overevaluate]]

Jumble [[Iotas|iotas]] are a highly versatile and expressive method of [[The Stack|stack]] manipulation; a single jumble iota can copy, remove, and reorder the stack while being highly readable and instantly understandable.

When a **Hex** tries to process a jumble iota, a number of iotas from the stack indicated by the first number in the jumble iota are popped.

These popped iotas are stored on a temporary list called the reference list. Then, the list of positive integers indicated by the jumble iota are read one by one, indexing into the reference list and pushing the iota to the stack.

The jumble iota ***2 \[1 0\]*** will pop two iotas off the stack. Then, it will push the iota at index 1 of the reference list, followed by the iota at index 0. In practice, it is identical to [[Jester's Gambit]].

## [[Jumbling Gambit|Jumbling Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,74.2 15.8,74.2 27.0,54.7 49.5,54.7 38.3,35.3 15.8,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="38.3,74.2 15.8,74.2 27.0,54.7 49.5,54.7 38.3,35.3 15.8,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="74.2" x2="31.1" y2="74.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.9,74.2 31.1,71.8 31.1,76.7" fill="#fc77be"/><circle cx="38.3" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`number, list` → `jumble`

Creates a jumble iota corresponding to the argument count and the list of numbers.

## [[Jumbling Decomposition|Jumbling Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 15.8,35.3 38.3,35.3 49.5,54.7 27.0,54.7 15.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="27.0,15.8 15.8,35.3 38.3,35.3 49.5,54.7 27.0,54.7 15.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="23.4" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,26.5 21.3,20.8 25.6,23.2" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`jumble` → `number, list`

Dissolves a jumble iota back into a number and a list of numbers.

---
### Patterns on this page

- [[Jumbling Gambit]] · Pattern · Gambit
- [[Jumbling Decomposition]] · Pattern · Decomposition
