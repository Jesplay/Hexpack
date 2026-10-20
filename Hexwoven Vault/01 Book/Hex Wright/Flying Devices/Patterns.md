---
aliases: ["Patterns"]
tags: [hexbook/entry, mod/hexwright, category/flying-devices, topic/hexwright, topic/movement]
mod: "Hexwright"
category: "Flying Devices"
entry_id: "hexcasting:hex_wright/flying_devices/patterns"
---

# Patterns

> **Category:** Hex Wright › [[Flying Devices (Category)|Flying Devices]]  
> **Source mod:** [[Hexwright]]

## [[Conveyance's Reflection|Conveyance's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:180;--dur:2.2s" points="27.0,54.7 15.8,74.2 38.3,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`—` → `[state]`

Can only be cast by a broom or carpet. Returns everything the vehicle knows about this moment.
Free to cast

### State

List contains, in order:
vec: Rider input
vec: Facing
vec: Rightwards
vec: Velocity
num: Climb speed
num: Ground speed
vec: Last acceleration
num: [[Media|Media]]
\[any\]: memory from last cast

### Rider Input

Rider input is your controls, not a direction: x is strafe, y is climb, z is forward, each -1 to 1. Pair x with Rightwards, z with Facing.

Last acceleration is what was really applied, after your throttle was scaled and shortened.

## [[Locomotion's Reflection|Locomotion's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,18.5 31.8,41.5 58.2,41.5 71.5,18.5 45.0,18.5 18.5,18.5 31.8,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,18.5 31.8,41.5 58.2,41.5 71.5,18.5 45.0,18.5 18.5,18.5 31.8,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="18.5" x2="40.8" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.7,31.1 38.2,24.4 43.3,27.3" fill="#fc77be"/><circle cx="45.0" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`—` → `num, num`

Your climb speed, then your ground speed.
Free to cast

## [[Speedy's Reflection|Speedy's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 20.1,69.9 48.8,69.9 63.2,45.0 34.5,45.0 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="34.5,45.0 20.1,69.9 48.8,69.9 63.2,45.0 34.5,45.0 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="29.9" y2="53.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.6,58.7 27.1,51.4 32.6,54.5" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`—` → `[num]`

Your vehicle's three limits, these are flying device constants and won't change mid flight.
Free to cast

## [[Vessel's Dismissal|Vessel's Dismissal]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="55.2,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="55.2,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="32.0" x2="52.0" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,41.8 50.0,36.6 53.9,38.8" fill="#fc77be"/><circle cx="55.2" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`bool` → `—`

Puts everyone off - the pilot, and every passenger on a carpet. True leaves the vehicle hovering where it stopped; false packs it back into the pilot's inventory.
Costs 1 Amethyst Dust

### Dismissal

Your hex carries on to its throttle as normal - there is simply nobody left to fly for, so the vehicle parks (or is already in your bag).

You still fall. Nothing catches you.

An empty reservoir cannot pay for this, and the vehicle [[Mishaps|mishaps]] instead: you are dropped either way, just less tidily.

## [[Vessel's Reflection|Vessel's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,54.7 27.0,74.2 49.5,74.2 60.8,54.7 38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 49.5,35.3 72.0,35.3 60.8,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="38.3,54.7 27.0,74.2 49.5,74.2 60.8,54.7 38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 49.5,35.3 72.0,35.3 60.8,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="54.7" x2="34.7" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,65.5 32.5,59.7 36.8,62.2" fill="#fc77be"/><circle cx="38.3" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`—` → `entity`

Returns the Broom or Carpet itself as an entity. This is how you read its position.
Free to cast

---
### Patterns on this page

- [[Conveyance's Reflection]] · Pattern · Reflection
- [[Locomotion's Reflection]] · Pattern · Reflection
- [[Speedy's Reflection]] · Pattern · Reflection
- [[Vessel's Dismissal]] · Pattern · Other
- [[Vessel's Reflection]] · Pattern · Reflection

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
