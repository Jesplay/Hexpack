---
aliases: ["Menu Manifestation"]
tags: [hexbook/entry, mod/hexintent, category/spells, topic/visuals]
mod: "HexIntent"
category: "Spells"
entry_id: "hexcasting:manifestation/menus"
---

# Menu Manifestation

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[HexIntent]]

Hex Casting has long been about manifesting patterns in the world, but I began to think, what if I could capture intent, and shape those patterns around it?
If I could truly capture these intents and imbue them into my manifestations, I could create dynamic spells that respond to my will, allowing for a level of adaptability and control I've never had before.

Create menus in hex. Buttons take a pattern that trigger on click, inputs for things like text are pushed to the button action stacks on click. All inputs are pushed to all button [[The Stack|stack]].s
The Stack from initial menu cast is also preserved and pushed to button stacks.

## [[Manifest List Menu|Manifest List Menu]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 45.0,32.0 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 45.0,32.0 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="68.7" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,22.3 70.7,27.5 66.8,25.2" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`list, text` → `—`

Takes a list of intents, a menu title and manifests a list menu.

## [[Manifest Grid Menu|Manifest Grid Menu]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 45.0,32.0 24.5,32.0 34.8,14.3 45.0,32.0 55.2,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:327;--dur:4.5s" points="65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 45.0,32.0 24.5,32.0 34.8,14.3 45.0,32.0 55.2,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="68.7" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,22.3 70.7,27.5 66.8,25.2" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`list, text, num` → `—`

Takes a list of intents, a menu title, and a number of columns to manifest a grid menu. Columns are limited to 1-10.

## [[Manifest Radial Menu|Manifest Radial Menu]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 45.0,32.0 24.5,32.0 34.8,14.3 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:4.8s" points="65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 45.0,32.0 24.5,32.0 34.8,14.3 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="68.7" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,22.3 70.7,27.5 66.8,25.2" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`list, text` → `—`

Takes a list of intents and a menu title to manifest a radial button menu. Radial menus only accept button intents

## [[Intent Button|Intent Button]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="68.0,28.8 77.1,12.9 58.8,12.9 40.4,12.9 22.0,12.9 31.2,28.8 49.6,28.8 68.0,28.8 77.1,44.7 58.8,44.7 40.4,44.7 22.0,44.7 31.2,28.8 12.9,28.8 22.0,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="68.0,28.8 77.1,12.9 58.8,12.9 40.4,12.9 22.0,12.9 31.2,28.8 49.6,28.8 68.0,28.8 77.1,44.7 58.8,44.7 40.4,44.7 22.0,44.7 31.2,28.8 12.9,28.8 22.0,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="68.0" y1="28.8" x2="70.9" y2="23.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="73.0,20.0 72.6,24.7 69.1,22.7" fill="#fc77be"/><circle cx="68.0" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`list, text` → `intent`

Creates a button from a list of patterns and a label. When clicked, the button casts the patterns. The patterns are cast in their own environment so they can't use special patterns specific to [[Spell Circles|spell circles]] or charms. They inherit the stack of environment they are cast from.

## [[Intent Input|Intent Input]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="68.0,28.8 77.1,12.9 58.8,12.9 40.4,12.9 22.0,12.9 31.2,28.8 49.6,28.8 68.0,28.8 77.1,44.7 58.8,44.7 40.4,44.7 22.0,44.7 31.2,28.8 12.9,28.8 22.0,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="68.0,28.8 77.1,12.9 58.8,12.9 40.4,12.9 22.0,12.9 31.2,28.8 49.6,28.8 68.0,28.8 77.1,44.7 58.8,44.7 40.4,44.7 22.0,44.7 31.2,28.8 12.9,28.8 22.0,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="68.0" y1="28.8" x2="70.9" y2="23.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="73.0,20.0 72.6,24.7 69.1,22.7" fill="#fc77be"/><circle cx="68.0" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`text` → `intent`

Creates an input field from a label. Entered text is pushed to the stack of an executed action, like from a button.

## [[Intent Numeric Input|Intent Numeric Input]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="68.0,44.7 77.1,28.8 58.8,28.8 40.4,28.8 22.0,28.8 31.2,44.7 49.6,44.7 68.0,44.7 77.1,60.6 58.8,60.6 40.4,60.6 22.0,60.6 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:276;--dur:4.2s" points="68.0,44.7 77.1,28.8 58.8,28.8 40.4,28.8 22.0,28.8 31.2,44.7 49.6,44.7 68.0,44.7 77.1,60.6 58.8,60.6 40.4,60.6 22.0,60.6 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="68.0" y1="44.7" x2="70.9" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="73.0,35.9 72.6,40.6 69.1,38.6" fill="#fc77be"/><circle cx="68.0" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`num, text` → `intent`

