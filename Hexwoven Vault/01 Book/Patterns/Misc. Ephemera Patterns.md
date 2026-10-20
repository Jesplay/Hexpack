---
aliases: ["Misc. Ephemera Patterns"]
tags: [hexbook/entry, mod/ephemera, category/patterns, topic/blocks, topic/items, topic/math, topic/movement, topic/sensing, topic/stack, topic/wisps]
mod: "Ephemera"
category: "Patterns"
entry_id: "hexcasting:patterns/misc_patterns"
---

# Misc. Ephemera Patterns

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Ephemera]]

I've found a couple patterns designed to interact with item frames. Nothing fancy, but they're nice utility for things like controlling my [[Spell Circles|spell circles]].

Additionally, it seems [[Sorter's Purification]] works on them just fine, so my attempts to find a dedicated frame-item pattern were pointless.

## [[Decorator's Purification|Decorator's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="61" viewBox="0 0 90.0 61.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/><circle cx="10.7" cy="50.3" r="1.6"/><circle cx="25.9" cy="50.3" r="1.6"/><circle cx="41.2" cy="50.3" r="1.6"/><circle cx="56.4" cy="50.3" r="1.6"/><circle cx="71.7" cy="50.3" r="1.6"/><circle cx="86.9" cy="50.3" r="1.6"/></g><polyline class="hexpat-base" points="33.6,10.7 25.9,23.9 18.3,37.1 10.7,50.3 25.9,50.3 41.2,50.3 56.4,50.3 64.1,37.1 71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 33.6,37.1 48.8,37.1 56.4,50.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:244;--dur:4.5s" points="33.6,10.7 25.9,23.9 18.3,37.1 10.7,50.3 25.9,50.3 41.2,50.3 56.4,50.3 64.1,37.1 71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 33.6,37.1 48.8,37.1 56.4,50.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="33.6" y1="10.7" x2="31.1" y2="14.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.4,17.9 29.7,14.1 32.6,15.7" fill="#fc77be"/><circle cx="33.6" cy="10.7" r="2.6" fill="#fc77be"/></svg>

`item frame` → `number`

Accepts an item frame entity, and returns its rotation, from 0 to 7.

## [[Decorator's Gambit|Decorator's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="61" viewBox="0 0 90.0 61.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/><circle cx="10.7" cy="50.3" r="1.6"/><circle cx="25.9" cy="50.3" r="1.6"/><circle cx="41.2" cy="50.3" r="1.6"/><circle cx="56.4" cy="50.3" r="1.6"/><circle cx="71.7" cy="50.3" r="1.6"/><circle cx="86.9" cy="50.3" r="1.6"/></g><polyline class="hexpat-base" points="33.6,10.7 25.9,23.9 18.3,37.1 10.7,50.3 25.9,50.3 41.2,50.3 56.4,50.3 64.1,37.1 71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 48.8,37.1 56.4,50.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:244;--dur:4.5s" points="33.6,10.7 25.9,23.9 18.3,37.1 10.7,50.3 25.9,50.3 41.2,50.3 56.4,50.3 64.1,37.1 71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 48.8,37.1 56.4,50.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="33.6" y1="10.7" x2="31.1" y2="14.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.4,17.9 29.7,14.1 32.6,15.7" fill="#fc77be"/><circle cx="33.6" cy="10.7" r="2.6" fill="#fc77be"/></svg>

`item frame, number` → `—`

Accepts an item frame entity and an integer from 0 to 7, and sets its rotation to the number. Costs a negligible amount of [[Media|media]].

## [[Scout's Exaltation|Scout's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 34.8,32.0 55.2,32.0 65.5,14.3 75.7,32.0 65.5,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:123;--dur:1.7s" points="14.3,32.0 34.8,32.0 55.2,32.0 65.5,14.3 75.7,32.0 65.5,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="20.9" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,32.0 20.9,34.3 20.9,29.8" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`[type], vec, vec` → `entity | NULL`

As [[Scout's Distillation]], but also accepts a list of entity type [[Iotas|iotas]]. Any entity whose type is not present in the list will be ignored. Costs an amount of media slightly less negligible than that of the aforementioned pattern.

## [[Entropy Reflection II|Entropy Reflection II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="35.3" x2="19.4" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,24.5 21.5,30.3 17.2,27.8" fill="#fc77be"/><circle cx="15.8" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Returns a random number with a gaussian distribution, with a mean of zero and a standard deviation of one.

## [[Clearance Purification|Clearance Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,49.7 55.2,32.0 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="65.5,49.7 55.2,32.0 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="49.7" x2="62.2" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.8,40.0 64.1,43.0 60.2,45.2" fill="#fc77be"/><circle cx="65.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`vec` → `bool`

Accepts a vector, and returns whether that vector is within (or just on the edge of) the block in that space. Seems to involve the boxes I see in my mind's eye.

## [[Flock's Reflection II|Flock's Reflection II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 39.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:3.6s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 39.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Returns the total size of all iotas on my [[The Stack|stack]], including the contents of lists and similar. If this number ever reaches 1024, my stack shall come crashing down into garbage.

## [[Flock's Purification|Flock's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 39.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1 27.7,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:323;--dur:3.9s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 39.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1 27.7,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Accepts any iota, and returns its total size.

## [[State Purification|State Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="89" viewBox="0 0 90.0 89.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/><circle cx="3.7" cy="76.5" r="1.6"/><circle cx="22.0" cy="76.5" r="1.6"/><circle cx="40.4" cy="76.5" r="1.6"/><circle cx="58.8" cy="76.5" r="1.6"/><circle cx="77.1" cy="76.5" r="1.6"/></g><polyline class="hexpat-base" points="12.9,60.6 31.2,60.6 40.4,44.7 22.0,44.7 12.9,60.6 22.0,76.5 40.4,76.5 49.6,60.6 40.4,44.7 31.2,28.8 22.0,12.9 40.4,12.9 58.8,12.9 68.0,28.8 77.1,44.7 58.8,44.7 40.4,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:294;--dur:4.5s" points="12.9,60.6 31.2,60.6 40.4,44.7 22.0,44.7 12.9,60.6 22.0,76.5 40.4,76.5 49.6,60.6 40.4,44.7 31.2,28.8 22.0,12.9 40.4,12.9 58.8,12.9 68.0,28.8 77.1,44.7 58.8,44.7 40.4,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="60.6" x2="18.7" y2="60.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.0,60.6 18.7,62.6 18.7,58.6" fill="#fc77be"/><circle cx="12.9" cy="60.6" r="2.6" fill="#fc77be"/></svg>

`vec` → `[string]`

Accepts a vector, and returns a list of strings corresponding to values contained in the state of the block at the corresponding position.

## [[State Distillation|State Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="89" viewBox="0 0 90.0 89.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/><circle cx="3.7" cy="76.5" r="1.6"/><circle cx="22.0" cy="76.5" r="1.6"/><circle cx="40.4" cy="76.5" r="1.6"/><circle cx="58.8" cy="76.5" r="1.6"/><circle cx="77.1" cy="76.5" r="1.6"/></g><polyline class="hexpat-base" points="12.9,60.6 31.2,60.6 40.4,44.7 22.0,44.7 12.9,60.6 22.0,76.5 40.4,76.5 49.6,60.6 40.4,44.7 58.8,44.7 77.1,44.7 68.0,28.8 58.8,12.9 40.4,12.9 22.0,12.9 31.2,28.8 40.4,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:294;--dur:4.5s" points="12.9,60.6 31.2,60.6 40.4,44.7 22.0,44.7 12.9,60.6 22.0,76.5 40.4,76.5 49.6,60.6 40.4,44.7 58.8,44.7 77.1,44.7 68.0,28.8 58.8,12.9 40.4,12.9 22.0,12.9 31.2,28.8 40.4,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="60.6" x2="18.7" y2="60.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.0,60.6 18.7,62.6 18.7,58.6" fill="#fc77be"/><circle cx="12.9" cy="60.6" r="2.6" fill="#fc77be"/></svg>

`vec, string` → `bool|num|vec|string`

Accepts a vector and a string, and uses the string as a key to retrieve the value it corresponds to in the target block's state, or null if there is no corresponding value.

## [[Ride Wisp|Ride Wisp]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.1s" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.3" y1="26.1" x2="25.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.8,18.2 27.3,20.6 24.1,22.4" fill="#fc77be"/><circle cx="28.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`wisp` → `—`

Accepts one of my wisps (or a wandering wisp), and places me atop it, similar to sitting in a minecart. I must be within the wisp's ambit, or four blocks for wandering wisps. Costs one amethyst shard.

I should take care not to accelerate too quickly when riding a wisp, as unlike with Impulse, the acceleration is not applied evenly across my body, and can thus cause significant harm. The threshold for how quickly I can accelerate safely seems to be about 98 m/s over one second.
I should take care when mounting fast wisps, as the instantaneous velocity involved can contribute to this damage if I am too far from my final seated position.

## [[Dismount|Dismount]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1 27.7,36.1 16.2,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1 27.7,36.1 16.2,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.7" y1="36.1" x2="24.0" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,47.1 21.8,41.3 26.2,43.8" fill="#fc77be"/><circle cx="27.7" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Takes me off of whatever entity I may be riding when cast. Costs 1/100 of a dust.

## [[Equestrian Purification|Equestrian Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="12.9,60.6 22.0,44.7 40.4,44.7 49.6,28.8 40.4,12.9 31.2,28.8 40.4,44.7 58.8,44.7 77.1,44.7 68.0,28.8 58.8,44.7 49.6,60.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:202;--dur:3.1s" points="12.9,60.6 22.0,44.7 40.4,44.7 49.6,28.8 40.4,12.9 31.2,28.8 40.4,44.7 58.8,44.7 77.1,44.7 68.0,28.8 58.8,44.7 49.6,60.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="60.6" x2="15.8" y2="55.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.9,51.8 17.5,56.5 14.0,54.5" fill="#fc77be"/><circle cx="12.9" cy="60.6" r="2.6" fill="#fc77be"/></svg>

`entity` → `[entity]`

Accepts an entity, and returns a list of entities seated atop it.

## [[Equestrian Purification II|Equestrian Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="12.9,44.7 22.0,28.8 40.4,28.8 49.6,44.7 40.4,60.6 31.2,44.7 40.4,28.8 58.8,28.8 77.1,28.8 68.0,12.9 58.8,28.8 49.6,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:202;--dur:3.1s" points="12.9,44.7 22.0,28.8 40.4,28.8 49.6,44.7 40.4,60.6 31.2,44.7 40.4,28.8 58.8,28.8 77.1,28.8 68.0,12.9 58.8,28.8 49.6,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="44.7" x2="15.8" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.9,35.9 17.5,40.6 14.0,38.6" fill="#fc77be"/><circle cx="12.9" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`entity` → `entity`

Accepts an entity, and returns the entity it is seated atop.

---
### Patterns on this page

- [[Decorator's Purification]] · Pattern · Purification
- [[Decorator's Gambit]] · Pattern · Gambit
- [[Scout's Exaltation]] · Pattern · Exaltation
- [[Entropy Reflection II]] · Pattern · Reflection
- [[Clearance Purification]] · Pattern · Purification
- [[Flock's Reflection II]] · Pattern · Reflection
- [[Flock's Purification]] · Pattern · Purification
- [[State Purification]] · Pattern · Purification
- [[State Distillation]] · Pattern · Distillation
- [[Ride Wisp]] · Pattern · Other
- [[Dismount]] · Pattern · Other
- [[Equestrian Purification]] · Pattern · Purification
- [[Equestrian Purification II]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
