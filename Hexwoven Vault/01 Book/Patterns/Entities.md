---
aliases: ["Entities"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/sensing]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/entities"
---

# Entities

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

## [[Entity Purification|Entity Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="38.0" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,41.8 36.1,38.8 40.0,36.6" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vector` → `entity or null`

Transform the position on the [[The Stack|stack]] into the entity at that location (or **Null** if there isn't one).

## [[Entity Purification Animal|Entity Purification: Animal]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="47.7" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,34.0 46.1,31.6 49.3,29.8" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vector` → `entity or null`

Transform the position on the stack into the animal at that location (or **Null** if there isn't one).

## [[Entity Purification Monster|Entity Purification: Monster]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,37.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:198;--dur:3.6s" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,37.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="23.9" x2="51.3" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="53.0,31.2 49.8,29.0 52.7,27.3" fill="#fc77be"/><circle cx="48.8" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`vector` → `entity or null`

Transform the position on the stack into the monster at that location (or **Null** if there isn't one).

## [[Entity Purification Item|Entity Purification: Item]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 45.0,9.8 38.0,22.0 45.0,34.2 52.0,22.0 38.0,22.0 23.9,22.0 9.8,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.6s" points="52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 45.0,9.8 38.0,22.0 45.0,34.2 52.0,22.0 38.0,22.0 23.9,22.0 9.8,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.0" y1="22.0" x2="54.3" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.9,28.7 52.9,26.7 55.6,25.1" fill="#fc77be"/><circle cx="52.0" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`vector` → `entity or null`

Transform the position on the stack into the dropped item at that location (or **Null** if there isn't one).

## [[Entity Purification Player|Entity Purification: Player]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,10.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:198;--dur:3.6s" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,10.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="23.9" x2="51.3" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="53.0,31.2 49.8,29.0 52.7,27.3" fill="#fc77be"/><circle cx="48.8" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`vector` → `entity or null`

Transform the position on the stack into the player at that location (or **Null** if there isn't one).

## [[Entity Purification Living|Entity Purification: Living]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="47.7" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,34.0 46.1,31.6 49.3,29.8" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vector` → `entity or null`

Transform the position on the stack into the living creature at that location (or **Null** if there isn't one).

## [[Zone Distillation Animal|Zone Distillation: Animal]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,40.5 28.3,26.1 36.7,11.7 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,40.5 28.3,26.1 36.7,11.7 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="47.7" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,34.0 46.1,31.6 49.3,29.8" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of animals near the position.

## [[Zone Distillation Non-Animal|Zone Distillation: Non-Animal]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="47.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,18.2 49.3,22.4 46.1,20.6" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of non-animal entities near the position.

## [[Zone Distillation Monster|Zone Distillation: Monster]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,37.1 33.6,23.9 41.2,10.7 48.8,23.9 33.6,23.9 18.3,23.9 10.7,37.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:198;--dur:3.6s" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,37.1 33.6,23.9 41.2,10.7 48.8,23.9 33.6,23.9 18.3,23.9 10.7,37.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="23.9" x2="51.3" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="53.0,31.2 49.8,29.0 52.7,27.3" fill="#fc77be"/><circle cx="48.8" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of monsters near the position.

## [[Zone Distillation Non-Monster|Zone Distillation: Non-Monster]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="48.8,23.9 56.4,10.7 71.7,10.7 79.3,23.9 71.7,37.1 56.4,37.1 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,37.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:198;--dur:3.6s" points="48.8,23.9 56.4,10.7 71.7,10.7 79.3,23.9 71.7,37.1 56.4,37.1 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,37.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="23.9" x2="51.3" y2="19.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="53.0,16.6 52.7,20.5 49.8,18.8" fill="#fc77be"/><circle cx="48.8" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of non-monster entities near the position.

## [[Zone Distillation Item|Zone Distillation: Item]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 45.0,34.2 38.0,22.0 45.0,9.8 52.0,22.0 38.0,22.0 23.9,22.0 9.8,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.6s" points="52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 45.0,34.2 38.0,22.0 45.0,9.8 52.0,22.0 38.0,22.0 23.9,22.0 9.8,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.0" y1="22.0" x2="54.3" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.9,28.7 52.9,26.7 55.6,25.1" fill="#fc77be"/><circle cx="52.0" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of dropped items near the position.

## [[Zone Distillation Non-Item|Zone Distillation: Non-Item]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="52.0,22.0 59.1,9.8 73.1,9.8 80.2,22.0 73.1,34.2 59.1,34.2 52.0,22.0 45.0,9.8 38.0,22.0 45.0,34.2 52.0,22.0 38.0,22.0 23.9,22.0 9.8,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.6s" points="52.0,22.0 59.1,9.8 73.1,9.8 80.2,22.0 73.1,34.2 59.1,34.2 52.0,22.0 45.0,9.8 38.0,22.0 45.0,34.2 52.0,22.0 38.0,22.0 23.9,22.0 9.8,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.0" y1="22.0" x2="54.3" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.9,15.3 55.6,18.9 52.9,17.4" fill="#fc77be"/><circle cx="52.0" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of non-dropped-item entities near the position.

## [[Zone Distillation Player|Zone Distillation: Player]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,37.1 33.6,23.9 41.2,10.7 48.8,23.9 33.6,23.9 18.3,23.9 10.7,10.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:198;--dur:3.6s" points="48.8,23.9 56.4,37.1 71.7,37.1 79.3,23.9 71.7,10.7 56.4,10.7 48.8,23.9 41.2,37.1 33.6,23.9 41.2,10.7 48.8,23.9 33.6,23.9 18.3,23.9 10.7,10.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="23.9" x2="51.3" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="53.0,31.2 49.8,29.0 52.7,27.3" fill="#fc77be"/><circle cx="48.8" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of players near the position.

## [[Zone Distillation Non-Player|Zone Distillation: Non-Player]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="48.8,23.9 56.4,10.7 71.7,10.7 79.3,23.9 71.7,37.1 56.4,37.1 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,10.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:198;--dur:3.6s" points="48.8,23.9 56.4,10.7 71.7,10.7 79.3,23.9 71.7,37.1 56.4,37.1 48.8,23.9 41.2,10.7 33.6,23.9 41.2,37.1 48.8,23.9 33.6,23.9 18.3,23.9 10.7,10.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="23.9" x2="51.3" y2="19.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="53.0,16.6 52.7,20.5 49.8,18.8" fill="#fc77be"/><circle cx="48.8" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of non-player characters near the position.

## [[Zone Distillation Living|Zone Distillation: Living]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,40.5 28.3,26.1 36.7,11.7 45.0,26.1 28.3,26.1 11.7,26.1 20.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,40.5 28.3,26.1 36.7,11.7 45.0,26.1 28.3,26.1 11.7,26.1 20.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="47.7" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,34.0 46.1,31.6 49.3,29.8" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of living creatures near the position.

## [[Zone Distillation Non-Living|Zone Distillation: Non-Living]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 28.3,26.1 11.7,26.1 20.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="47.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,18.2 49.3,22.4 46.1,20.6" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of non-living entities near the position.

## [[Zone Distillation Any|Zone Distillation: Any]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="38.0" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,41.8 36.1,38.8 40.0,36.6" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `list`

Take a position and maximum distance on the stack, and combine them into a list of all entities near the position.

---
### Patterns on this page

- [[Entity Purification]] · Pattern · Purification
- [[Entity Purification Animal]] · Pattern · Purification
- [[Entity Purification Monster]] · Pattern · Purification
- [[Entity Purification Item]] · Pattern · Purification
- [[Entity Purification Player]] · Pattern · Purification
- [[Entity Purification Living]] · Pattern · Purification
- [[Zone Distillation Animal]] · Pattern · Distillation
- [[Zone Distillation Non-Animal]] · Pattern · Distillation
- [[Zone Distillation Monster]] · Pattern · Distillation
- [[Zone Distillation Non-Monster]] · Pattern · Distillation
- [[Zone Distillation Item]] · Pattern · Distillation
- [[Zone Distillation Non-Item]] · Pattern · Distillation
- [[Zone Distillation Player]] · Pattern · Distillation
- [[Zone Distillation Non-Player]] · Pattern · Distillation
- [[Zone Distillation Living]] · Pattern · Distillation
- [[Zone Distillation Non-Living]] · Pattern · Distillation
- [[Zone Distillation Any]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/entities) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
