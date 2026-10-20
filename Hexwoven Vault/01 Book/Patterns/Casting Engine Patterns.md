---
aliases: ["Casting Engine Patterns"]
tags: [hexbook/entry, mod/hexic, category/patterns, topic/circles]
mod: "Hexic"
category: "Patterns"
entry_id: "hexcasting:addon/hexic/enginespell"
---

# Casting Engine Patterns

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hexic]]

## [[Gearbox Reflection|Gearbox Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 38.3,54.7 15.8,54.7 27.0,35.3 38.3,54.7 49.5,35.3 60.8,15.8 72.0,35.3 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="49.5,35.3 60.8,54.7 49.5,74.2 27.0,74.2 38.3,54.7 15.8,54.7 27.0,35.3 38.3,54.7 49.5,35.3 60.8,15.8 72.0,35.3 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="53.1" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.7,46.0 51.0,42.7 55.3,40.3" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`—` → `vector`

Returns the position of the engine casting the current **Hex**. If any of these patterns are used outside a casting engine, [[Mishaps|mishaps]] rather spectacularly.

## [[Engineer's Distraction|Engineer's Distraction]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="40.4,28.8 49.6,44.7 40.4,60.6 22.0,60.6 31.2,44.7 12.9,44.7 22.0,28.8 31.2,44.7 40.4,28.8 49.6,12.9 58.8,28.8 68.0,44.7 77.1,28.8 58.8,28.8 40.4,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="40.4,28.8 49.6,44.7 40.4,60.6 22.0,60.6 31.2,44.7 12.9,44.7 22.0,28.8 31.2,44.7 40.4,28.8 49.6,12.9 58.8,28.8 68.0,44.7 77.1,28.8 58.8,28.8 40.4,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="28.8" x2="43.3" y2="33.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="45.5,37.5 41.6,34.9 45.1,32.8" fill="#fc77be"/><circle cx="40.4" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`number` → `—`

Sleeps for at least the given number of seconds. Nature seems to round this to the next twentieth of a second.

## [[Engineer's Departure|Engineer's Departure]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="36.7,11.7 45.0,26.1 36.7,40.5 20.0,40.5 28.3,26.1 11.7,26.1 20.0,11.7 28.3,26.1 36.7,11.7 53.3,11.7 45.0,26.1 61.7,26.1 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:4.2s" points="36.7,11.7 45.0,26.1 36.7,40.5 20.0,40.5 28.3,26.1 11.7,26.1 20.0,11.7 28.3,26.1 36.7,11.7 53.3,11.7 45.0,26.1 61.7,26.1 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="11.7" x2="39.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.3,19.6 37.7,17.2 40.9,15.4" fill="#fc77be"/><circle cx="36.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Suspends the current cast, as if I walked away from the engine (or stopped the rotor).

## [[Engineer's Resignation|Engineer's Resignation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="36.7,11.7 45.0,26.1 36.7,40.5 20.0,40.5 28.3,26.1 11.7,26.1 20.0,11.7 28.3,26.1 36.7,11.7 53.3,11.7 45.0,26.1 61.7,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:4.2s" points="36.7,11.7 45.0,26.1 36.7,40.5 20.0,40.5 28.3,26.1 11.7,26.1 20.0,11.7 28.3,26.1 36.7,11.7 53.3,11.7 45.0,26.1 61.7,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="11.7" x2="39.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.3,19.6 37.7,17.2 40.9,15.4" fill="#fc77be"/><circle cx="36.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`—` → `...`

Destroys the current cast, as if I stopped the rotor while sneaking. Any following patterns will never be cast.

---
### Patterns on this page

- [[Gearbox Reflection]] · Pattern · Reflection
- [[Engineer's Distraction]] · Pattern · Other
- [[Engineer's Departure]] · Pattern · Other
- [[Engineer's Resignation]] · Pattern · Other
