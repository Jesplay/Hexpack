---
aliases: ["List Manipulation"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/lists]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/lists"
---

# List Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

## [[Selection Distillation|Selection Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="29.9" y2="37.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.6,31.3 32.6,35.5 27.1,38.6" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`list, number` → `any`

Remove the number at the top of the [[The Stack|stack]], then replace the list at the top with the nth element of that list (where n is the number you removed). Replaces the list with **Null** if the number is out of bounds.

## [[Selection Exaltation|Selection Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`list, num, num` → `list`

Remove the two numbers at the top of the stack, then take a sublist of the list at the top of the stack between those indices, lower bound inclusive, upper bound exclusive. For example, the 0, 2 sublist of \[0, 1, 2, 3, 4\] would be \[0, 1\].

## [[Integration Distillation|Integration Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.2" y1="45.0" x2="58.6" y2="53.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.3,58.7 55.9,51.4 61.4,54.5" fill="#fc77be"/><circle cx="63.2" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`list, any` → `list`

Remove the top of the stack, then add it to the end of the list at the top of the stack.

## [[Derivation Decomposition|Derivation Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="63.2,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="63.2,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.2" y1="45.0" x2="58.6" y2="37.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.3,31.3 61.4,35.5 55.9,38.6" fill="#fc77be"/><circle cx="63.2" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`list` → `list, any`

Remove the [[Iotas|iota]] on the end of the list at the top of the stack, and add it to the top of the stack.

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`list, list` → `list`

Remove the list at the top of the stack, then add all its elements to the end of the list at the top of the stack.

## [[Vacant Reflection|Vacant Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,69.9 63.2,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9 34.5,45.0 48.8,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:230;--dur:2.2s" points="48.8,69.9 63.2,45.0 48.8,20.1 20.1,20.1 34.5,45.0 20.1,69.9 48.8,69.9 34.5,45.0 48.8,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="69.9" x2="53.4" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="56.8,56.2 56.2,63.5 50.7,60.3" fill="#fc77be"/><circle cx="48.8" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`—` → `list`

Push an empty list to the top of the stack.

## [[Single's Purification|Single's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 45.0,41.5 31.8,18.5 58.2,18.5 71.5,41.5 58.2,64.4 31.8,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="18.5,41.5 45.0,41.5 31.8,18.5 58.2,18.5 71.5,41.5 58.2,64.4 31.8,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="27.0" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.1,41.5 27.0,44.4 27.0,38.5" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any` → `list`

Remove the top of the stack, then push a list containing only that element.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`list` → `num`

Remove the list at the top of the stack, then push the number of elements in the list to the stack.

## [[Retrograde Purification|Retrograde Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="40.2" y2="64.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.3,64.4 40.2,67.3 40.2,61.5" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`list` → `list`

Reverse the list at the top of the stack.

## [[Locator's Distillation|Locator's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:201;--dur:2.0s" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="43.7" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="50.3,45.0 43.7,48.2 43.7,41.8" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`list, any` → `num`

Remove the iota at the top of the stack, then replace the list at the top with the first index of that iota within the list (starting from 0). Replaces the list with -1 if the iota doesn't exist in the list.

## [[Excisor's Distillation|Excisor's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,47.1 44.9,41.3 49.3,43.8" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`list, num` → `list`

Remove the number at the top of the stack, then remove the nth element of the list at the top of the stack (where n is the number you removed).

## [[Surgeon's Exaltation|Surgeon's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="71.5,64.4 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 18.5,64.4 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.5" y1="64.4" x2="67.2" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="64.2,51.8 69.8,55.6 64.7,58.5" fill="#fc77be"/><circle cx="71.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`list, num, any` → `list`

Remove the top iota of the stack and the number at the top, then set the nth element of the list at the top of the stack to that iota (where n is the number you removed). Does nothing if the number is out of bounds.

## [[Flock's Gambit|Flock's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,56.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="73.8,36.1 62.3,56.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,47.1 68.0,41.3 72.4,43.8" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`many, num` → `list`

Remove num elements from the stack, then add them to a list at the top of the stack.

## [[Flock's Disintegration|Flock's Disintegration]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`list` → `many`

Remove the list at the top of the stack, then push its contents to the stack.

## [[Speaker's Distillation|Speaker's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="24.5,14.3 34.8,32.0 14.3,32.0 24.5,14.3 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="24.5,14.3 34.8,32.0 14.3,32.0 24.5,14.3 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="14.3" x2="27.8" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,24.1 25.9,21.1 29.8,18.9" fill="#fc77be"/><circle cx="24.5" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`list, any` → `list`

Remove the top iota, then add it as the first element to the list at the top of the stack.

## [[Speaker's Decomposition|Speaker's Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="65.5,14.3 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 24.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="65.5,14.3 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 24.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="14.3" x2="62.2" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.8,24.1 60.2,18.9 64.1,21.1" fill="#fc77be"/><circle cx="65.5" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`list` → `list, any`

Remove the first iota from the list at the top of the stack, then push that iota to the stack.

---
### Patterns on this page

- [[Selection Distillation]] · Pattern · Distillation
- [[Selection Exaltation]] · Pattern · Exaltation
- [[Integration Distillation]] · Pattern · Distillation
- [[Derivation Decomposition]] · Pattern · Decomposition
- [[Additive Distillation]] · Pattern · Distillation
- [[Vacant Reflection]] · Pattern · Reflection
- [[Single's Purification]] · Pattern · Purification
- [[Length Purification]] · Pattern · Purification
- [[Retrograde Purification]] · Pattern · Purification
- [[Locator's Distillation]] · Pattern · Distillation
- [[Excisor's Distillation]] · Pattern · Distillation
- [[Surgeon's Exaltation]] · Pattern · Exaltation
- [[Flock's Gambit]] · Pattern · Gambit
- [[Flock's Disintegration]] · Pattern · Disintegration
- [[Speaker's Distillation]] · Pattern · Distillation
- [[Speaker's Decomposition]] · Pattern · Decomposition

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/lists) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
