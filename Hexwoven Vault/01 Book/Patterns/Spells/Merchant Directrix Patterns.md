---
aliases: ["Merchant Directrix Patterns"]
tags: [hexbook/entry, mod/slate-works, category/spells, topic/circles, topic/items]
mod: "Slate Works"
category: "Spells"
entry_id: "hexcasting:patterns/spells/trade_loci"
---

# Merchant Directrix Patterns

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Slate Works]]

## [[Merchant's Purification|Merchant's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,45.0 31.5,61.0 13.0,61.0 22.2,77.0 40.7,77.0 59.2,77.0 68.5,61.0 50.0,61.0 40.7,45.0 31.5,29.0 40.7,13.0 50.0,29.0 40.7,45.0 59.2,45.0 50.0,61.0 40.7,77.0 31.5,61.0 22.2,45.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:333;--dur:5.0s" points="40.7,45.0 31.5,61.0 13.0,61.0 22.2,77.0 40.7,77.0 59.2,77.0 68.5,61.0 50.0,61.0 40.7,45.0 31.5,29.0 40.7,13.0 50.0,29.0 40.7,45.0 59.2,45.0 50.0,61.0 40.7,77.0 31.5,61.0 22.2,45.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="45.0" x2="37.7" y2="50.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.6,53.8 36.0,49.1 39.5,51.1" fill="#fc77be"/><circle cx="40.7" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`vec` → `complicated`

Despite the return of this spell, its not extremely complicated... I think. This takes a [[Merchant Directrix]], and returns: \[\[**Item [[The Stack|Stack]]**, **Item Stack**, **Item Stack**, **Number**\]...\]. Free to cast.

Where the first [[Items|Item Stack]] is the item(s) the Villager will buy; second [[Items|Item Stack]] is another set of item(s) the Villager may want to buy; the 3rd [[Items|Item Stack]] is the item will the Villager sell to me; and the **Number** is ratio between the times the trade has been done (since last restock), and the max times it can be done. If the **Number** is at one, the trade can no longer be traded.

## [[Exchange Mind|Exchange Mind]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/><circle cx="11.7" cy="55.0" r="1.6"/><circle cx="28.3" cy="55.0" r="1.6"/><circle cx="45.0" cy="55.0" r="1.6"/><circle cx="61.7" cy="55.0" r="1.6"/><circle cx="78.3" cy="55.0" r="1.6"/><circle cx="3.3" cy="69.4" r="1.6"/><circle cx="20.0" cy="69.4" r="1.6"/><circle cx="36.7" cy="69.4" r="1.6"/><circle cx="53.3" cy="69.4" r="1.6"/><circle cx="70.0" cy="69.4" r="1.6"/><circle cx="86.7" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 36.7,40.5 45.0,55.0 53.3,40.5 61.7,26.1 45.0,26.1 53.3,11.7 61.7,26.1 70.0,40.5 78.3,55.0 61.7,55.0 53.3,69.4 36.7,69.4 28.3,55.0 11.7,55.0 20.0,40.5 28.3,26.1 36.7,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:6.0s" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 36.7,40.5 45.0,55.0 53.3,40.5 61.7,26.1 45.0,26.1 53.3,11.7 61.7,26.1 70.0,40.5 78.3,55.0 61.7,55.0 53.3,69.4 36.7,69.4 28.3,55.0 11.7,55.0 20.0,40.5 28.3,26.1 36.7,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="42.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,34.0 40.7,29.8 43.9,31.6" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vector, entity` → `—`

This feels... [[Flay Mind (Book Entry)|familiar]]. This spell takes a Villager, and a [[Merchant Directrix]] and exchanges their Jobs. Costs 1 [[Amethyst|Charged Amethyst]] to reapply the knowledge of Working.

