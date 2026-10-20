---
aliases: ["Hash Set Manipulation"]
tags: [hexbook/entry, mod/hextra-patterns, category/patterns, topic/data-structures]
mod: "Hextra Patterns"
category: "Patterns"
entry_id: "hexcasting:patterns/hextrapats_sets"
---

# Hash Set Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hextra Patterns]]

A hash set contains elements in an unordered fashion, and cannot contain duplicates. Excellent for keeping track of whether or not something exists.&lt;br&gt;Due to a magical process known as '**hashing**', access is reduced to nearly O(1), relaxing the burden on Nature.

However, '**hashing**' seems to have a different, albeit minor, burden on Nature. Numbers and vectors lose precision in the process of getting inserted into a set, becoming rounded to 4 decimals. This seems to be related to [[Equality Distillation|how Nature considers equality]].

## [[Vacant Reflection Number Set|Vacant Reflection: Number Set]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 24.5,49.7 34.8,32.0 55.2,32.0 65.5,49.7 75.7,32.0 65.5,14.3 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:266;--dur:3.6s" points="24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 24.5,49.7 34.8,32.0 55.2,32.0 65.5,49.7 75.7,32.0 65.5,14.3 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="49.7" x2="21.3" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,40.0 23.2,43.0 19.3,45.2" fill="#fc77be"/><circle cx="24.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`—` → `num set`

Push an empty set that can hold numbers to the top of the [[The Stack|stack]].

## [[Vacant Reflection Vector Set|Vacant Reflection: Vector Set]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="20.0,40.5 11.7,26.1 20.0,11.7 36.7,11.7 28.3,26.1 36.7,40.5 20.0,40.5 28.3,26.1 45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:233;--dur:3.9s" points="20.0,40.5 11.7,26.1 20.0,11.7 36.7,11.7 28.3,26.1 36.7,40.5 20.0,40.5 28.3,26.1 45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.0" y1="40.5" x2="17.3" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="15.4,32.6 18.9,35.0 15.7,36.8" fill="#fc77be"/><circle cx="20.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`—` → `vec set`

Push an empty set that can hold vectors to the top of the stack.

## [[Vacant Reflection Entity Set|Vacant Reflection: Entity Set]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="20.0,40.5 11.7,26.1 20.0,11.7 36.7,11.7 28.3,26.1 36.7,40.5 20.0,40.5 28.3,26.1 45.0,26.1 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="20.0,40.5 11.7,26.1 20.0,11.7 36.7,11.7 28.3,26.1 36.7,40.5 20.0,40.5 28.3,26.1 45.0,26.1 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.0" y1="40.5" x2="17.3" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="15.4,32.6 18.9,35.0 15.7,36.8" fill="#fc77be"/><circle cx="20.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`—` → `entity set`

Push an empty set that can hold entities to the top of the stack.

## [[Integration Distillation|Integration Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.2" y1="45.0" x2="58.6" y2="53.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.3,58.7 55.9,51.4 61.4,54.5" fill="#fc77be"/><circle cx="63.2" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`set, (num|vec|entity)` → `set`

Remove the [[Iotas|iota]] at the top of the stack, then add it to the set. The iota's type must match the set's type.

## [[Insertion Gambit|Insertion Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="58.2,41.5 45.0,64.4 18.5,64.4 31.8,41.5 18.5,18.5 45.0,18.5 58.2,41.5 71.5,64.4 45.0,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="58.2,41.5 45.0,64.4 18.5,64.4 31.8,41.5 18.5,18.5 45.0,18.5 58.2,41.5 71.5,64.4 45.0,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.2" y1="41.5" x2="54.0" y2="48.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="51.0,54.1 51.5,47.3 56.5,50.2" fill="#fc77be"/><circle cx="58.2" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`set, (num|vec|entity)` → `set, bool`

Remove the iota at the top of the stack, then add it to the set. Return True if it wasn't already in the set, otherwise return False. The iota's type must match the set's type.

## [[Excisor's Distillation|Excisor's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,47.1 44.9,41.3 49.3,43.8" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`set, (num|vec|entity)` → `set`

Remove the iota from top of the stack, then remove it set. The iota's type must match the set's type.

## [[Removal Gambit|Removal Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,32.0 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 65.5,32.0 55.2,14.3 45.0,32.0 65.5,32.0 75.7,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:266;--dur:3.6s" points="45.0,32.0 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 65.5,32.0 55.2,14.3 45.0,32.0 65.5,32.0 75.7,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="32.0" x2="41.7" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,41.8 39.8,36.6 43.7,38.8" fill="#fc77be"/><circle cx="45.0" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`set, (num|vec|entity)` → `set, bool`

