---
aliases: ["Splicing Table Patterns"]
tags: [hexbook/entry, mod/hexdebug, category/patterns, topic/debugging]
mod: "HexDebug"
category: "Patterns"
entry_id: "hexcasting:patterns/splicing_table"
---

# Splicing Table Patterns

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[HexDebug]]

## [[Sprocket's Purification|Sprocket's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 58.8,28.8 68.0,44.7 77.1,28.8 68.0,12.9 58.8,28.8 49.6,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 58.8,28.8 68.0,44.7 77.1,28.8 68.0,12.9 58.8,28.8 49.6,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="44.7" x2="46.7" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,35.9 48.4,38.6 44.9,40.6" fill="#fc77be"/><circle cx="49.6" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`vec` → `num`

Remove a vector from the [[The Stack|stack]], then push the index of the leftmost [[Iotas|iota]] currently visible in the [[Splicing Table]] at that position.

## [[Sprocket's Gambit|Sprocket's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 58.8,28.8 68.0,12.9 77.1,28.8 68.0,44.7 58.8,28.8 49.6,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 58.8,28.8 68.0,12.9 77.1,28.8 68.0,44.7 58.8,28.8 49.6,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="12.9" x2="46.7" y2="17.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,21.6 44.9,16.9 48.4,19.0" fill="#fc77be"/><circle cx="49.6" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `—`

Remove a vector and number from the stack, then move the view of the [[Splicing Table]] at that position so that the given index is the leftmost one visible.

## [[Splicer's Decomposition|Splicer's Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 58.8,28.8 49.6,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 58.8,28.8 49.6,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="44.7" x2="46.7" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,35.9 48.4,38.6 44.9,40.6" fill="#fc77be"/><circle cx="49.6" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`vec` → `num | null, num | null`

Remove a vector from the stack, then push two values representing the current selection in the [[Splicing Table]] at that position.

If a range of iotas is selected, two numbers are pushed, representing the start (inclusive) and end (exclusive) of the selected range. If the edge between two iotas is selected, the index of the iota to the right of the selected edge is pushed, followed by **Null**. And if nothing is selected, **Null** is pushed twice.

## [[Splicer's Gambit|Splicer's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 58.8,28.8 49.6,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 58.8,28.8 49.6,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="12.9" x2="46.7" y2="17.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,21.6 44.9,16.9 48.4,19.0" fill="#fc77be"/><circle cx="49.6" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`vec, num | null, num | null` → `—`

Remove a vector and two values from the stack, then set the selection of the [[Splicing Table]] at that position as previously described.

## [[Compositor's Purification|Compositor's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 58.8,28.8 68.0,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="49.6,44.7 40.4,28.8 31.2,12.9 12.9,12.9 22.0,28.8 12.9,44.7 31.2,44.7 40.4,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 49.6,44.7 58.8,28.8 68.0,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="44.7" x2="46.7" y2="39.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,35.9 48.4,38.6 44.9,40.6" fill="#fc77be"/><circle cx="49.6" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`vec` → `any`

Remove a vector from the stack, then copy the contents of the secondary item in the [[Splicing Table]] at that position and push it to the stack.

## [[Compositor's Gambit|Compositor's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 58.8,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 58.8,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="12.9" x2="46.7" y2="17.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,21.6 44.9,16.9 48.4,19.0" fill="#fc77be"/><circle cx="49.6" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`vec, any` → `—`

