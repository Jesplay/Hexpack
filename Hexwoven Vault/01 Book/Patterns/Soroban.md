---
aliases: ["Soroban"]
tags: [hexbook/entry, mod/overevaluate, category/patterns, topic/storage]
mod: "Overevaluate"
category: "Patterns"
entry_id: "hexcasting:soroban"
---

# Soroban

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Overevaluate]]

These patterns perform simple integer operations on an imaginary counter called the soroban. They may be useful when [[Reading and Writing|Ravenmind]] is occupied or unnecessarily complicated, for example to count iterations in a [[Thoth's Gambit|Thoth's]] loop.

## [[Soroban Reflection|Soroban Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="16.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="56.1" x2="19.8" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,45.1 22.0,51.0 17.6,48.5" fill="#fc77be"/><circle cx="16.2" cy="56.1" r="2.6" fill="#fc77be"/></svg>

`—` → `number`

Pushes soroban's current value and then increases it. Starts at 0.

## [[Soroban Gambit|Soroban Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,64.4 31.8,41.5 18.5,18.5 45.0,18.5 58.2,41.5 45.0,64.4 71.5,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="18.5,64.4 31.8,41.5 18.5,18.5 45.0,18.5 58.2,41.5 45.0,64.4 71.5,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="64.4" x2="22.8" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,51.8 25.3,58.5 20.2,55.6" fill="#fc77be"/><circle cx="18.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Resets the soroban to 0.

## [[Soroban Gambit II|Soroban Gambit II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1 39.2,16.2 62.3,16.2 73.8,36.1 62.3,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="16.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1 39.2,16.2 62.3,16.2 73.8,36.1 62.3,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="16.2" x2="19.8" y2="22.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,27.1 17.6,23.8 22.0,21.3" fill="#fc77be"/><circle cx="16.2" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`number` → `—`

Pops a number from the [[The Stack|stack]] and sets the soroban to it.

---
### Patterns on this page

- [[Soroban Reflection]] · Pattern · Reflection
- [[Soroban Gambit]] · Pattern · Gambit
- [[Soroban Gambit II]] · Pattern · Gambit
