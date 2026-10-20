---
aliases: ["Vector Manipulation"]
tags: [hexbook/entry, mod/hextra-patterns, category/patterns, topic/math]
mod: "Hextra Patterns"
category: "Patterns"
entry_id: "hexcasting:patterns/hextrapats_vecmanip"
---

# Vector Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hextra Patterns]]

## [[Coordinate Purification X|Coordinate Purification: X]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1 70.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.1s" points="11.7,26.1 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1 70.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="17.0" y2="26.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,26.1 17.0,27.9 17.0,24.3" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vec` → `num`

Returns the X, Y, or Z component of a vector.

## [[Coordinate Distillation X|Coordinate Distillation: X]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1 70.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.1s" points="11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1 70.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="17.0" y2="26.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,26.1 17.0,27.9 17.0,24.3" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `vec`

Overrides a vector's X, Y, or Z component.

## [[Swizzling Purification|Swizzling Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="26.8,29.6 14.6,29.6 8.5,19.0 14.6,8.5 26.8,8.5 32.8,19.0 45.0,19.0 38.9,8.5 32.8,19.0 26.8,29.6 20.7,19.0 32.8,19.0 38.9,29.6 45.0,19.0 51.1,29.6 57.2,19.0 63.2,8.5 69.3,19.0 81.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:219;--dur:5.0s" points="26.8,29.6 14.6,29.6 8.5,19.0 14.6,8.5 26.8,8.5 32.8,19.0 45.0,19.0 38.9,8.5 32.8,19.0 26.8,29.6 20.7,19.0 32.8,19.0 38.9,29.6 45.0,19.0 51.1,29.6 57.2,19.0 63.2,8.5 69.3,19.0 81.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="26.8" y1="29.6" x2="22.9" y2="29.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.1,29.6 22.9,28.2 22.9,30.9" fill="#fc77be"/><circle cx="26.8" cy="29.6" r="2.6" fill="#fc77be"/></svg>

`vec` → `vec`

A family of patterns that "swizzles" a vector's components based on the 3 sections of its tail.

A leftward point represents X, a straight line is Y, and a rightward point is Z.

The first pattern encodes ZXY;&lt;br&gt;(1, 2, 3) becomes (3, 1, 2)

The second pattern encodes YXX;&lt;br&gt;(1, 2, 3) becomes (2, 1, 1)

The third pattern encodes YYY;&lt;br&gt;(1, 2, 3) becomes (2, 2, 2)

The fourth pattern encodes ZZX;&lt;br&gt;(1, 2, 3) becomes (3, 3, 1)

## [[Vector Purification|Vector Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="38.5" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.8,14.3 38.5,12.1 38.5,16.6" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`[num] | vec` → `vec | [num]`

Converts a list of 3 numbers to a vector, or a vector into a list of 3 numbers.

---
### Patterns on this page

- [[Coordinate Purification X]] · Pattern · Purification
- [[Coordinate Distillation X]] · Pattern · Distillation
- [[Swizzling Purification]] · Pattern · Purification
- [[Vector Purification]] · Pattern · Purification

**Sources:** 🧩 [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
