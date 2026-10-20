---
aliases: ["Patterned Assembler Patterns"]
tags: [hexbook/entry, mod/slate-works, category/spells, topic/circles, topic/items]
mod: "Slate Works"
category: "Spells"
entry_id: "hexcasting:patterns/spells/crafting_loci"
---

# Patterned Assembler Patterns

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Slate Works]]

While the [[Patterned Assemblers|Patterned Assembler]] is quite accessible without a **[[Staves|staff]]**, it does become extreme powerful with **spells**. I have listed all the spells that interact with the [[Patterned Assemblers|Patterned Assembler]] here.

*AE2? Never heard of it.*

## [[Set Recipe|Set Recipe]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 20.0,55.0 36.7,55.0 53.3,55.0 70.0,55.0 61.7,40.5 78.3,40.5 70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:6.0s" points="45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 20.0,55.0 36.7,55.0 53.3,55.0 70.0,55.0 61.7,40.5 78.3,40.5 70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="11.7" x2="42.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,19.6 40.7,15.4 43.9,17.2" fill="#fc77be"/><circle cx="45.0" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`vec, [item stack|item type|null]` → `—`

Sets the crafting recipe of a [[Patterned Assemblers|Patterned Assembler]]. Takes a list of Nulls, items, or item variants, and applies it to the targeted [[Patterned Assemblers|Assembler]].

To note: the given list gets "formatted" as left to right, top to bottom in the [[Patterned Assemblers|Patterned Assembler]]. As well, the Nulls represent blank spots in recipes. Finally, the Spell will [[Mishaps|mishap]] if the given list is longer than 9 items (but passes with less than or equal to 9 items).

Does not mishap if cast outside of a [[Spell Circles|Spell Circle]]. Costs a negligible amount of **[[Media|media]]**.

## [[Set Craft Count|Set Craft Count]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/><circle cx="11.7" cy="55.0" r="1.6"/><circle cx="28.3" cy="55.0" r="1.6"/><circle cx="45.0" cy="55.0" r="1.6"/><circle cx="61.7" cy="55.0" r="1.6"/><circle cx="78.3" cy="55.0" r="1.6"/><circle cx="3.3" cy="69.4" r="1.6"/><circle cx="20.0" cy="69.4" r="1.6"/><circle cx="36.7" cy="69.4" r="1.6"/><circle cx="53.3" cy="69.4" r="1.6"/><circle cx="70.0" cy="69.4" r="1.6"/><circle cx="86.7" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5 11.7,55.0 28.3,55.0 20.0,69.4 36.7,69.4 53.3,69.4 70.0,69.4 61.7,55.0 78.3,55.0 70.0,40.5 78.3,26.1 61.7,26.1 45.0,26.1 36.7,11.7 53.3,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:433;--dur:6.0s" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5 11.7,55.0 28.3,55.0 20.0,69.4 36.7,69.4 53.3,69.4 70.0,69.4 61.7,55.0 78.3,55.0 70.0,40.5 78.3,26.1 61.7,26.1 45.0,26.1 36.7,11.7 53.3,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="42.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,34.0 40.7,29.8 43.9,31.6" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vec, number` → `—`

This sets how many times the [[Patterned Assemblers|Patterned Assembler]] will craft a recipe.

## [[Get Craft Count|Get Craft Count]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/><circle cx="11.7" cy="55.0" r="1.6"/><circle cx="28.3" cy="55.0" r="1.6"/><circle cx="45.0" cy="55.0" r="1.6"/><circle cx="61.7" cy="55.0" r="1.6"/><circle cx="78.3" cy="55.0" r="1.6"/><circle cx="3.3" cy="69.4" r="1.6"/><circle cx="20.0" cy="69.4" r="1.6"/><circle cx="36.7" cy="69.4" r="1.6"/><circle cx="53.3" cy="69.4" r="1.6"/><circle cx="70.0" cy="69.4" r="1.6"/><circle cx="86.7" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5 11.7,55.0 28.3,55.0 20.0,69.4 36.7,69.4 53.3,69.4 70.0,69.4 61.7,55.0 78.3,55.0 70.0,40.5 78.3,26.1 61.7,26.1 45.0,26.1 53.3,11.7 36.7,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:433;--dur:6.0s" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5 11.7,55.0 28.3,55.0 20.0,69.4 36.7,69.4 53.3,69.4 70.0,69.4 61.7,55.0 78.3,55.0 70.0,40.5 78.3,26.1 61.7,26.1 45.0,26.1 53.3,11.7 36.7,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="42.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,34.0 40.7,29.8 43.9,31.6" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vec` → `number`

This gets how many times the [[Patterned Assemblers|Patterned Assembler]] will craft a recipe.

---
### Patterns on this page

- [[Set Recipe]] · Pattern · Other
- [[Set Craft Count]] · Pattern · Other
- [[Get Craft Count]] · Pattern · Other

**Sources:** 🧩 [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
