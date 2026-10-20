---
aliases: ["Hexal Maths"]
tags: [hexbook/entry, mod/hexal, category/patterns, topic/math]
mod: "Hexal"
category: "Patterns"
entry_id: "hexcasting:patterns/hexal_maths"
---

# Hexal Maths

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hexal]]

## [[Factorial Purification|Factorial Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1 39.2,16.2 27.7,36.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="16.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1 39.2,16.2 27.7,36.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="16.2" x2="19.8" y2="22.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,27.1 17.6,23.8 22.0,21.3" fill="#fc77be"/><circle cx="16.2" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`num` → `num`

Takes a number from the [[The Stack|stack]] and computes its factorial, for example inputting 4 would return 4\*3\*2\*1=24.

## [[Running Sum Purification|Running Sum Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="20.1" x2="39.7" y2="20.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.0,20.1 39.7,17.0 39.7,23.3" fill="#fc77be"/><circle cx="48.8" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`[num]` → `[num]`

Takes a list from the stack and computes its running sum, for example inputting \[1,2,5\] would return \[1,3,8\].

## [[Running Product Purification|Running Product Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="31.8,64.4 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="36.0" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.0,51.8 38.5,58.5 33.5,55.6" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`[num]` → `[num]`

Takes a list from the stack and computes its running product, for example inputting \[1,2,5\] would return \[1,2,10\].

---
### Patterns on this page

- [[Factorial Purification]] · Pattern · Purification
- [[Running Sum Purification]] · Pattern · Purification
- [[Running Product Purification]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/hexal_maths) · 🧩 [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
