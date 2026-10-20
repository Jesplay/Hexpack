---
aliases: ["Specks"]
tags: [hexbook/entry, mod/hexical, category/spells, topic/visuals]
mod: "Hexical"
category: "Spells"
entry_id: "hexcasting:patterns/spells/specks"
---

# Specks

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hexical]]

I can conjure an image of an [[Iotas|iota]] into the world, called a **speck**. Nature allows me to customize them quite extensively, altering their size, thickness of stroke, and more. They also take on my [[Pigments|pigment]] color. Once they are in the world, I can move, rotate, and otherwise alter them for free, regardless of range.

## [[Conjure Speck|Conjure Speck]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,20.1 34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="48.8,20.1 34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="20.1" x2="44.3" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.9,33.8 41.5,26.5 47.0,29.7" fill="#fc77be"/><circle cx="48.8" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`any, vector, vector` → `entity`

Conjures a speck with the iota, position, and rotation. Costs a negligible amount of **[[Media|media]]**. Pushes the speck iota to the [[The Stack|stack]].

## [[Move Speck|Move Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,15.8 38.3,35.3 60.8,35.3 49.5,54.7 27.0,54.7 15.8,74.2 38.3,74.2 27.0,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="49.5,15.8 38.3,35.3 60.8,35.3 49.5,54.7 27.0,54.7 15.8,74.2 38.3,74.2 27.0,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="15.8" x2="45.9" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="43.3,26.5 43.8,20.8 48.1,23.2" fill="#fc77be"/><circle cx="49.5" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`speck entity, vector` → `—`

Moves a speck entity to the position.

## [[Rotate Speck|Rotate Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 53.7 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,13.0 22.2,29.0 40.7,29.0 31.5,45.0 13.0,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:111;--dur:1.7s" points="31.5,13.0 22.2,29.0 40.7,29.0 31.5,45.0 13.0,45.0 22.2,61.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="13.0" x2="28.5" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.4,21.8 26.7,17.1 30.3,19.1" fill="#fc77be"/><circle cx="31.5" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`speck entity, vector` → `—`

Rotates a speck entity to face the vector.

## [[Roll Speck|Roll Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,13.0 31.5,29.0 50.0,29.0 40.7,45.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0 50.0,61.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:167;--dur:2.5s" points="40.7,13.0 31.5,29.0 50.0,29.0 40.7,45.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0 50.0,61.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="13.0" x2="37.7" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.6,21.8 36.0,17.1 39.5,19.1" fill="#fc77be"/><circle cx="40.7" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`speck entity, number` → `—`

Accepts a number between 0 and 1, representing a fraction of a full rotation, and rotates the speck's image by that amount.

## [[Alter Speck|Alter Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="60.8,15.8 49.5,35.3 72.0,35.3 60.8,54.7 38.3,54.7 27.0,35.3 15.8,54.7 27.0,74.2 38.3,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:180;--dur:2.2s" points="60.8,15.8 49.5,35.3 72.0,35.3 60.8,54.7 38.3,54.7 27.0,35.3 15.8,54.7 27.0,74.2 38.3,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.8" y1="15.8" x2="57.2" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.6,26.5 55.0,20.8 59.3,23.2" fill="#fc77be"/><circle cx="60.8" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`speck entity, any` → `—`

Changes the speck's iota.

## [[Time Speck|Time Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="59.2,13.0 50.0,29.0 68.5,29.0 59.2,45.0 40.7,45.0 31.5,61.0 40.7,77.0 50.0,61.0 31.5,61.0 13.0,61.0 22.2,45.0 31.5,61.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:204;--dur:3.1s" points="59.2,13.0 50.0,29.0 68.5,29.0 59.2,45.0 40.7,45.0 31.5,61.0 40.7,77.0 50.0,61.0 31.5,61.0 13.0,61.0 22.2,45.0 31.5,61.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.2" y1="13.0" x2="56.2" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.1,21.8 54.5,17.1 58.0,19.1" fill="#fc77be"/><circle cx="59.2" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`speck entity, number` → `—`

