---
aliases: ["Sentinel Cache Patterns"]
tags: [hexbook/entry, mod/slate-works, category/spells, topic/circles, topic/sentinels]
mod: "Slate Works"
category: "Spells"
entry_id: "hexcasting:patterns/spells/sentinel_loci_patterns"
---

# Sentinel Cache Patterns

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Slate Works]]

[[Sentinel Cache|Sentinel Caches]] have an odd method of interaction compared to their non-[[Spell Circles|Spell Circle]] [[Summon Greater Sentinel (Book Entry)|counter parts]]. They are interfaced via a list of vectors, opposed to a single vector.

These all [[Enlightened Mishaps|mishap when not cast in a Spell Circle]].

*It's doing physical and mental damage to it, as if it had body parts and organs and memories being consumed as **[[Media|Media]]***. **

*It burns its life story for fuel. *

*It burns its fingers and toes and its attachments to the ground until there's nothing but pure cognition.*

## [[Apply Pseudosentinels|Apply Pseudosentinels]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="24.5,49.7 45.0,49.7 65.5,49.7 55.2,32.0 65.5,14.3 45.0,14.3 24.5,14.3 34.8,32.0 24.5,49.7 14.3,67.5 34.8,67.5 55.2,67.5 75.7,67.5 65.5,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:266;--dur:3.6s" points="24.5,49.7 45.0,49.7 65.5,49.7 55.2,32.0 65.5,14.3 45.0,14.3 24.5,14.3 34.8,32.0 24.5,49.7 14.3,67.5 34.8,67.5 55.2,67.5 75.7,67.5 65.5,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="49.7" x2="31.1" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.8,49.7 31.1,52.0 31.1,47.5" fill="#fc77be"/><circle cx="24.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`[vectors]` → `—`

This takes a list of vectors, and attempts to move the Pseudosentinels to the positions.

Helpfully, the list of vectors from **[[Locate Pseudosentinels]]** will always match the order of **[[Apply Pseudosentinels]]**, meaning I need not worry about the ordering.

This will [[Mishaps|mishap]] if the given list is larger than the current awoken [[Sentinel Cache|Cache]] amount. Costs 1 [[Amethyst|dust]] to command the Pseudosentinel, and due to fatigue, an extra 1/8th of a [[Amethyst|dust]] (stacking) for each Pseudosentinel that was already moved in the same instance.

## [[Locate Pseudosentinels|Locate Pseudosentinels]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="24.5,49.7 45.0,49.7 65.5,49.7 55.2,32.0 65.5,14.3 45.0,14.3 24.5,14.3 34.8,32.0 24.5,49.7 14.3,67.5 34.8,67.5 55.2,67.5 75.7,67.5 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:307;--dur:4.2s" points="24.5,49.7 45.0,49.7 65.5,49.7 55.2,32.0 65.5,14.3 45.0,14.3 24.5,14.3 34.8,32.0 24.5,49.7 14.3,67.5 34.8,67.5 55.2,67.5 75.7,67.5 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="49.7" x2="31.1" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.8,49.7 31.1,52.0 31.1,47.5" fill="#fc77be"/><circle cx="24.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`—` → `[vectors]`

This retrieves the current positions of all Pseudosentinels in the order they were awoken. Free to cast.

---
### Patterns on this page

- [[Apply Pseudosentinels]] · Pattern · Other
- [[Locate Pseudosentinels]] · Pattern · Other

**Sources:** 🧩 [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
