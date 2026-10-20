---
aliases: []
tags: [hexbook/entry, mod/hexintent, category/spells, topic/devices]
mod: "HexIntent"
category: "Spells"
entry_id: "hexcasting:manifestation/manifest_relay"
---

# Manifest Relay

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[HexIntent]]

## [[Manifest Relay|Manifest Relay]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 37.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/></g><polyline class="hexpat-base" points="36.7,11.7 28.3,26.1 11.7,26.1 20.0,11.7 36.7,11.7 53.3,11.7 61.7,26.1 78.3,26.1 70.0,11.7 53.3,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:150;--dur:2.5s" points="36.7,11.7 28.3,26.1 11.7,26.1 20.0,11.7 36.7,11.7 53.3,11.7 61.7,26.1 78.3,26.1 70.0,11.7 53.3,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="11.7" x2="34.0" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,19.6 32.4,15.4 35.6,17.2" fill="#fc77be"/><circle cx="36.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`vec, vec, vec, num?` → `none`

Takes a destination vector for where the relay will go, then a face vector to choose which side to manifest it on, and then a target vector for the thing to link the relay to.

Optionally you can specify a power up to 15 as the final argument to emit a redstone signal when the relay is activated.
Activating that relay will proxy the caster's interaction to the target as if they were interacting with it themselves.

Be aware that some things require the presence of a person to work fully, so the dream of opening chests from across the world will have to stay a dream.

## [[Raze Manifestation|Raze Manifestation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,49.7 34.8,32.0 45.0,14.3 55.2,32.0 45.0,49.7 24.5,49.7 14.3,32.0 34.8,32.0 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="45.0,49.7 34.8,32.0 45.0,14.3 55.2,32.0 45.0,49.7 24.5,49.7 14.3,32.0 34.8,32.0 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="49.7" x2="41.7" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,40.0 43.7,43.0 39.8,45.2" fill="#fc77be"/><circle cx="45.0" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `none`

Takes an origin vector and a radius, destroys manifestations (portals and relays) in that area. Based on permissions.

Takes an origin vector and a radius, destroys manifestations (portals and relays) in that area. Based on permissions.

---
### Patterns on this page

- [[Manifest Relay]] · Pattern · Other
- [[Raze Manifestation]] · Pattern · Other

**Sources:** 🧩 [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
