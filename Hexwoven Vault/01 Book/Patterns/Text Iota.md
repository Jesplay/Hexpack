---
aliases: ["Text Iota"]
tags: [hexbook/entry, mod/hexpose, category/patterns, topic/text]
mod: "Hexpose"
category: "Patterns"
entry_id: "hexcasting:patterns/text_iotas"
---

# Text Iota

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hexpose]]

**Text [[Iotas|iotas]]** represent textual information. Not only does it encode all the information required for reading, it also contains styles such as **coloring** and *italics*.

They can also encode an incredible amount of information in one iota. With how dense they are, they can quickly collapse into meaninglessness.

## [[Reading Purification (Hexpose)|Reading Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`any` → `text`

Takes an iota and returns a **text iota** representing it. If taking a **text iota**, strips all the styles of it.

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`text, text` → `text`

Pushes a **text iota** of the first followed immediately by the second.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`text` → `number`

Pushes the length of the **text iota**.

## [[Glyph Purification|Glyph Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,61.0 13.0,77.0 31.5,77.0 50.0,77.0 40.7,61.0 22.2,61.0 13.0,45.0 22.2,29.0 40.7,29.0 50.0,45.0 40.7,61.0 31.5,45.0 22.2,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:241;--dur:3.6s" points="22.2,61.0 13.0,77.0 31.5,77.0 50.0,77.0 40.7,61.0 22.2,61.0 13.0,45.0 22.2,29.0 40.7,29.0 50.0,45.0 40.7,61.0 31.5,45.0 22.2,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="61.0" x2="19.2" y2="66.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="17.1,69.8 17.5,65.1 21.0,67.2" fill="#fc77be"/><circle cx="22.2" cy="61.0" r="2.6" fill="#fc77be"/></svg>

`text` → `list`

Splits a **text iota** into a list of individual glyphs.

## [[Collage Purification|Collage Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,61.0 50.0,77.0 31.5,77.0 13.0,77.0 22.2,61.0 40.7,61.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 22.2,61.0 31.5,45.0 40.7,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:241;--dur:3.6s" points="40.7,61.0 50.0,77.0 31.5,77.0 13.0,77.0 22.2,61.0 40.7,61.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 22.2,61.0 31.5,45.0 40.7,29.0 31.5,13.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="61.0" x2="43.7" y2="66.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="45.8,69.8 41.9,67.2 45.4,65.1" fill="#fc77be"/><circle cx="40.7" cy="61.0" r="2.6" fill="#fc77be"/></svg>

`list` → `text`

Transforms all the elements of a list into text (or keeps them if they were) and pastes it all together.

## [[Selection Distillation|Selection Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="29.9" y2="37.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.6,31.3 32.6,35.5 27.1,38.6" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`text, number` → `text`

Pushes the nth glyph in the text.

## [[Selection Exaltation|Selection Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`text, number, number` → `text`

Pushes the specified slice of the text.

## [[Surgeon's Exaltation|Surgeon's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.5" y1="64.4" x2="67.2" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="64.2,51.8 69.8,55.6 64.7,58.5" fill="#fc77be"/><circle cx="71.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`text, number, text` → `text`

Replaces the glyph at the nth position in the text with another segment of text. Replacement can be longer than one glyph.

## [[Excisor's Distillation|Excisor's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,47.1 44.9,41.3 49.3,43.8" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`text, number` → `text`

Removes the nth glyph in the text.

## [[Retrograde Purification|Retrograde Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="40.2" y2="64.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.3,64.4 40.2,67.3 40.2,61.5" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`text` → `text`

Reverses the text.

## [[Uniqueness Purification|Uniqueness Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:143;--dur:2.0s" points="65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="49.7" x2="68.7" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,40.0 70.7,45.2 66.8,43.0" fill="#fc77be"/><circle cx="65.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`text` → `text`

Keeps only the first instance of any glyph in the text. Styling is enough to distinguish a glyph.

The following are a set of patterns involving the styling of the text. Despite resembling distillations, they are actually *gambits* due to their unique argument structure.

If the regular arguments are passed in, they will style the text according to their speciality (example of modification included).

If passed in just a text however, they will return the current value of the setting that they target.

## [[Lumière Gambit|Lumière Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 38.3,35.3 49.5,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:3.9s" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,74.2 27.0,54.7 38.3,35.3 49.5,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`text, vector` → `text`

Takes a **text iota** and a color encoded as a vector whose components range from 0 to 1, and produces a **text iota** of that **color**.

## [[Gothic Gambit|Gothic Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 60.8,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 60.8,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`text, boolean` → `text`

Takes a **text iota** and sets whether it is **bold**.

## [[Manutius' Gambit|Manutius' Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 49.5,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 49.5,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`text, boolean` → `text`

Takes a **text iota** and sets whether it is in *italics*.

## [[Notetaker's Gambit|Notetaker's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`text, boolean` → `text`

Takes a **text iota** and sets whether it is **underlined**.

## [[Editor's Gambit|Editor's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 15.8,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 15.8,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`text, boolean` → `text`

Takes a **text iota** and sets whether it has been struck through.

## [[Censor's Gambit|Censor's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 27.0,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 38.3,35.3 27.0,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="54.7" x2="23.4" y2="61.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,65.5 21.3,59.7 25.6,62.2" fill="#fc77be"/><circle cx="27.0" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`text, boolean` → `text`

Takes a **text iota** and sets whether it is obfuscated.

---
### Patterns on this page

- [[Reading Purification (Hexpose)]] · Pattern · Purification
- [[Additive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Length Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Glyph Purification]] · Pattern · Purification
- [[Collage Purification]] · Pattern · Purification
- [[Selection Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Selection Exaltation]] · Pattern · Exaltation *(from Hex Casting)*
- [[Surgeon's Exaltation]] · Pattern · Exaltation *(from Hex Casting)*
- [[Excisor's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Retrograde Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Uniqueness Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Lumière Gambit]] · Pattern · Gambit
- [[Gothic Gambit]] · Pattern · Gambit
- [[Manutius' Gambit]] · Pattern · Gambit
- [[Notetaker's Gambit]] · Pattern · Gambit
- [[Editor's Gambit]] · Pattern · Gambit
- [[Censor's Gambit]] · Pattern · Gambit

**Sources:** 🧩 [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
