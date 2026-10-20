---
aliases: ["Types"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/logic]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/types"
---

# Types

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

> [!note] Two versions of this page
> More than one mod ships a book page with this ID, so the in-game book shows only one of them (depending on load order). The other version: [[Types (MoreIotas)]].

## [[Similarity Distillation (Hex Casting)|Similarity Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,49.7 34.8,49.7 55.2,49.7 45.0,32.0 34.8,14.3 55.2,14.3 75.7,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:123;--dur:1.7s" points="14.3,49.7 34.8,49.7 55.2,49.7 45.0,32.0 34.8,14.3 55.2,14.3 75.7,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="49.7" x2="20.9" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,49.7 20.9,52.0 20.9,47.5" fill="#fc77be"/><circle cx="14.3" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`any, any` → `bool`

Compare the given [[Iotas|iotas]] and push whether they are of the same type.

## [[Dissimilarity Distillation|Dissimilarity Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,14.3 34.8,14.3 55.2,14.3 45.0,32.0 34.8,49.7 55.2,49.7 75.7,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:123;--dur:1.7s" points="14.3,14.3 34.8,14.3 55.2,14.3 45.0,32.0 34.8,49.7 55.2,49.7 75.7,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="14.3" x2="20.9" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,14.3 20.9,16.6 20.9,12.1" fill="#fc77be"/><circle cx="14.3" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`any, any` → `bool`

Compare the given iotas and push whether they are of different types.

## [[Sorter's Distillation|Sorter's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="36.7,26.1 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,26.1 53.3,26.1 61.7,11.7 78.3,11.7 70.0,26.1 78.3,40.5 61.7,40.5 53.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="36.7,26.1 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,26.1 53.3,26.1 61.7,11.7 78.3,11.7 70.0,26.1 78.3,40.5 61.7,40.5 53.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="26.1" x2="34.0" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,18.2 35.6,20.6 32.4,22.4" fill="#fc77be"/><circle cx="36.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`entity, entity` → `bool`

Compare the items represented or held by the given entities and push whether they are the same type of item. Two [[Focus|Foci]] will always compare as equal.

## [[Stacking Distillation|Stacking Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.8" cy="9.8" r="1.6"/><circle cx="23.9" cy="9.8" r="1.6"/><circle cx="38.0" cy="9.8" r="1.6"/><circle cx="52.0" cy="9.8" r="1.6"/><circle cx="66.1" cy="9.8" r="1.6"/><circle cx="80.2" cy="9.8" r="1.6"/><circle cx="2.8" cy="22.0" r="1.6"/><circle cx="16.9" cy="22.0" r="1.6"/><circle cx="30.9" cy="22.0" r="1.6"/><circle cx="45.0" cy="22.0" r="1.6"/><circle cx="59.1" cy="22.0" r="1.6"/><circle cx="73.1" cy="22.0" r="1.6"/><circle cx="87.2" cy="22.0" r="1.6"/><circle cx="9.8" cy="34.2" r="1.6"/><circle cx="23.9" cy="34.2" r="1.6"/><circle cx="38.0" cy="34.2" r="1.6"/><circle cx="52.0" cy="34.2" r="1.6"/><circle cx="66.1" cy="34.2" r="1.6"/><circle cx="80.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="30.9,22.0 23.9,9.8 9.8,9.8 16.9,22.0 9.8,34.2 23.9,34.2 30.9,22.0 45.0,22.0 59.1,22.0 66.1,9.8 80.2,9.8 73.1,22.0 80.2,34.2 66.1,34.2 59.1,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:197;--dur:3.9s" points="30.9,22.0 23.9,9.8 9.8,9.8 16.9,22.0 9.8,34.2 23.9,34.2 30.9,22.0 45.0,22.0 59.1,22.0 66.1,9.8 80.2,9.8 73.1,22.0 80.2,34.2 66.1,34.2 59.1,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.9" y1="22.0" x2="28.7" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.1,15.3 30.0,17.4 27.3,18.9" fill="#fc77be"/><circle cx="30.9" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`entity, entity` → `bool`

Like [[Sorter's Distillation]], but push whether they are identical items. Two [[Focus|Foci]] only compare as identical if they hold the same iota and have [[Caster's Glamour (Book Entry)|the same appearance]].

### On Item Holders

