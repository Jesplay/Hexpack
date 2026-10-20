---
aliases: ["Advanced List Operators"]
tags: [hexbook/entry, mod/hexthings, category/patterns, topic/lists]
mod: "HexThings"
category: "Patterns"
entry_id: "hexcasting:patterns/uiua"
---

# Advanced List Operators

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[HexThings]]

## [[Retention Distillation (HexThings)|Retention Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`list, (num | [num])` → `list`

Keeps elements of the list based on the second argument. If it's a number and positive, it keeps the first *n* elements of the list, if negative; the last *n*. If given a list, then its elements are treated as indices to keep.

## [[Flattening Exaltation|Flattening Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="75.7" y1="49.7" x2="72.4" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="70.1,40.0 74.4,43.0 70.5,45.2" fill="#fc77be"/><circle cx="75.7" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`list, num, list` → `list`

Similar to [[Surgeon's Exaltation]], but replaces the element of the first list instead with the *contents* of the second list, rather than with the list itself.

## [[Ranger's Exaltation|Ranger's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 52.0,22.0 66.1,22.0 73.1,34.2 80.2,22.0 73.1,9.8 66.1,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:169;--dur:3.4s" points="38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 52.0,22.0 66.1,22.0 73.1,34.2 80.2,22.0 73.1,9.8 66.1,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="22.0" x2="35.7" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="34.1,15.3 37.1,17.4 34.4,18.9" fill="#fc77be"/><circle cx="38.0" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`num, num, (num | null)` → `list`

Creates a list from num #1 to num #2, with a step size of num #3 (or 1 if passed null). For example, A [[The Stack|stack]] of \[1, 2, 0.5\] would result in a list of \[1, 1.5\].

This pattern will [[Mishaps|mishap]] if the step size is 0, or if the resulting list would be larger than Nature allows.

## [[Rejection Distillation (HexThings)|Rejection Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`list, (num | [num])` → `list`

The exact inverse of [[Retention Distillation (HexThings)|Retention Distillation]], dropping the specified elements instead of dropping the rest.

## [[Ferris Distillation (HexThings)|Ferris Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:202;--dur:3.1s" points="40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="28.8" x2="37.5" y2="23.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.4,20.0 39.2,22.7 35.7,24.7" fill="#fc77be"/><circle cx="40.4" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`list, num` → `list`

Shifts the contents of the list by the specified amount, wrapping elements that go "outside" the list.

## [[Deduction Purification|Deduction Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:208;--dur:2.5s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`list` → `list`

Pushes a list containing the indices of values that are [[Augur's Purification|considered true by Nature]].

## [[Division Distillation|Division Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`list, list` → `list`

Pushes a list containing pairs of values corresponding to the values at that index in each list.

## [[Multiplicative Distillation|Multiplicative Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="24.7" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,33.8 22.0,29.7 27.4,26.5" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`list, list` → `list`

Pushes a list containing every combination of elements between the two lists.

---
### Patterns on this page

- [[Retention Distillation (HexThings)]] · Pattern · Distillation
- [[Flattening Exaltation]] · Pattern · Exaltation
- [[Ranger's Exaltation]] · Pattern · Exaltation
- [[Rejection Distillation (HexThings)]] · Pattern · Distillation
- [[Ferris Distillation (HexThings)]] · Pattern · Distillation
- [[Deduction Purification]] · Pattern · Purification
- [[Division Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Multiplicative Distillation]] · Pattern · Distillation *(from Hex Casting)*

**Sources:** 🧩 [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
