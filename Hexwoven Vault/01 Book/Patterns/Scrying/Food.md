---
aliases: ["Food"]
tags: [hexbook/entry, mod/hexpose, category/scrying, topic/sensing]
mod: "Hexpose"
category: "Scrying"
entry_id: "hexcasting:patterns/scrying/food"
---

# Food

> **Category:** Patterns › [[Scrying (Category)|Scrying]]  
> **Source mod:** [[Hexpose]]

## [[Hunger Purification|Hunger Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,16.2 27.7,16.2 16.2,36.1 27.7,56.1 50.8,56.1 39.2,36.1 62.3,36.1 50.8,16.2 73.8,16.2 62.3,36.1 50.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,16.2 27.7,16.2 16.2,36.1 27.7,56.1 50.8,56.1 39.2,36.1 62.3,36.1 50.8,16.2 73.8,16.2 62.3,36.1 50.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="16.2" x2="43.4" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="38.1,16.2 43.4,13.6 43.4,18.7" fill="#fc77be"/><circle cx="50.8" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`player entity` → `number`

Pushes how full a **Player** is. I can think of this in terms of half-hunches.

## [[Stamina Purification|Stamina Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,16.2 27.7,16.2 16.2,36.1 27.7,56.1 50.8,56.1 39.2,36.1 62.3,36.1 50.8,16.2 73.8,16.2 62.3,36.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,16.2 27.7,16.2 16.2,36.1 27.7,56.1 50.8,56.1 39.2,36.1 62.3,36.1 50.8,16.2 73.8,16.2 62.3,36.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="16.2" x2="43.4" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="38.1,16.2 43.4,13.6 43.4,18.7" fill="#fc77be"/><circle cx="50.8" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`entity entity` → `number`

Pushes a **Player**'s saturation, which roughly correlates to how long the **Player** will remain full.

## [[Edibility Purification|Edibility Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,16.2 27.7,16.2 39.2,36.1 16.2,36.1 27.7,56.1 50.8,56.1 62.3,36.1 50.8,16.2 73.8,16.2 62.3,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:208;--dur:2.5s" points="50.8,16.2 27.7,16.2 39.2,36.1 16.2,36.1 27.7,56.1 50.8,56.1 62.3,36.1 50.8,16.2 73.8,16.2 62.3,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="16.2" x2="43.4" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="38.1,16.2 43.4,13.6 43.4,18.7" fill="#fc77be"/><circle cx="50.8" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`identifier` → `boolean`

Takes in an item identifier and pushes whether I can eat it.

## [[Calorie Purification|Calorie Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,15.8 27.0,15.8 38.3,35.3 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 72.0,15.8 60.8,35.3 72.0,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="49.5,15.8 27.0,15.8 38.3,35.3 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 72.0,15.8 60.8,35.3 72.0,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="15.8" x2="42.3" y2="15.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,15.8 42.3,13.3 42.3,18.2" fill="#fc77be"/><circle cx="49.5" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`identifier` → `number`

Takes in an item identifier and pushes the amount it will fill me.

## [[Satiation Purification|Satiation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 24.5,14.3 34.8,32.0 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 45.0,14.3 65.5,14.3 55.2,32.0 65.5,49.7 75.7,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="45.0,14.3 24.5,14.3 34.8,32.0 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 45.0,14.3 65.5,14.3 55.2,32.0 65.5,49.7 75.7,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="38.5" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.8,14.3 38.5,12.1 38.5,16.6" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`identifier` → `number`

Takes in an item identifier and pushes the saturation value, which roughly affects how long it will fill me.

## [[Flesh Purification|Flesh Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="40.4,12.9 22.0,12.9 31.2,28.8 12.9,28.8 22.0,44.7 40.4,44.7 49.6,28.8 40.4,12.9 58.8,12.9 49.6,28.8 68.0,28.8 77.1,44.7 58.8,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:3.4s" points="40.4,12.9 22.0,12.9 31.2,28.8 12.9,28.8 22.0,44.7 40.4,44.7 49.6,28.8 40.4,12.9 58.8,12.9 49.6,28.8 68.0,28.8 77.1,44.7 58.8,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="12.9" x2="34.5" y2="12.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.3,12.9 34.5,10.8 34.5,14.9" fill="#fc77be"/><circle cx="40.4" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`identifier` → `boolean`

Takes in an item identifier and pushes whether it's considered a meat. **Wolves** can eat meats.

## [[Dessert Purification|Dessert Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="40.4,12.9 22.0,12.9 31.2,28.8 12.9,28.8 22.0,44.7 40.4,44.7 49.6,28.8 40.4,12.9 58.8,12.9 49.6,28.8 68.0,28.8 77.1,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:202;--dur:3.1s" points="40.4,12.9 22.0,12.9 31.2,28.8 12.9,28.8 22.0,44.7 40.4,44.7 49.6,28.8 40.4,12.9 58.8,12.9 49.6,28.8 68.0,28.8 77.1,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="12.9" x2="34.5" y2="12.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.3,12.9 34.5,10.8 34.5,14.9" fill="#fc77be"/><circle cx="40.4" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`identifier` → `boolean`

Takes in an item identifier and pushes whether it's a snack.

---
### Patterns on this page

- [[Hunger Purification]] · Pattern · Purification
- [[Stamina Purification]] · Pattern · Purification
- [[Edibility Purification]] · Pattern · Purification
- [[Calorie Purification]] · Pattern · Purification
- [[Satiation Purification]] · Pattern · Purification
- [[Flesh Purification]] · Pattern · Purification
- [[Dessert Purification]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/food) · 🧩 [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
