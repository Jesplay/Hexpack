---
aliases: ["Sets"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/lists]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/sets"
---

# Sets

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

Set operations are odd, in that some of them can accept two numbers or two lists, but not a combination thereof. Such arguments will be written as "(num, num)|(list, list)".

When numbers are used in those operations, they are being used as so-called binary "bitsets", lists of 1 and 0, true and false, "on" and "off".

## [[Disjunction Distillation|Disjunction Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="18.5" x2="22.8" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,31.1 20.2,27.3 25.3,24.4" fill="#fc77be"/><circle cx="18.5" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`(num, num)|(list, list)` → `num|list`

Unifies two sets.

As such:
- With two numbers at the top of the [[The Stack|stack]], combines them into a bitset containing every "on" bit in either bitset.
- With two lists, this creates a list of every element from the first list, plus every element from the second list that is not in the first list. This is somewhat similar to [[Additive Distillation]].

## [[Conjunction Distillation|Conjunction Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,64.4 31.8,41.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="18.5,64.4 31.8,41.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="64.4" x2="22.8" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,51.8 25.3,58.5 20.2,55.6" fill="#fc77be"/><circle cx="18.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`(num, num)|(list, list)` → `num|list`

Takes the intersection of two sets.

As such:
- With two numbers at the top of the stack, combines them into a bitset containing every "on" bit present in both bitsets.
- With two lists, this creates a list of every element from the first list that is also in the second list.

## [[Exclusion Distillation|Exclusion Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="31.8,64.4 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="27.5" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,51.8 30.1,55.6 25.0,58.5" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`(num, num)|(list, list)` → `num|list`

Takes the exclusive disjunction of two sets.

As such:
- With two numbers at the top of the stack, combines them into a bitset containing every "on" bit present in exactly one of the bitsets.
- With two lists, this creates a list of every element in both lists that is not in the other list.

## [[Negation Purification|Negation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="31.8,41.5 18.5,18.5 45.0,18.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:79;--dur:1.6s" points="31.8,41.5 18.5,18.5 45.0,18.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="41.5" x2="27.5" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,28.8 30.1,32.7 25.0,35.6" fill="#fc77be"/><circle cx="31.8" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`num` → `num`

Takes the inversion of a bitset, changing all "on" bits to "off" and vice versa. In my experience, this will take the form of that number negated and decreased by one. For example, 0 will become -1, and -100 will become 99.

## [[Uniqueness Purification|Uniqueness Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:143;--dur:2.0s" points="65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="49.7" x2="68.7" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,40.0 70.7,45.2 66.8,43.0" fill="#fc77be"/><circle cx="65.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`list` → `list`

Removes duplicate entries from a list.

---
### Patterns on this page

- [[Disjunction Distillation]] · Pattern · Distillation
- [[Conjunction Distillation]] · Pattern · Distillation
- [[Exclusion Distillation]] · Pattern · Distillation
- [[Negation Purification]] · Pattern · Purification
- [[Uniqueness Purification]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/sets) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
