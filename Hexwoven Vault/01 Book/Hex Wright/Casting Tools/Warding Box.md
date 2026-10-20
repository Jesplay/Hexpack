---
aliases: ["Warding Box"]
tags: [hexbook/entry, mod/hexwright, category/casting-tools, topic/devices, topic/hexwright]
mod: "Hexwright"
category: "Casting Tools"
entry_id: "hexcasting:hex_wright/implements/warding_box"
---

# Warding Box

> **Category:** Hex Wright › [[Casting Tools (Category)|Casting Tools]]  
> **Source mod:** [[Hexwright]]

### Warding Box

*Item:* **Warding Box**

An autonomous caster with a configurable trigger.
The Warding Box will fire when the configured trigger crosses into its boundary. The entity that triggered it will be put onto the [[The Stack|stack]] and the stored hex will be cast.

Inscribe by casting [[Chronicler's Gambit]] at the box's position. It can be given [[Media|media]] by hand or by casting Recharge Item at its location with media lying in the ward volume.
The volume and trigger can be configured manually or by hex.
The Warding box active state can also be controlled by redstone.

### Ambit

The Warding Box's Ambit is the size of the ward cuboid plus 4 blocks additional in all directions. This allows the Ward to act as an alarm of sorts by things outside it's area, such as by triggering a bell or closing a door.

## [[Watcher's Gambit|Watcher's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,18.5 45.0,18.5 31.8,41.5 45.0,64.4 58.2,41.5 45.0,18.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="18.5,18.5 45.0,18.5 31.8,41.5 45.0,64.4 58.2,41.5 45.0,18.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="18.5" x2="27.0" y2="18.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.1,18.5 27.0,21.4 27.0,15.6" fill="#fc77be"/><circle cx="18.5" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`vec, entity/item type` → `—`

Sets the Warding Box trigger to a specific entitytype or itemtype. Manual trigger setting resets this.
Costs 1/10 Dust

## [[Warden's Reflection|Warden's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="31.5" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,24.1 29.6,18.9 33.4,21.1" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`vec` → `[vec,vec]`

Returns the upper and lower vector of the ward cuboid for the Warding Box at the position vec.

## [[Warden's Gambit|Warden's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="31.5" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,24.1 29.6,18.9 33.4,21.1" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`[vec,vec]` → `—`

Sets the size of the wards cuboid. Takes a list of two vectors, upper and lower points of the cuboid.

The vectors can be in any order, upper and lower or lower and upper.
The hex will [[Mishaps|mishap]] if a size larger than the ward can generate is provided.

Costs 1/10 Dust

## [[Redirect|Redirect]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0 34.8,49.7 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0 34.8,49.7 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="31.5" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,24.1 29.6,18.9 33.4,21.1" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`entity, vec` → `—`

Moves an entity from it's current position to the position specified.

Costs 1/20 dust.

Both the entity and the destination of redirect must be within the Warding Box's cuboid. Despite the box having an ambit of the cuboid +4 blocks, redirect only works within the cuboid itself.

---
### Patterns on this page

- [[Watcher's Gambit]] · Spell · Gambit
- [[Warden's Reflection]] · Pattern · Reflection
- [[Warden's Gambit]] · Spell · Gambit
- [[Redirect]] · Spell · Spell

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
