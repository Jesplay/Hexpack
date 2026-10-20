---
aliases: ["Numeric Jumps"]
tags: [hexbook/entry, mod/caduceus, category/patterns, topic/control-flow]
mod: "Caduceus"
category: "Patterns"
entry_id: "hexcasting:caduceus/patterns/goto"
---

# Numeric Jumps

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Caduceus]]

## [[Elbrus' Gambit|Elbrus' Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="56" viewBox="0 0 90.0 56.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/><circle cx="9.8" cy="46.4" r="1.6"/><circle cx="23.9" cy="46.4" r="1.6"/><circle cx="38.0" cy="46.4" r="1.6"/><circle cx="52.0" cy="46.4" r="1.6"/><circle cx="66.1" cy="46.4" r="1.6"/><circle cx="80.2" cy="46.4" r="1.6"/></g><polyline class="hexpat-base" points="9.8,46.4 23.9,46.4 30.9,34.2 23.9,22.0 38.0,22.0 45.0,9.8 52.0,22.0 66.1,22.0 59.1,34.2 66.1,46.4 80.2,46.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:141;--dur:2.8s" points="9.8,46.4 23.9,46.4 30.9,34.2 23.9,22.0 38.0,22.0 45.0,9.8 52.0,22.0 66.1,22.0 59.1,34.2 66.1,46.4 80.2,46.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.8" y1="46.4" x2="14.3" y2="46.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.6,46.4 14.3,47.9 14.3,44.8" fill="#fc77be"/><circle cx="9.8" cy="46.4" r="2.6" fill="#fc77be"/></svg>

`[pattern]` → `many`

Cast a list of patterns from the [[The Stack|stack]] exactly like [[Hermes' Gambit]], except that a [[Prometheus' Gambit]] within this cast will jump to an index within this list of patterns.

## [[Prometheus' Gambit|Prometheus' Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="75.7,67.5 65.5,49.7 55.2,32.0 75.7,32.0 65.5,14.3 55.2,32.0 45.0,49.7 34.8,32.0 24.5,49.7 14.3,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="75.7,67.5 65.5,49.7 55.2,32.0 75.7,32.0 65.5,14.3 55.2,32.0 45.0,49.7 34.8,32.0 24.5,49.7 14.3,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="75.7" y1="67.5" x2="72.4" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="70.1,57.7 74.4,60.7 70.5,62.9" fill="#fc77be"/><circle cx="75.7" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`number` → `many`

Remove a number from the stack, then jump to that index in the list cast by the enclosing [[Elbrus' Gambit]]. If the number is negative, the index counts from the end of the list instead.

---
### Patterns on this page

- [[Elbrus' Gambit]] · Pattern · Gambit
- [[Prometheus' Gambit]] · Pattern · Gambit

**Sources:** 🌐 [Official online docs](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/goto) · 🧩 [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
