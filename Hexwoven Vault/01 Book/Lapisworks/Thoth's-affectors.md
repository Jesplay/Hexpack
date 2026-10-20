---
aliases: ["Thoth's-affectors"]
tags: [hexbook/entry, mod/lapisworks, category/lapisworks, topic/control-flow]
mod: "Lapisworks"
category: "Lapisworks"
entry_id: "hexcasting:lapis_thoth"
---

# Thoth's-affectors

> **Category:** [[Lapisworks (Category)|Lapisworks]]  
> **Source mod:** [[Lapisworks]]

These patterns deal with [[Thoth's Gambit]] or any other patterns that function exactly like it, such as [[Cubic Exaltation]] or [[Spherical Exaltation]]. These patterns can prove quite useful when I want to directly modify the **datum list** (the list the Thoth's-like is iterating over) or the **instruction list** (the hex the Thoth's-like is executing at each iteration).

## [[Queue Reflection|Queue Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 24.5,14.3 45.0,14.3 65.5,14.3 75.7,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:82;--dur:1.6s" points="14.3,32.0 24.5,14.3 45.0,14.3 65.5,14.3 75.7,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="17.6" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,22.3 19.5,27.5 15.6,25.2" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `[any]`

Pushes a list of the datum left to iterate over by the current Thoth's-like.

## [[Queue Gambit|Queue Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="14.3,14.3 24.5,32.0 45.0,32.0 65.5,32.0 75.7,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:82;--dur:1.6s" points="14.3,14.3 24.5,32.0 45.0,32.0 65.5,32.0 75.7,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="14.3" x2="17.6" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,24.1 15.6,21.1 19.5,18.9" fill="#fc77be"/><circle cx="14.3" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`[any]` → `—`

Pops a list of [[Iotas|iotas]] and sets the datum left to execute by the current Thoth's-like to that list.

## [[Executioner's Reflection|Executioner's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 37.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 20.0,11.7 36.7,11.7 45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:100;--dur:1.7s" points="11.7,26.1 20.0,11.7 36.7,11.7 45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="14.3" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.2,18.2 15.9,22.4 12.7,20.6" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `[pattern]`

Pushes the **instruction list** the current Thoth's-like is executing.

## [[Judge's Gambit|Judge's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 37.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/></g><polyline class="hexpat-base" points="11.7,11.7 20.0,26.1 36.7,26.1 45.0,11.7 53.3,26.1 70.0,26.1 78.3,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:100;--dur:1.7s" points="11.7,11.7 20.0,26.1 36.7,26.1 45.0,11.7 53.3,26.1 70.0,26.1 78.3,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="11.7" x2="14.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.2,19.6 12.7,17.2 15.9,15.4" fill="#fc77be"/><circle cx="11.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`[pattern]` → `—`

Pops a list of iotas and sets the **instruction list** of the current Thoth's-like to that list.

---
### Patterns on this page

- [[Queue Reflection]] · Pattern · Reflection
- [[Queue Gambit]] · Pattern · Gambit
- [[Executioner's Reflection]] · Pattern · Reflection
- [[Judge's Gambit]] · Pattern · Gambit

**Sources:** 🧩 [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
