---
aliases: ["Viewforming"]
tags: [hexbook/entry, mod/scryglass, category/patterns, topic/visuals]
mod: "Scryglass"
category: "Patterns"
entry_id: "hexcasting:scryglass"
---

# Viewforming

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Scryglass]]

I can use **Hexcasting** to conjure small strands of visible **[[Media|media]]** just before my eyes using a family of spells called the oculus spells. The small strand of **media**—called an **Icon**—dissolves in just about a second after conjuration, but if I had some method to automatically cast a **Hex**, I could permanently augment my own vision to suit my needs.

All oculus spells take an integer henceforth referred to as an index, which will be used to refer to and distinguish **Icons**. Using a used index in an oculus spell will destroy the old **Icon** and reassign it to the new one.

*Virtual instruments spread out in front of her like playing cards show her relative position and velocity, her airspeed and orientation, her oxygen levels, mana levels, and her degree of concentration.*

I can imagine the position vector for an oculus spell as the zero vector at the center of my vision. As I increase the x and y components, the position travels rightwards and downwards respectively. [[Periphery Reflection]] determines the boundaries of my vision; half of it will place my text on the peripherals of what is visible to me.

## [[Periphery Reflection|Periphery Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,41.5 45.0,18.5 18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="31.8,41.5 45.0,18.5 18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="41.5" x2="36.0" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.0,28.8 38.5,35.6 33.5,32.7" fill="#fc77be"/><circle cx="31.8" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`—` → `vector`

Pushes a vector whose respective components are the width and height of my vision, and the breadth of my vision in radians.

## [[Icons Reflection|Icons Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,18.5 71.5,18.5 58.2,41.5 45.0,64.4 31.8,41.5 18.5,18.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,18.5 71.5,18.5 58.2,41.5 45.0,64.4 31.8,41.5 18.5,18.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="18.5" x2="53.5" y2="18.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,18.5 53.5,21.4 53.5,15.6" fill="#fc77be"/><circle cx="45.0" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`—` → `list of indices`

Pushes a list of the indices of all the **Icons** I have. This appears as a list of numbers.

## [[Clear Icon|Clear Icon]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,18.5 18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,18.5 18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="18.5" x2="36.5" y2="18.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.4,18.5 36.5,15.6 36.5,21.4" fill="#fc77be"/><circle cx="45.0" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`index` → `—`

Immediately clears an **Icon**.

## [[Draw Text|Draw Text]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="38.3,35.3 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="41.9" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,24.5 44.0,30.3 39.7,27.8" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`index, vector, number, any` → `—`

Takes an index, a position, a number, and any [[Iotas|iota]]. Conjures an **Icon** displaying that iota.

The number determines the mode of text justification. 0 centers the text at the position given, 1 aligns the left side to the position given, and 2 aligns the right side to the position given.

## [[Rotate Icon|Rotate Icon]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3 49.5,54.7 27.0,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="38.3,35.3 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3 49.5,54.7 27.0,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="41.9" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,24.5 44.0,30.3 39.7,27.8" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`index, number` → `—`

Takes an index and an amount of rotation counterclockwise in terms of a full circle. Rotates the icon.