Remove a vector and iota from the stack, then save the iota into the secondary item in the [[Splicing Table]] at that position.
As with [[Chronicler's Gambit]], I cannot write my own Name using this spell.

## [[Producer's Purification|Producer's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="49.6,60.6 40.4,44.7 31.2,28.8 12.9,28.8 22.0,44.7 12.9,60.6 31.2,60.6 40.4,44.7 49.6,28.8 68.0,28.8 77.1,44.7 68.0,60.6 49.6,60.6 58.8,44.7 68.0,28.8 77.1,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:276;--dur:4.2s" points="49.6,60.6 40.4,44.7 31.2,28.8 12.9,28.8 22.0,44.7 12.9,60.6 31.2,60.6 40.4,44.7 49.6,28.8 68.0,28.8 77.1,44.7 68.0,60.6 49.6,60.6 58.8,44.7 68.0,28.8 77.1,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="60.6" x2="46.7" y2="55.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,51.8 48.4,54.5 44.9,56.5" fill="#fc77be"/><circle cx="49.6" cy="60.6" r="2.6" fill="#fc77be"/></svg>

`vec` → `bool`

Replace the vector at the top of the stack with **True** if there is a [[Splicing Table]] at that position containing a secondary item that I could read, or **False** otherwise.

## [[Director's Purification|Director's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 58.8,28.8 68.0,44.7 77.1,60.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:276;--dur:4.2s" points="49.6,12.9 40.4,28.8 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 58.8,28.8 68.0,44.7 77.1,60.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.6" y1="12.9" x2="46.7" y2="17.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,21.6 44.9,16.9 48.4,19.0" fill="#fc77be"/><circle cx="49.6" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`vec` → `bool`

Replace the vector at the top of the stack with **True** if there is a [[Splicing Table]] at that position containing a secondary item that I could save an iota into, or **False** otherwise.

## [[Projectionist's Purification|Projectionist's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="55.2,49.7 45.0,32.0 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 45.0,32.0 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7 65.5,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="55.2,49.7 45.0,32.0 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 45.0,32.0 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7 65.5,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="49.7" x2="52.0" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,40.0 53.9,43.0 50.0,45.2" fill="#fc77be"/><circle cx="55.2" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`vec` → `num`

Remove a vector from the stack, then push the current page number (starting at 1) of the [[Spellbook]] in the main slot of the [[Splicing Table]] or [[Focal Frame]] at that position.

## [[Projectionist's Gambit|Projectionist's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="55.2,14.3 45.0,32.0 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 65.5,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="55.2,14.3 45.0,32.0 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 65.5,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="14.3" x2="52.0" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,24.1 50.0,18.9 53.9,21.1" fill="#fc77be"/><circle cx="55.2" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `—`

Remove a vector and number from the stack, then flip the [[Spellbook]] in the main slot of the [[Splicing Table]] or [[Focal Frame]] at that position to the given page number.

## [[Shutter's Purification|Shutter's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="55.2,67.5 45.0,49.7 34.8,32.0 14.3,32.0 24.5,49.7 14.3,67.5 34.8,67.5 45.0,49.7 55.2,32.0 75.7,32.0 65.5,49.7 75.7,67.5 55.2,67.5 65.5,49.7 55.2,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:307;--dur:4.2s" points="55.2,67.5 45.0,49.7 34.8,32.0 14.3,32.0 24.5,49.7 14.3,67.5 34.8,67.5 45.0,49.7 55.2,32.0 75.7,32.0 65.5,49.7 75.7,67.5 55.2,67.5 65.5,49.7 55.2,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="67.5" x2="52.0" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,57.7 53.9,60.7 50.0,62.9" fill="#fc77be"/><circle cx="55.2" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`vec` → `bool`

Replace the vector at the top of the stack with **True** if there is a [[Splicing Table]] or [[Focal Frame]] at that position with a [[Spellbook]] in its main slot that contains at least one page, or **False** otherwise.

## [[Projectionist's Purification II|Projectionist's Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="55.2,49.7 45.0,32.0 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="55.2,49.7 45.0,32.0 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="49.7" x2="52.0" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,40.0 53.9,43.0 50.0,45.2" fill="#fc77be"/><circle cx="55.2" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`vec` → `num`

Like [[Projectionist's Purification]], but the page number of the [[Spellbook]] in the secondary slot is read instead of the main slot.

## [[Projectionist's Gambit II|Projectionist's Gambit II]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="55.2,14.3 45.0,32.0 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 65.5,32.0 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="55.2,14.3 45.0,32.0 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 65.5,32.0 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="14.3" x2="52.0" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,24.1 50.0,18.9 53.9,21.1" fill="#fc77be"/><circle cx="55.2" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `—`

Like [[Projectionist's Gambit]], but the page number of the [[Spellbook]] in the secondary slot is written instead of the main slot.

## [[Shutter's Purification II|Shutter's Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="55.2,67.5 45.0,49.7 34.8,32.0 14.3,32.0 24.5,49.7 14.3,67.5 34.8,67.5 45.0,49.7 55.2,32.0 65.5,49.7 55.2,67.5 75.7,67.5 65.5,49.7 75.7,32.0 55.2,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:307;--dur:4.2s" points="55.2,67.5 45.0,49.7 34.8,32.0 14.3,32.0 24.5,49.7 14.3,67.5 34.8,67.5 45.0,49.7 55.2,32.0 65.5,49.7 55.2,67.5 75.7,67.5 65.5,49.7 75.7,32.0 55.2,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="67.5" x2="52.0" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,57.7 53.9,60.7 50.0,62.9" fill="#fc77be"/><circle cx="55.2" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`vec` → `bool`

Like [[Shutter's Purification]], but the [[Spellbook]] in the secondary slot is checked instead of the main slot.

---
### Patterns on this page

- [[Sprocket's Purification]] · Pattern · Purification
- [[Sprocket's Gambit]] · Spell · Gambit
- [[Splicer's Decomposition]] · Pattern · Decomposition
- [[Splicer's Gambit]] · Spell · Gambit
- [[Compositor's Purification]] · Pattern · Purification
- [[Compositor's Gambit]] · Spell · Gambit
- [[Producer's Purification]] · Pattern · Purification
- [[Director's Purification]] · Pattern · Purification
- [[Projectionist's Purification]] · Pattern · Purification
- [[Projectionist's Gambit]] · Spell · Gambit
- [[Shutter's Purification]] · Pattern · Purification
- [[Projectionist's Purification II]] · Pattern · Purification
- [[Projectionist's Gambit II]] · Spell · Gambit
- [[Shutter's Purification II]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/splicing_table) · 🧩 [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
