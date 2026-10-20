---
aliases: ["Number Literals"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/math]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/numbers"
---

# Number Literals

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

## [[Numerical Reflection|Numerical Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,45.0 34.5,69.9 48.8,45.0 34.5,20.1 20.1,45.0 48.8,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,45.0 34.5,69.9 48.8,45.0 34.5,20.1 20.1,45.0 48.8,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="45.0" x2="24.7" y2="53.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,58.7 22.0,54.5 27.4,51.4" fill="#fc77be"/><circle cx="20.1" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`—` → `number`

Irritatingly, there is no easy way to draw numbers. Here is the method Nature deigned to give us.

First, I draw one of the two shapes shown on the other page. Next, the angles following will modify a running count starting at 0.
- Forward: Add 1
- Left: Add 5
- Right: Add 10
- Sharp Left: Multiply by 2
- Sharp Right: Divide by 2.

The clockwise version of the pattern, on the right of the other page, will negate the value at the very end. (The left-hand counter-clockwise version keeps the number positive).

Once I finish drawing, the number's pushed to the top of the [[The Stack|stack]].

## [[Example 1|Example 1]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="27.7,16.2 16.2,36.1 39.2,36.1 50.8,56.1 62.3,36.1 50.8,16.2 39.2,36.1 62.3,36.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="27.7,16.2 16.2,36.1 39.2,36.1 50.8,56.1 62.3,36.1 50.8,16.2 39.2,36.1 62.3,36.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.7" y1="16.2" x2="24.0" y2="22.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,27.1 21.8,21.3 26.2,23.8" fill="#fc77be"/><circle cx="27.7" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

This pattern pushes 10.

## [[Example 2|Example 2]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="74" viewBox="0 0 90.0 74.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/><circle cx="10.7" cy="50.3" r="1.6"/><circle cx="25.9" cy="50.3" r="1.6"/><circle cx="41.2" cy="50.3" r="1.6"/><circle cx="56.4" cy="50.3" r="1.6"/><circle cx="71.7" cy="50.3" r="1.6"/><circle cx="86.9" cy="50.3" r="1.6"/><circle cx="3.1" cy="63.5" r="1.6"/><circle cx="18.3" cy="63.5" r="1.6"/><circle cx="33.6" cy="63.5" r="1.6"/><circle cx="48.8" cy="63.5" r="1.6"/><circle cx="64.1" cy="63.5" r="1.6"/><circle cx="79.3" cy="63.5" r="1.6"/></g><polyline class="hexpat-base" points="10.7,50.3 25.9,50.3 41.2,50.3 48.8,63.5 56.4,50.3 48.8,37.1 41.2,50.3 56.4,50.3 64.1,37.1 71.7,23.9 79.3,10.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:153;--dur:2.8s" points="10.7,50.3 25.9,50.3 41.2,50.3 48.8,63.5 56.4,50.3 48.8,37.1 41.2,50.3 56.4,50.3 64.1,37.1 71.7,23.9 79.3,10.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="10.7" y1="50.3" x2="15.6" y2="50.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.1,50.3 15.6,52.0 15.6,48.6" fill="#fc77be"/><circle cx="10.7" cy="50.3" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

This pattern pushes 7: 5 + 1 + 1.

## [[Example 3|Example 3]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="61" viewBox="0 0 90.0 61.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/><circle cx="3.1" cy="50.3" r="1.6"/><circle cx="18.3" cy="50.3" r="1.6"/><circle cx="33.6" cy="50.3" r="1.6"/><circle cx="48.8" cy="50.3" r="1.6"/><circle cx="64.1" cy="50.3" r="1.6"/><circle cx="79.3" cy="50.3" r="1.6"/></g><polyline class="hexpat-base" points="18.3,50.3 10.7,37.1 25.9,37.1 33.6,23.9 41.2,37.1 33.6,50.3 25.9,37.1 41.2,37.1 56.4,37.1 64.1,23.9 79.3,23.9 71.7,10.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:168;--dur:3.1s" points="18.3,50.3 10.7,37.1 25.9,37.1 33.6,23.9 41.2,37.1 33.6,50.3 25.9,37.1 41.2,37.1 56.4,37.1 64.1,23.9 79.3,23.9 71.7,10.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.3" y1="50.3" x2="15.9" y2="46.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="14.1,43.0 17.3,45.2 14.4,46.9" fill="#fc77be"/><circle cx="18.3" cy="50.3" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

This pattern pushes -32: negate 1 + 5 + 10 \* 2.

## [[Example 4|Example 4]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="61" viewBox="0 0 90.0 61.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/><circle cx="3.1" cy="50.3" r="1.6"/><circle cx="18.3" cy="50.3" r="1.6"/><circle cx="33.6" cy="50.3" r="1.6"/><circle cx="48.8" cy="50.3" r="1.6"/><circle cx="64.1" cy="50.3" r="1.6"/><circle cx="79.3" cy="50.3" r="1.6"/></g><polyline class="hexpat-base" points="10.7,10.7 18.3,23.9 33.6,23.9 41.2,37.1 48.8,23.9 41.2,10.7 33.6,23.9 48.8,23.9 56.4,10.7 64.1,23.9 71.7,37.1 79.3,50.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:168;--dur:3.1s" points="10.7,10.7 18.3,23.9 33.6,23.9 41.2,37.1 48.8,23.9 41.2,10.7 33.6,23.9 48.8,23.9 56.4,10.7 64.1,23.9 71.7,37.1 79.3,50.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="10.7" y1="10.7" x2="13.1" y2="14.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="14.9,17.9 11.7,15.7 14.6,14.1" fill="#fc77be"/><circle cx="10.7" cy="10.7" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

This pattern pushes 4.5: 5 / 2 + 1 + 1.

In certain cases it might be easier to just use an [[Abacus]]. But, it's worth knowing the "proper" way to do things.

---
### Patterns on this page

- [[Numerical Reflection]] · Pattern · Reflection
- [[Example 1]] · Pattern · Other *(from Hextra Patterns)*
- [[Example 2]] · Pattern · Other *(from Hextra Patterns)*
- [[Example 3]] · Pattern · Other *(from Hextra Patterns)*
- [[Example 4]] · Pattern · Other *(from Hextra Patterns)*

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/numbers) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