## [[Scale Icon|Scale Icon]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="85" height="90" viewBox="0 0 84.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="11.0" r="1.6"/><circle cx="18.8" cy="11.0" r="1.6"/><circle cx="34.6" cy="11.0" r="1.6"/><circle cx="50.3" cy="11.0" r="1.6"/><circle cx="66.0" cy="11.0" r="1.6"/><circle cx="81.7" cy="11.0" r="1.6"/><circle cx="11.0" cy="24.6" r="1.6"/><circle cx="26.7" cy="24.6" r="1.6"/><circle cx="42.4" cy="24.6" r="1.6"/><circle cx="58.1" cy="24.6" r="1.6"/><circle cx="73.8" cy="24.6" r="1.6"/><circle cx="3.1" cy="38.2" r="1.6"/><circle cx="18.8" cy="38.2" r="1.6"/><circle cx="34.6" cy="38.2" r="1.6"/><circle cx="50.3" cy="38.2" r="1.6"/><circle cx="66.0" cy="38.2" r="1.6"/><circle cx="81.7" cy="38.2" r="1.6"/><circle cx="11.0" cy="51.8" r="1.6"/><circle cx="26.7" cy="51.8" r="1.6"/><circle cx="42.4" cy="51.8" r="1.6"/><circle cx="58.1" cy="51.8" r="1.6"/><circle cx="73.8" cy="51.8" r="1.6"/><circle cx="3.1" cy="65.4" r="1.6"/><circle cx="18.8" cy="65.4" r="1.6"/><circle cx="34.6" cy="65.4" r="1.6"/><circle cx="50.3" cy="65.4" r="1.6"/><circle cx="66.0" cy="65.4" r="1.6"/><circle cx="81.7" cy="65.4" r="1.6"/><circle cx="11.0" cy="79.0" r="1.6"/><circle cx="26.7" cy="79.0" r="1.6"/><circle cx="42.4" cy="79.0" r="1.6"/><circle cx="58.1" cy="79.0" r="1.6"/><circle cx="73.8" cy="79.0" r="1.6"/></g><polyline class="hexpat-base" points="42.4,24.6 50.3,11.0 34.6,11.0 42.4,24.6 58.1,24.6 73.8,24.6 66.0,38.2 58.1,51.8 50.3,65.4 42.4,79.0 34.6,65.4 26.7,51.8 18.8,38.2 11.0,24.6 26.7,24.6 42.4,24.6 50.3,38.2 42.4,51.8 34.6,38.2 42.4,24.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:5.3s" points="42.4,24.6 50.3,11.0 34.6,11.0 42.4,24.6 58.1,24.6 73.8,24.6 66.0,38.2 58.1,51.8 50.3,65.4 42.4,79.0 34.6,65.4 26.7,51.8 18.8,38.2 11.0,24.6 26.7,24.6 42.4,24.6 50.3,38.2 42.4,51.8 34.6,38.2 42.4,24.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="42.4" y1="24.6" x2="44.9" y2="20.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.7,17.1 46.4,21.1 43.4,19.4" fill="#fc77be"/><circle cx="42.4" cy="24.6" r="2.6" fill="#fc77be"/></svg>

`index, number` → `—`

Takes an index and a scaling factor. Scales the icon.

## [[Draw Rectangle|Draw Rectangle]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3 49.5,54.7 60.8,74.2 72.0,54.7 60.8,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:293;--dur:3.6s" points="38.3,35.3 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 15.8,35.3 38.3,35.3 49.5,54.7 60.8,74.2 72.0,54.7 60.8,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="41.9" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,24.5 44.0,30.3 39.7,27.8" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`index, vector, vector, vector` → `—`

Takes an index, a position, a size, and a color. Conjures an **Icon** displaying that rectangle.

## [[Draw Line|Draw Line]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,29.0 40.7,13.0 22.2,13.0 31.5,29.0 50.0,29.0 40.7,45.0 31.5,61.0 22.2,45.0 13.0,29.0 31.5,29.0 40.7,45.0 50.0,61.0 59.2,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:222;--dur:3.4s" points="31.5,29.0 40.7,13.0 22.2,13.0 31.5,29.0 50.0,29.0 40.7,45.0 31.5,61.0 22.2,45.0 13.0,29.0 31.5,29.0 40.7,45.0 50.0,61.0 59.2,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="29.0" x2="34.4" y2="23.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,20.2 36.2,24.9 32.7,22.8" fill="#fc77be"/><circle cx="31.5" cy="29.0" r="2.6" fill="#fc77be"/></svg>

`index, vector, vector, vector` → `—`

Takes an index, a position, a size, and a color. Conjures an **Icon** that joins between the two with a thin line.

---
### Patterns on this page

- [[Periphery Reflection]] · Pattern · Reflection
- [[Icons Reflection]] · Pattern · Reflection
- [[Clear Icon]] · Pattern · Other
- [[Draw Text]] · Pattern · Other
- [[Rotate Icon]] · Pattern · Other
- [[Scale Icon]] · Pattern · Other
- [[Draw Rectangle]] · Pattern · Other
- [[Draw Line]] · Pattern · Other