Creates a numeric input from a default value and label. The entered number is pushed to the stack of an executed action.

## [[Intent Slider|Intent Slider]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1 41.2,23.9 25.9,23.9 10.7,23.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:214;--dur:3.9s" points="71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1 41.2,23.9 25.9,23.9 10.7,23.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.7" y1="23.9" x2="74.1" y2="19.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="75.9,16.6 75.6,20.5 72.7,18.8" fill="#fc77be"/><circle cx="71.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`num, num, num, text` → `intent`

Creates a slider from min, max, current values, and a label. The chosen value is pushed to the stack of an executed action.

## [[Intent Checkbox|Intent Checkbox]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/><circle cx="11.7" cy="55.0" r="1.6"/><circle cx="28.3" cy="55.0" r="1.6"/><circle cx="45.0" cy="55.0" r="1.6"/><circle cx="61.7" cy="55.0" r="1.6"/><circle cx="78.3" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="70.0,40.5 78.3,26.1 61.7,26.1 45.0,26.1 28.3,26.1 36.7,40.5 53.3,40.5 70.0,40.5 78.3,55.0 61.7,55.0 45.0,55.0 28.3,55.0 36.7,40.5 20.0,40.5 11.7,26.1 20.0,11.7 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:283;--dur:4.8s" points="70.0,40.5 78.3,26.1 61.7,26.1 45.0,26.1 28.3,26.1 36.7,40.5 53.3,40.5 70.0,40.5 78.3,55.0 61.7,55.0 45.0,55.0 28.3,55.0 36.7,40.5 20.0,40.5 11.7,26.1 20.0,11.7 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="70.0" y1="40.5" x2="72.7" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="74.6,32.6 74.3,36.8 71.1,35.0" fill="#fc77be"/><circle cx="70.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`bool, text` → `intent`

Creates a checkbox from a default checked state and label. The resulting boolean is pushed to the stack of an executed action.

## [[Intent Select List|Intent Select List]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7 28.3,11.7 36.7,26.1 53.3,26.1 70.0,26.1 78.3,40.5 61.7,40.5 45.0,40.5 28.3,40.5 36.7,26.1 20.0,26.1 28.3,40.5 20.0,55.0 11.7,40.5 28.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:283;--dur:4.8s" points="70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7 28.3,11.7 36.7,26.1 53.3,26.1 70.0,26.1 78.3,40.5 61.7,40.5 45.0,40.5 28.3,40.5 36.7,26.1 20.0,26.1 28.3,40.5 20.0,55.0 11.7,40.5 28.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="70.0" y1="26.1" x2="72.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="74.6,18.2 74.3,22.4 71.1,20.6" fill="#fc77be"/><circle cx="70.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`list, num, text, bool?` → `intent`

Creates a selectable list from options, a max visible row count, and a label. Optionally accepts a boolean to enable multi-select. The selected option or options are pushed to the stack of an executed action.

## [[Intent Section|Intent Section]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1 41.2,23.9 25.9,23.9 10.7,23.9 18.3,10.7 33.6,10.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:244;--dur:4.5s" points="71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1 41.2,23.9 25.9,23.9 10.7,23.9 18.3,10.7 33.6,10.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.7" y1="23.9" x2="74.1" y2="19.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="75.9,16.6 75.6,20.5 72.7,18.8" fill="#fc77be"/><circle cx="71.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`text` → `intent`

Creates a non-interactive section from a label, acting as a sub heading.

## [[Intent Dropdown|Intent Dropdown]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1 41.2,23.9 25.9,23.9 10.7,23.9 18.3,37.1 33.6,37.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:244;--dur:4.5s" points="71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 33.6,10.7 41.2,23.9 56.4,23.9 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1 41.2,23.9 25.9,23.9 10.7,23.9 18.3,37.1 33.6,37.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.7" y1="23.9" x2="74.1" y2="19.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="75.9,16.6 75.6,20.5 72.7,18.8" fill="#fc77be"/><circle cx="71.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`list, text` → `intent`

Creates a dropdown from a list of options and a label. The selected option is pushed to the stack of an executed action.

---
### Patterns on this page

- [[Manifest List Menu]] · Pattern · Other
- [[Manifest Grid Menu]] · Pattern · Other
- [[Manifest Radial Menu]] · Pattern · Other
- [[Intent Button]] · Pattern · Other
- [[Intent Input]] · Pattern · Other
- [[Intent Numeric Input]] · Pattern · Other
- [[Intent Slider]] · Pattern · Other
- [[Intent Checkbox]] · Pattern · Other
- [[Intent Select List]] · Pattern · Other
- [[Intent Section]] · Pattern · Other
- [[Intent Dropdown]] · Pattern · Other

**Sources:** 🧩 [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