Remove the iota from top of the stack, then remove it set. Return True if it was previously in the set, otherwise return False. The iota's type must match the set's type.

## [[Locator's Distillation|Locator's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:201;--dur:2.0s" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="43.7" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="50.3,45.0 43.7,48.2 43.7,41.8" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`set, (num|vec|entity)` → `set, bool`

Remove the iota top of the stack, then return True if the set contains that iota, otherwise return False. The iota's type must match the set's type.

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`set, set` → `set`

Remove the set at the top of the stack, then add all its elements to the set at the top of the stack. The two sets must be of matching type.

## [[Subtractive Distillation|Subtractive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="69.9" x2="44.3" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.9,56.2 47.0,60.3 41.5,63.5" fill="#fc77be"/><circle cx="48.8" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`set, set` → `set`

Remove the set at the top of the stack, then remove all its elements from the set at the top of the stack. The two sets must be of matching type.

## [[Conjunction Distillation|Conjunction Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,64.4 31.8,41.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="18.5,64.4 31.8,41.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="64.4" x2="22.8" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,51.8 25.3,58.5 20.2,55.6" fill="#fc77be"/><circle cx="18.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`set, set` → `set`

Takes the intersection of two sets. The new set will only contain elements that were present in both of the previous sets. The two sets must be of matching type.

## [[Exclusion Distillation|Exclusion Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="31.8,64.4 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="27.5" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,51.8 30.1,55.6 25.0,58.5" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`set, set` → `set`

Takes the exclusive disjunction of two sets. The new set will only contain elements that were present in only one of the two sets. The two sets must be of matching type.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`set` → `num`

Remove the set at the top of the stack, then push the number of elements in the set to the stack.

## [[Ordering Purification|Ordering Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 24.5,49.7 34.8,32.0 55.2,32.0 65.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:307;--dur:4.2s" points="24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 24.5,49.7 34.8,32.0 55.2,32.0 65.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="49.7" x2="21.3" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,40.0 23.2,43.0 19.3,45.2" fill="#fc77be"/><circle cx="24.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`set` → `list`

Converts the set at the top of the stack to a list. The order is up to Nature's whims.

## [[Disordering Purification|Disordering Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,49.7 45.0,32.0 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 24.5,32.0 45.0,32.0 65.5,32.0 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:327;--dur:4.5s" points="34.8,49.7 45.0,32.0 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 24.5,32.0 45.0,32.0 65.5,32.0 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="49.7" x2="38.0" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,40.0 40.0,45.2 36.1,43.0" fill="#fc77be"/><circle cx="34.8" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`[num]|[vec]|[entity]` → `set`

Converts the list at the top of the stack to a set. The list must be non-empty and entirely numbers, vectors, or entities.

## [[Bucketing Gambit|Bucketing Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,56.1 39.2,56.1 16.2,56.1 27.7,36.1 39.2,16.2 16.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1 39.2,16.2 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:277;--dur:3.4s" points="73.8,36.1 62.3,56.1 39.2,56.1 16.2,56.1 27.7,36.1 39.2,16.2 16.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1 39.2,16.2 62.3,16.2 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,47.1 68.0,41.3 72.4,43.8" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`many, num` → `set`

Remove num iota from the top of the stack, then add them to a set at the top of the stack. Must be either all numbers, all vectors, or all entities.

## [[Spilling Disintegration|Spilling Disintegration]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 39.2,56.1 16.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:277;--dur:3.4s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 39.2,56.1 16.2,56.1 27.7,36.1 39.2,16.2 50.8,36.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`set` → `many`

Remove the set at the top of the stack, then push its contents to the stack. The order is up to Nature's whims.

---
### Patterns on this page

- [[Vacant Reflection Number Set]] · Pattern · Reflection
- [[Vacant Reflection Vector Set]] · Pattern · Reflection
- [[Vacant Reflection Entity Set]] · Pattern · Reflection
- [[Integration Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Insertion Gambit]] · Pattern · Gambit
- [[Excisor's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Removal Gambit]] · Pattern · Gambit
- [[Locator's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Additive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Subtractive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Conjunction Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Exclusion Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Length Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Ordering Purification]] · Pattern · Purification
- [[Disordering Purification]] · Pattern · Purification
- [[Bucketing Gambit]] · Pattern · Gambit
- [[Spilling Disintegration]] · Pattern · Disintegration

**Sources:** 🧩 [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