## [[Induce Restock|Induce Restock]] ⭐ *Great Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="85" height="90" viewBox="0 0 84.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.0" cy="11.0" r="1.6"/><circle cx="26.7" cy="11.0" r="1.6"/><circle cx="42.4" cy="11.0" r="1.6"/><circle cx="58.1" cy="11.0" r="1.6"/><circle cx="73.8" cy="11.0" r="1.6"/><circle cx="3.1" cy="24.6" r="1.6"/><circle cx="18.8" cy="24.6" r="1.6"/><circle cx="34.6" cy="24.6" r="1.6"/><circle cx="50.3" cy="24.6" r="1.6"/><circle cx="66.0" cy="24.6" r="1.6"/><circle cx="81.7" cy="24.6" r="1.6"/><circle cx="11.0" cy="38.2" r="1.6"/><circle cx="26.7" cy="38.2" r="1.6"/><circle cx="42.4" cy="38.2" r="1.6"/><circle cx="58.1" cy="38.2" r="1.6"/><circle cx="73.8" cy="38.2" r="1.6"/><circle cx="3.1" cy="51.8" r="1.6"/><circle cx="18.8" cy="51.8" r="1.6"/><circle cx="34.6" cy="51.8" r="1.6"/><circle cx="50.3" cy="51.8" r="1.6"/><circle cx="66.0" cy="51.8" r="1.6"/><circle cx="81.7" cy="51.8" r="1.6"/><circle cx="11.0" cy="65.4" r="1.6"/><circle cx="26.7" cy="65.4" r="1.6"/><circle cx="42.4" cy="65.4" r="1.6"/><circle cx="58.1" cy="65.4" r="1.6"/><circle cx="73.8" cy="65.4" r="1.6"/><circle cx="3.1" cy="79.0" r="1.6"/><circle cx="18.8" cy="79.0" r="1.6"/><circle cx="34.6" cy="79.0" r="1.6"/><circle cx="50.3" cy="79.0" r="1.6"/><circle cx="66.0" cy="79.0" r="1.6"/><circle cx="81.7" cy="79.0" r="1.6"/></g><polyline class="hexpat-base" points="42.4,38.2 34.6,51.8 18.8,51.8 26.7,65.4 42.4,65.4 58.1,65.4 66.0,51.8 50.3,51.8 42.4,38.2 26.7,38.2 11.0,38.2 18.8,51.8 11.0,65.4 26.7,65.4 34.6,79.0 50.3,79.0 58.1,65.4 73.8,65.4 66.0,51.8 73.8,38.2 58.1,38.2 42.4,38.2 34.6,24.6 50.3,24.6 42.4,11.0 34.6,24.6 26.7,38.2 34.6,51.8 42.4,65.4 50.3,51.8 58.1,38.2 50.3,24.6 42.4,38.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:503;--dur:6.0s" points="42.4,38.2 34.6,51.8 18.8,51.8 26.7,65.4 42.4,65.4 58.1,65.4 66.0,51.8 50.3,51.8 42.4,38.2 26.7,38.2 11.0,38.2 18.8,51.8 11.0,65.4 26.7,65.4 34.6,79.0 50.3,79.0 58.1,65.4 73.8,65.4 66.0,51.8 73.8,38.2 58.1,38.2 42.4,38.2 34.6,24.6 50.3,24.6 42.4,11.0 34.6,24.6 26.7,38.2 34.6,51.8 42.4,65.4 50.3,51.8 58.1,38.2 50.3,24.6 42.4,38.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="42.4" y1="38.2" x2="39.9" y2="42.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="38.1,45.7 38.4,41.7 41.4,43.4" fill="#fc77be"/><circle cx="42.4" cy="38.2" r="2.6" fill="#fc77be"/></svg>

`vector` → `—`

While digging amongst archives, I had oddly found this Great Spell. It takes a position of a [[Merchant Directrix]], and forces it to restock; like the day had began or ended. Costs 5 [[Amethyst|Charged Amethyst]].

---
### Patterns on this page

- [[Merchant's Purification]] · Pattern · Purification
- [[Exchange Mind]] · Spell · Spell
- [[Induce Restock]] · Great Spell · Great Spell

**Sources:** 🧩 [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
