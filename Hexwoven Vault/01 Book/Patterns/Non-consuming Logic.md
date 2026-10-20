---
aliases: ["Non-consuming Logic"]
tags: [hexbook/entry, mod/hextra-patterns, category/patterns, topic/logic]
mod: "Hextra Patterns"
category: "Patterns"
entry_id: "hexcasting:patterns/hextrapats_retained_comparison"
---

# Non-consuming Logic

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hextra Patterns]]

Sometimes, I want operate on [[Iotas|iotas]] after using them for logic operations, but it gets rather annoying to have to copy them every time I do. All this copying also places a heavy burden on Nature, especially with large iotas like lists where [[Mishaps|stack size can be a concern]].

### Retention Gambit

With some experimentation, I have found that prepending certain logic operations with a clockwise triangle will cause the inputs to not be consumed.

## [[Retention Gambit Augur's|Retention Gambit: Augur's]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="62.3,16.2 73.8,36.1 50.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="62.3,16.2 73.8,36.1 50.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="62.3" y1="16.2" x2="66.0" y2="22.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="68.7,27.1 63.8,23.8 68.2,21.3" fill="#fc77be"/><circle cx="62.3" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`A` → `A, bool`

Convert an argument to a boolean. The number **0**, **Null**, False, the vector (0, 0, 0), and the empty list become False; everything else becomes True.

## [[Retention Gambit Equality|Retention Gambit: Equality]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="39.2,36.1 16.2,36.1 27.7,16.2 39.2,36.1 62.3,36.1 50.8,16.2 73.8,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:138;--dur:1.7s" points="39.2,36.1 16.2,36.1 27.7,16.2 39.2,36.1 62.3,36.1 50.8,16.2 73.8,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="39.2" y1="36.1" x2="31.8" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.5,36.1 31.8,33.6 31.8,38.7" fill="#fc77be"/><circle cx="39.2" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the first argument equals the second (within a small tolerance), return True. Otherwise, return False.

## [[Retention Gambit Inequality|Retention Gambit: Inequality]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="39.2,36.1 16.2,36.1 27.7,16.2 39.2,36.1 62.3,36.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:138;--dur:1.7s" points="39.2,36.1 16.2,36.1 27.7,16.2 39.2,36.1 62.3,36.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="39.2" y1="36.1" x2="31.8" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.5,36.1 31.8,33.6 31.8,38.7" fill="#fc77be"/><circle cx="39.2" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the first argument does not equal the second (outside a small tolerance), return True. Otherwise, return False.

## [[Retention Gambit Maximus|Retention Gambit: Maximus]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 49.5,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:113;--dur:1.6s" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 49.5,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="31.1" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.9,35.3 31.1,32.8 31.1,37.7" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the first argument is greater than the second, return True. Otherwise, return False.

## [[Retention Gambit Minimus|Retention Gambit: Minimus]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:113;--dur:1.6s" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="31.1" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.9,35.3 31.1,32.8 31.1,37.7" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the first argument is less than the second, return True. Otherwise, return False.

## [[Retention Gambit Maximus II|Retention Gambit: Maximus II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 49.5,54.7 38.3,74.2 15.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 49.5,54.7 38.3,74.2 15.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="31.1" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.9,35.3 31.1,32.8 31.1,37.7" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the first argument is greater than or equal to the second, return True. Otherwise, return False.

## [[Retention Gambit Minimus II|Retention Gambit: Minimus II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 38.3,74.2 60.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 38.3,74.2 60.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="31.1" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.9,35.3 31.1,32.8 31.1,37.7" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the first argument is less than or equal to the second, return True. Otherwise, return False.

## [[Retention Gambit Length Equality|Retention Gambit: Length Equality]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 14.3,67.5 24.5,49.7 34.8,67.5 55.2,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 55.2,32.0 65.5,49.7 75.7,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,67.5 14.3,67.5 24.5,49.7 34.8,67.5 55.2,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 55.2,32.0 65.5,49.7 75.7,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="28.2" y2="67.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.5,67.5 28.2,65.2 28.2,69.7" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the two vectors are the same length (within small tolerance), return True. Otherwise, return False.

## [[Retention Gambit Length Inequality|Retention Gambit: Length Inequality]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,29.0 13.0,29.0 22.2,13.0 31.5,29.0 50.0,29.0 40.7,45.0 59.2,45.0 68.5,61.0 59.2,77.0 50.0,61.0 59.2,45.0 68.5,29.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:204;--dur:3.1s" points="31.5,29.0 13.0,29.0 22.2,13.0 31.5,29.0 50.0,29.0 40.7,45.0 59.2,45.0 68.5,61.0 59.2,77.0 50.0,61.0 59.2,45.0 68.5,29.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="29.0" x2="25.5" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,29.0 25.5,26.9 25.5,31.0" fill="#fc77be"/><circle cx="31.5" cy="29.0" r="2.6" fill="#fc77be"/></svg>

`A, B` → `A, B, bool`

If the two vectors are not the same length (outside small tolerance), return True. Otherwise, return False.

---
### Patterns on this page

- [[Retention Gambit Augur's]] · Pattern · Gambit
- [[Retention Gambit Equality]] · Pattern · Gambit
- [[Retention Gambit Inequality]] · Pattern · Gambit
- [[Retention Gambit Maximus]] · Pattern · Gambit
- [[Retention Gambit Minimus]] · Pattern · Gambit
- [[Retention Gambit Maximus II]] · Pattern · Gambit
- [[Retention Gambit Minimus II]] · Pattern · Gambit
- [[Retention Gambit Length Equality]] · Pattern · Gambit
- [[Retention Gambit Length Inequality]] · Pattern · Gambit

**Sources:** 🧩 [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
