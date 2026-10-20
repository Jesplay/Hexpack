---
aliases: ["Splinters"]
tags: [hexbook/entry, mod/hexintent, category/spells, topic/devices]
mod: "HexIntent"
category: "Spells"
entry_id: "hexcasting:manifestation/splinters"
---

# Splinters

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[HexIntent]]

An experiment in splitting my mind. I created unstable slivers of my own will that whilst a shadow of myself, could indeptendently cast, whilst taking the cost from myself. They would often collapse after one cast but I did observe some splinters renewing themselves perpetually, fascinating.

A peculiar side effect is that my mind splits larges amounts of work into chunks, across ticks, instead of doing it all at once. A splinter only has 16 ambit around itself. If it [[Mishaps|mishaps]], it simply collapses rather than rebounding on to myself.

## [[Manifest Splinter|Manifest Splinter]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="40.8" y2="48.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.7,54.1 38.2,47.3 43.3,50.2" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`vec, num, list` → `none`

Creates a splinter at a position, with a delay and a list of patterns to cast. The splinter casts the payload as if it were the caster, then collapses. The splinter's location is only visible to its creator.

Position: vector to summon at.
delay: time in ticks before cast.
list of patterns: list of patterns to be cast.

Cost: 5 dust x (active splinters + 1).

Splinters resume casts between Hex continuation steps. They cannot interrupt one heavy synchronous operation, so very large block edits should be implemented as explicit per-tick walkers.

## [[Renew Splinter|Renew Splinter]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="40.8" y2="48.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.7,54.1 38.2,47.3 43.3,50.2" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `none`

Only castable by a splinter. Renews the current splinter, with the same pattern. Preserves the Ravenmind between splinters.

Inputs:
position: vector to summon at.
delay: time in ticks before cast after current splinter work has finished.

Cost: replacing at the same position is free but moving to a new position costs 0.5 dust.

## [[Saturn's Gambit|Saturn's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,15.8 60.8,35.3 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 72.0,54.7 60.8,74.2 49.5,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="49.5,15.8 60.8,35.3 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 72.0,54.7 60.8,74.2 49.5,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="15.8" x2="53.1" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.7,26.5 51.0,23.2 55.3,20.8" fill="#fc77be"/><circle cx="49.5" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`pattern` → `none`

Like [[Hermes' Gambit]] but executes it asynchronously, splitting up large lists of work over multiple ticks that execute synchronously.

It borrows the technique from Splinters but is not a Splinter.

## [[Get Splinter Location|Get Splinter Location]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 36.7,55.0 28.3,40.5 36.7,26.1 45.0,40.5 53.3,26.1 61.7,40.5 53.3,55.0 36.7,55.0 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5 70.0,55.0 53.3,55.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:5.0s" points="45.0,40.5 36.7,55.0 28.3,40.5 36.7,26.1 45.0,40.5 53.3,26.1 61.7,40.5 53.3,55.0 36.7,55.0 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5 70.0,55.0 53.3,55.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="42.3" y2="45.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,48.5 40.7,44.2 43.9,46.1" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`—` → `vec`

Only castable by a splinter. Returns the current vector of the splinter. Useful for self-renewal.

## [[Destroy Current Splinter|Destroy Current Splinter]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:238;--dur:2.5s" points="45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="40.8" y2="48.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.7,54.1 38.2,47.3 43.3,50.2" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`—` → `none`

Only castable by a splinter. Dismisses only the current splinter.

## [[Destroy Splinters|Destroy Splinters]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0 45.0,14.3 55.2,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0 45.0,14.3 55.2,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="31.5" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,41.8 29.6,36.6 33.4,38.8" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `none`

Dismisses all splinters you have active.

---
### Patterns on this page

- [[Manifest Splinter]] · Pattern · Other
- [[Renew Splinter]] · Pattern · Other
- [[Saturn's Gambit]] · Pattern · Gambit
- [[Get Splinter Location]] · Pattern · Other
- [[Destroy Current Splinter]] · Pattern · Other
- [[Destroy Splinters]] · Pattern · Other

**Sources:** 🧩 [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
