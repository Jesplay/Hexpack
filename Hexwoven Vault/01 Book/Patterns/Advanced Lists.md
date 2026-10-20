---
aliases: ["Advanced Lists"]
tags: [hexbook/entry, mod/hexic, category/patterns, topic/lists]
mod: "Hexic"
category: "Patterns"
entry_id: "hexcasting:addon/hexic/lists2"
---

# Advanced Lists

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hexic]]

These patterns seem to represent common operations I perform on lists. Though these are perfectly possible to do with normal [[List Manipulation]], it often requires more effort. Perhaps Nature willed them into existence to better understand what I'm doing... or perhaps they are my own invention?

## [[Ferris Distillation|Ferris Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:202;--dur:3.1s" points="40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="28.8" x2="37.5" y2="23.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.4,20.0 39.2,22.7 35.7,24.7" fill="#fc77be"/><circle cx="40.4" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`[a], int` → `[a]`

Rotates the list leftward: an argument of 1 moves the first item of the list to the end. Negative numbers allow rotating the list rightward.

## [[Retention Distillation|Retention Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[a], int` → `[a]`

Selects the first few elements from the input list. If the number is negative, selects the last few elements instead.

## [[Rejection Distillation|Rejection Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[a], int` → `[a]`

Drops the first few elements from the input list; the dual to [[Retention Distillation]].

## [[Retention Distillation|Retention Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[a], [int]` → `[a]`

Selects from the list only elements corresponding to the given indices. Unlike [[Selection Distillation]], this cannot reorder or duplicate [[Iotas|iotas]].

## [[Rejection Distillation|Rejection Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[a], [int]` → `[a]`

Drops from the list iotas at the given indices. All other iotas are kept in the same order as the input.

## [[Moses's Gambit|Moses's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="40.4,44.7 31.2,28.8 12.9,28.8 22.0,44.7 12.9,60.6 31.2,60.6 40.4,44.7 58.8,44.7 68.0,60.6 77.1,44.7 68.0,28.8 58.8,12.9 49.6,28.8 58.8,44.7 68.0,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="40.4,44.7 31.2,28.8 12.9,28.8 22.0,44.7 12.9,60.6 31.2,60.6 40.4,44.7 58.8,44.7 68.0,60.6 77.1,44.7 68.0,28.8 58.8,12.9 49.6,28.8 58.8,44.7 68.0,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="44.7" x2="37.5" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.4,35.9 39.2,38.6 35.7,40.6" fill="#fc77be"/><circle cx="40.4" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`[a], (a, a → bool)` → `[[a]]`

Casts the supplied **Hex** between each pair of elements in the given list, and splits the list at locations that return False.

## [[Deductive Purification|Deductive Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:208;--dur:2.5s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[bool]` → `[int]`

Creates a list of the indices for which True is present. For example, \[True, False, True\] becomes \[0, 2\]. I can also mix bools and ints in the input list.

## [[Deductive Purification|Deductive Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:208;--dur:2.5s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[num]` → `[int]`

Adds each index to the list based on the value. For example, \[1, 2, 3\] becomes \[0, 1, 1, 2, 2, 2\]. Zero or negative numbers do not appear at all.

## [[Multiplicative Distillation|Multiplicative Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="24.7" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,33.8 22.0,29.7 27.4,26.5" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`[a], [b]` → `[[a, b]]`

Calculates the outer product of two lists. The resulting list has an entry for every possible pairing between the input lists.

## [[Division Distillation|Division Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`[a], [b]` → `[[a, b]]`

Calculates the inner product of two lists. If the two lists were arranged side-by-side, the resulting list has one entry for every row (unpaired elements are skipped).

## [[Excisor's Gambit|Excisor's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.7" y1="36.1" x2="24.0" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,25.1 26.2,28.5 21.8,31.0" fill="#fc77be"/><circle cx="27.7" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[a], int` → `[a], [a], a`

Extracts an iota from the given list, and breaks the list apart around it. Returns all items before the iota, all items after the iota, and finally the iota itself.

## [[Refinement Distillation|Refinement Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 73.8,36.1 62.3,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 73.8,36.1 62.3,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`[a], [a → bool]` → `[a]`

Discards iotas from the given list unless they satisfy the given predicate. Using [[Charon's Gambit]] discards the current and all remaining iotas.

---
### Patterns on this page

- [[Ferris Distillation]] · Pattern · Distillation
- [[Retention Distillation]] · Pattern · Distillation
- [[Rejection Distillation]] · Pattern · Distillation
- [[Moses's Gambit]] · Pattern · Gambit
- [[Deductive Purification]] · Pattern · Purification
- [[Multiplicative Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Division Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Excisor's Gambit]] · Pattern · Gambit
- [[Refinement Distillation]] · Pattern · Distillation