The way Nature interprets an entity given to [[Sorter's Distillation|Sorter's Dstl.]] or [[Stacking Distillation|Stacking Dstl.]] depends on its type:
- Item entities resolve to the items they represents
- Item frames resolve to the item they contain, and [[Mishaps|mishap]] if they are empty
- Players resolve to the item they are currently holding, prioritizing their primary hand

## [[Taxonomer's Distillation|Taxonomer's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0 55.2,32.0 65.5,14.3 75.7,32.0 65.5,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0 55.2,32.0 65.5,14.3 75.7,32.0 65.5,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="31.5" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,22.3 33.4,25.2 29.6,27.5" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`entity, entity` → `bool`

Compare the given entities and push whether they are of the same type.

## [[Surveyor's Distillation|Surveyor's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 52.0,22.0 59.1,9.8 73.1,9.8 80.2,22.0 73.1,34.2 59.1,34.2 52.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.6s" points="38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 52.0,22.0 59.1,9.8 73.1,9.8 80.2,22.0 73.1,34.2 59.1,34.2 52.0,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="22.0" x2="35.7" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="34.1,15.3 37.1,17.4 34.4,18.9" fill="#fc77be"/><circle cx="38.0" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`vec, vec` → `bool`

Compare the blocks specified by the given vectors and push whether they are of the same type. Two [[Edified Trees|Edified Stairs]] will always compare as equal regardless of their shape or if they are waterlogged.

## [[Inspector's Distillation|Inspector's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="8.5" r="1.6"/><circle cx="20.7" cy="8.5" r="1.6"/><circle cx="32.8" cy="8.5" r="1.6"/><circle cx="45.0" cy="8.5" r="1.6"/><circle cx="57.2" cy="8.5" r="1.6"/><circle cx="69.3" cy="8.5" r="1.6"/><circle cx="81.5" cy="8.5" r="1.6"/><circle cx="2.4" cy="19.0" r="1.6"/><circle cx="14.6" cy="19.0" r="1.6"/><circle cx="26.8" cy="19.0" r="1.6"/><circle cx="38.9" cy="19.0" r="1.6"/><circle cx="51.1" cy="19.0" r="1.6"/><circle cx="63.2" cy="19.0" r="1.6"/><circle cx="75.4" cy="19.0" r="1.6"/><circle cx="87.6" cy="19.0" r="1.6"/><circle cx="8.5" cy="29.6" r="1.6"/><circle cx="20.7" cy="29.6" r="1.6"/><circle cx="32.8" cy="29.6" r="1.6"/><circle cx="45.0" cy="29.6" r="1.6"/><circle cx="57.2" cy="29.6" r="1.6"/><circle cx="69.3" cy="29.6" r="1.6"/><circle cx="81.5" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="38.9,19.0 32.8,8.5 20.7,8.5 8.5,8.5 14.6,19.0 20.7,29.6 32.8,29.6 45.0,29.6 38.9,19.0 51.1,19.0 45.0,8.5 57.2,8.5 69.3,8.5 75.4,19.0 81.5,29.6 69.3,29.6 57.2,29.6 51.1,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:207;--dur:4.8s" points="38.9,19.0 32.8,8.5 20.7,8.5 8.5,8.5 14.6,19.0 20.7,29.6 32.8,29.6 45.0,29.6 38.9,19.0 51.1,19.0 45.0,8.5 57.2,8.5 69.3,8.5 75.4,19.0 81.5,29.6 69.3,29.6 57.2,29.6 51.1,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.9" y1="19.0" x2="37.0" y2="15.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.6,13.3 38.1,15.0 35.8,16.3" fill="#fc77be"/><circle cx="38.9" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`vec, vec` → `bool`

Compare the blocks specified by the given vectors and push whether they appear to be identical. Two placed [[Edified Trees|Edified Stairs]] will only compare as equal if they appear exactly the same.

Nature seems a bit arbitrary as to when blocks appear identical to each other. Placed [[Slates]] appear identical if they face the same direction and share waterlogging state, even if they have different patterns written to them. Similarly, chests appear identical if their outward appearance is the same, even if their contents differ.

---
### Patterns on this page

- [[Similarity Distillation (Hex Casting)]] · Pattern · Distillation
- [[Dissimilarity Distillation]] · Pattern · Distillation
- [[Sorter's Distillation]] · Pattern · Distillation
- [[Stacking Distillation]] · Pattern · Distillation
- [[Taxonomer's Distillation]] · Pattern · Distillation
- [[Surveyor's Distillation]] · Pattern · Distillation
- [[Inspector's Distillation]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/types) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
