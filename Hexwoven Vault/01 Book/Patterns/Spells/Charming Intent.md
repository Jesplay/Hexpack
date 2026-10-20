---
aliases: ["Charming Intent"]
tags: [hexbook/entry, mod/hexintent, category/spells, topic/control-flow, topic/devices]
mod: "HexIntent"
category: "Spells"
entry_id: "hexcasting:manifestation/charm_audio"
---

# Charming Intent

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[HexIntent]]

Sometimes I just want to add a little flair to my charmed items, and with this, I finally can. I can now set the sound a charmed item makes when it casts!

## [[Set Charm Cast Sound|Set Charm Cast Sound]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 45.0,32.0 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="24.5,32.0 45.0,32.0 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="31.1" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.8,32.0 31.1,34.3 31.1,29.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`string | text | null, item entity?` → `none`

Changes the sound a charmed item makes when casting.

Top of [[The Stack|stack]] is string, text, or null.
Cost: 10 dust.

Use string sound id (namespace:path) to change the charm in the offhand to that sound, or null to mute.

Optionally you can pass a charmed item entity at the top of the stack to change that instead.

## [[Exit If Interacting|Exit If Interacting]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 34.8,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="14.3,32.0 34.8,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="20.9" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,32.0 20.9,34.3 20.9,29.8" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`none` → `none`

Exits a hex if caster uses something interactable. Useful for charmed items that use righ-click.

## [[Open Casting Screen|Open Casting Screen]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="40.4,28.8 31.2,44.7 49.6,44.7 58.8,28.8 40.4,28.8 31.2,12.9 22.0,28.8 12.9,44.7 22.0,60.6 40.4,60.6 58.8,60.6 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 31.2,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:276;--dur:4.2s" points="40.4,28.8 31.2,44.7 49.6,44.7 58.8,28.8 40.4,28.8 31.2,12.9 22.0,28.8 12.9,44.7 22.0,60.6 40.4,60.6 58.8,60.6 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 31.2,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="28.8" x2="37.5" y2="33.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.4,37.5 35.7,32.8 39.2,34.9" fill="#fc77be"/><circle cx="40.4" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`none` → `none`

Opens the spell grid and must be cast by a person.

## [[Clear Stack|Clear Stack]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 38.3,54.7 60.8,54.7 72.0,35.3 49.5,35.3 38.3,15.8 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 72.0,74.2 60.8,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="49.5,35.3 38.3,54.7 60.8,54.7 72.0,35.3 49.5,35.3 38.3,15.8 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 72.0,74.2 60.8,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="45.9" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="43.3,46.0 43.8,40.3 48.1,42.7" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`none` → `none`

Clears the caster's persisted [[Staves|staff]] stack (the shift, right-click stack). Must be cast by a person.

---
### Patterns on this page

- [[Set Charm Cast Sound]] · Pattern · Other
- [[Exit If Interacting]] · Pattern · Other
- [[Open Casting Screen]] · Pattern · Other
- [[Clear Stack]] · Pattern · Other

**Sources:** 🧩 [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
