---
aliases: ["Additional Flight Patterns"]
tags: [hexbook/entry, mod/ephemera, category/spells, topic/movement]
mod: "Ephemera"
category: "Spells"
entry_id: "hexcasting:patterns/spells/ephemeral_flight"
---

# Additional Flight Patterns

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Ephemera]]

I've often found the preexisting suite of flight-related patterns to be a bit lackluster. They work perfectly well if all you want to do is cast the spell and go, but they are not well-suited to doing anything more complex. These patterns should help with that.

## [[Terminate Flight|Terminate Flight]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 45.0,32.0 65.5,32.0 75.7,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 45.0,32.0 65.5,32.0 75.7,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,41.8 19.3,36.6 23.2,38.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Cancels my current flight ability, at a cost of one tenth of an Amethyst Dust. Casting a new flight spell in the same instant allows me to continue flying, potentially with different parameters.

## [[Aviator's Purification II|Aviator's Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="22.0,28.8 31.2,12.9 40.4,28.8 49.6,44.7 31.2,44.7 12.9,44.7 22.0,28.8 40.4,28.8 58.8,28.8 68.0,12.9 77.1,28.8 68.0,44.7 58.8,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:3.4s" points="22.0,28.8 31.2,12.9 40.4,28.8 49.6,44.7 31.2,44.7 12.9,44.7 22.0,28.8 40.4,28.8 58.8,28.8 68.0,12.9 77.1,28.8 68.0,44.7 58.8,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="28.8" x2="25.0" y2="23.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.1,20.0 26.7,24.7 23.2,22.7" fill="#fc77be"/><circle cx="22.0" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`player` → `bool | null`

Returns true if the target is under the effects of [[Anchorite's Flight]], false if [[Wayfarer's Flight]], null if neither.

## [[Aviator's Purification III|Aviator's Purification III]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="22.0,28.8 31.2,12.9 40.4,28.8 49.6,44.7 31.2,44.7 12.9,44.7 22.0,28.8 40.4,28.8 58.8,28.8 68.0,12.9 77.1,28.8 68.0,44.7 58.8,28.8 77.1,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:239;--dur:3.6s" points="22.0,28.8 31.2,12.9 40.4,28.8 49.6,44.7 31.2,44.7 12.9,44.7 22.0,28.8 40.4,28.8 58.8,28.8 68.0,12.9 77.1,28.8 68.0,44.7 58.8,28.8 77.1,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="28.8" x2="25.0" y2="23.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.1,20.0 26.7,24.7 23.2,22.7" fill="#fc77be"/><circle cx="22.0" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`player` → `num | null`

Returns how close the target is to the end of their flight spell, represented as either their distance to the boundary or how many seconds remain.

---
### Patterns on this page

- [[Terminate Flight]] · Pattern · Other
- [[Aviator's Purification II]] · Pattern · Purification
- [[Aviator's Purification III]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/ephemeral_flight) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