Commands the speck to disappear after that many twentieths of a second.

## [[Resize Speck|Resize Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="68.0,12.9 58.8,28.8 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 22.0,28.8 12.9,12.9 31.2,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:147;--dur:2.2s" points="68.0,12.9 58.8,28.8 77.1,28.8 68.0,44.7 49.6,44.7 40.4,28.8 22.0,28.8 12.9,12.9 31.2,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="68.0" y1="12.9" x2="65.0" y2="17.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="62.9,21.6 63.3,16.9 66.8,19.0" fill="#fc77be"/><circle cx="68.0" cy="12.9" r="2.6" fill="#fc77be"/></svg>

`speck entity, number` → `—`

Scales the speck, can range from 0 to 10. Represents size in blocks and is zero by default.

## [[Thicken Speck|Thicken Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="70.0,11.7 61.7,26.1 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:117;--dur:2.0s" points="70.0,11.7 61.7,26.1 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="70.0" y1="11.7" x2="67.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="65.4,19.6 65.7,15.4 68.9,17.2" fill="#fc77be"/><circle cx="70.0" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`speck entity, number` → `—`

Changes the stroke thickness of a pattern speck, can range from 0 to 10. Represents twentieths of a block, and is zero by default.

## [[Paint Speck|Paint Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,13.0 31.5,29.0 50.0,29.0 40.7,45.0 22.2,45.0 13.0,61.0 22.2,77.0 31.5,61.0 22.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:148;--dur:2.2s" points="40.7,13.0 31.5,29.0 50.0,29.0 40.7,45.0 22.2,45.0 13.0,61.0 22.2,77.0 31.5,61.0 22.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="13.0" x2="37.7" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.6,21.8 36.0,17.1 39.5,19.1" fill="#fc77be"/><circle cx="40.7" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`speck entity, pigment` → `—`

Splashes a pigment iota onto the speck, changing its color after creation.

## [[Dismiss Speck|Dismiss Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 53.7 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,13.0 22.2,29.0 40.7,29.0 31.5,45.0 13.0,45.0 22.2,61.0 40.7,61.0 31.5,77.0 13.0,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:148;--dur:2.2s" points="31.5,13.0 22.2,29.0 40.7,29.0 31.5,45.0 13.0,45.0 22.2,61.0 40.7,61.0 31.5,77.0 13.0,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="13.0" x2="28.5" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.4,21.8 26.7,17.1 30.3,19.1" fill="#fc77be"/><circle cx="31.5" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`speck entity` → `—`

Forces a speck to disappear.

## [[Zone Distillation Speck|Zone Distillation: Speck]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 45.0,34.2 38.0,22.0 45.0,9.8 52.0,22.0 38.0,22.0 23.9,22.0 16.9,34.2 9.8,22.0 16.9,9.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:211;--dur:4.2s" points="52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 45.0,34.2 38.0,22.0 45.0,9.8 52.0,22.0 38.0,22.0 23.9,22.0 16.9,34.2 9.8,22.0 16.9,9.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.0" y1="22.0" x2="54.3" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.9,28.7 52.9,26.7 55.6,25.1" fill="#fc77be"/><circle cx="52.0" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list of entities`

Returns specks similarly to other [[Zone Distillation Animal|zone distillation]] patterns.

---
### Patterns on this page

- [[Conjure Speck]] · Spell · Spell
- [[Move Speck]] · Pattern · Other
- [[Rotate Speck]] · Pattern · Other
- [[Roll Speck]] · Pattern · Other
- [[Alter Speck]] · Pattern · Other
- [[Time Speck]] · Pattern · Other
- [[Resize Speck]] · Pattern · Other
- [[Thicken Speck]] · Pattern · Other
- [[Paint Speck]] · Pattern · Other
- [[Dismiss Speck]] · Pattern · Other
- [[Zone Distillation Speck]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks)
