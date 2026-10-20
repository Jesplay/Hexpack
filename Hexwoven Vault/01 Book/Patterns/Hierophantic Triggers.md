---
aliases: ["Hierophantic Triggers"]
tags: [hexbook/entry, mod/hierophantics, category/patterns, topic/minds]
mod: "Hierophantics"
category: "Patterns"
entry_id: "hexcasting:patterns/triggers"
---

# Hierophantic Triggers

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hierophantics]]

My [[Hierophantics (Book Entry)|embedded minds]] are independent to a point, but they cannot truly think for themselves. Thus, in order to begin casting, they require a trigger of some kind. The patterns listed here are used to create these triggers, based on a variety of common occurances. Once created, a trigger can be assigned to a mind using [[Trigger Gambit]].

Some triggers, when activated, may cast their hex with an [[Iotas|iota]] relevant to the triggering event already on the [[The Stack|stack]], to allow for a more fine-grained response. The triggers that do this are listed below, along with the iota they provide:

- Damage: amount of damage
- DmgType: amount of damage
- Drop: dropped item entity
- Attack: attacked entity
- Break: position broken
- Teleport: offset vector

## [[Trigger Reflection Damage|Trigger Reflection: Damage]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,9.8 66.1,22.0 73.1,34.2 80.2,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.6s" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,9.8 66.1,22.0 73.1,34.2 80.2,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.8" y1="22.0" x2="12.1" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="13.7,28.7 10.8,26.7 13.4,25.1" fill="#fc77be"/><circle cx="9.8" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`—` → `trigger`

Creates a trigger that activates whenever I take damage. To prevent a lethal chain-reaction, does not activate if the damage was caused by overcasting.

## [[Trigger Reflection Damage Type|Trigger Reflection: Damage Type]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="41" viewBox="0 0 90.0 40.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.6" cy="9.1" r="1.6"/><circle cx="15.7" cy="9.1" r="1.6"/><circle cx="28.7" cy="9.1" r="1.6"/><circle cx="41.7" cy="9.1" r="1.6"/><circle cx="54.8" cy="9.1" r="1.6"/><circle cx="67.8" cy="9.1" r="1.6"/><circle cx="80.9" cy="9.1" r="1.6"/><circle cx="9.1" cy="20.4" r="1.6"/><circle cx="22.2" cy="20.4" r="1.6"/><circle cx="35.2" cy="20.4" r="1.6"/><circle cx="48.3" cy="20.4" r="1.6"/><circle cx="61.3" cy="20.4" r="1.6"/><circle cx="74.3" cy="20.4" r="1.6"/><circle cx="87.4" cy="20.4" r="1.6"/><circle cx="2.6" cy="31.7" r="1.6"/><circle cx="15.7" cy="31.7" r="1.6"/><circle cx="28.7" cy="31.7" r="1.6"/><circle cx="41.7" cy="31.7" r="1.6"/><circle cx="54.8" cy="31.7" r="1.6"/><circle cx="67.8" cy="31.7" r="1.6"/><circle cx="80.9" cy="31.7" r="1.6"/></g><polyline class="hexpat-base" points="9.1,20.4 15.7,31.7 28.7,31.7 35.2,20.4 28.7,9.1 15.7,9.1 9.1,20.4 22.2,20.4 35.2,20.4 48.3,20.4 54.8,9.1 61.3,20.4 67.8,31.7 74.3,20.4 67.8,9.1 54.8,9.1 41.7,9.1 48.3,20.4 54.8,31.7 67.8,31.7 80.9,31.7 74.3,20.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:274;--dur:5.9s" points="9.1,20.4 15.7,31.7 28.7,31.7 35.2,20.4 28.7,9.1 15.7,9.1 9.1,20.4 22.2,20.4 35.2,20.4 48.3,20.4 54.8,9.1 61.3,20.4 67.8,31.7 74.3,20.4 67.8,9.1 54.8,9.1 41.7,9.1 48.3,20.4 54.8,31.7 67.8,31.7 80.9,31.7 74.3,20.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.1" y1="20.4" x2="11.2" y2="24.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="12.7,26.6 10.0,24.8 12.5,23.3" fill="#fc77be"/><circle cx="9.1" cy="20.4" r="2.6" fill="#fc77be"/></svg>

`—` → `trigger`

Creates a trigger that activates whenever I take damage, but only if the damage was of a specific type. The trigger will be attuned to whatever type of damage I last received.

## [[Trigger Purification Health|Trigger Purification: Health]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="56" viewBox="0 0 90.0 56.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/><circle cx="9.8" cy="46.4" r="1.6"/><circle cx="23.9" cy="46.4" r="1.6"/><circle cx="38.0" cy="46.4" r="1.6"/><circle cx="52.0" cy="46.4" r="1.6"/><circle cx="66.1" cy="46.4" r="1.6"/><circle cx="80.2" cy="46.4" r="1.6"/></g><polyline class="hexpat-base" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,34.2 66.1,46.4 73.1,34.2 80.2,22.0 73.1,9.8 66.1,22.0 59.1,9.8 52.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:239;--dur:4.8s" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,34.2 66.1,46.4 73.1,34.2 80.2,22.0 73.1,9.8 66.1,22.0 59.1,9.8 52.0,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.8" y1="22.0" x2="12.1" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="13.7,28.7 10.8,26.7 13.4,25.1" fill="#fc77be"/><circle cx="9.8" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`num` → `trigger`

Creates a trigger that activates when my health drops below the specified value. A full, standard healthbar has a value of 20.

## [[Trigger Purification Breath|Trigger Purification: Breath]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 66.1,22.0 59.1,9.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:239;--dur:4.8s" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8 59.1,9.8 52.0,22.0 66.1,22.0 59.1,9.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.8" y1="22.0" x2="12.1" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="13.7,28.7 10.8,26.7 13.4,25.1" fill="#fc77be"/><circle cx="9.8" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`num` → `trigger`

Creates a trigger that activates when my breath drops below the specified value. A full bar of bubbles has a value of 10.

## [[Trigger Purification Hunger|Trigger Purification: Hunger]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="11.7,40.5 20.0,55.0 36.7,55.0 45.0,40.5 36.7,26.1 20.0,26.1 11.7,40.5 28.3,40.5 45.0,40.5 61.7,40.5 53.3,26.1 61.7,11.7 70.0,26.1 78.3,40.5 61.7,40.5 70.0,55.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:4.2s" points="11.7,40.5 20.0,55.0 36.7,55.0 45.0,40.5 36.7,26.1 20.0,26.1 11.7,40.5 28.3,40.5 45.0,40.5 61.7,40.5 53.3,26.1 61.7,11.7 70.0,26.1 78.3,40.5 61.7,40.5 70.0,55.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="40.5" x2="14.3" y2="45.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.2,48.5 12.7,46.1 15.9,44.2" fill="#fc77be"/><circle cx="11.7" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`num` → `trigger`

Creates a trigger that activates when my hunger drops below the specified value. A full hunger bar has a value of 20.

## [[Trigger Purification Velocity|Trigger Purification: Velocity]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 28.3,26.1 45.0,26.1 61.7,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:4.5s" points="11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 28.3,26.1 45.0,26.1 61.7,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="14.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.2,34.0 12.7,31.6 15.9,29.8" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`num` → `trigger`

Creates a trigger that activates when my velocity (measured in blocks per tick) exceeds the specified value.

## [[Trigger Purification Fall|Trigger Purification: Fall]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="10.7,23.9 18.3,37.1 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 56.4,23.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:229;--dur:4.2s" points="10.7,23.9 18.3,37.1 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 71.7,23.9 79.3,10.7 64.1,10.7 48.8,10.7 56.4,23.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="10.7" y1="23.9" x2="13.1" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="14.9,31.2 11.7,29.0 14.6,27.3" fill="#fc77be"/><circle cx="10.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`num` → `trigger`

Creates a trigger that activates when I fall further than the specified number of blocks.

## [[Trigger Reflection Drop|Trigger Reflection: Drop]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="10.7,23.9 18.3,37.1 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 71.7,23.9 64.1,10.7 56.4,23.9 48.8,10.7 64.1,10.7 79.3,10.7 71.7,23.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:259;--dur:4.8s" points="10.7,23.9 18.3,37.1 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 71.7,23.9 64.1,10.7 56.4,23.9 48.8,10.7 64.1,10.7 79.3,10.7 71.7,23.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="10.7" y1="23.9" x2="13.1" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="14.9,31.2 11.7,29.0 14.6,27.3" fill="#fc77be"/><circle cx="10.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`—` → `trigger`

Creates a trigger that activates whenever I drop an item. This does **not** include involuntary dropping, such as on death or when using [[Greater Teleport (Book Entry)|Greater Teleport]].

## [[Trigger Reflection Attack|Trigger Reflection: Attack]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="56" viewBox="0 0 90.0 56.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.8" cy="9.8" r="1.6"/><circle cx="23.9" cy="9.8" r="1.6"/><circle cx="38.0" cy="9.8" r="1.6"/><circle cx="52.0" cy="9.8" r="1.6"/><circle cx="66.1" cy="9.8" r="1.6"/><circle cx="80.2" cy="9.8" r="1.6"/><circle cx="2.8" cy="22.0" r="1.6"/><circle cx="16.9" cy="22.0" r="1.6"/><circle cx="30.9" cy="22.0" r="1.6"/><circle cx="45.0" cy="22.0" r="1.6"/><circle cx="59.1" cy="22.0" r="1.6"/><circle cx="73.1" cy="22.0" r="1.6"/><circle cx="87.2" cy="22.0" r="1.6"/><circle cx="9.8" cy="34.2" r="1.6"/><circle cx="23.9" cy="34.2" r="1.6"/><circle cx="38.0" cy="34.2" r="1.6"/><circle cx="52.0" cy="34.2" r="1.6"/><circle cx="66.1" cy="34.2" r="1.6"/><circle cx="80.2" cy="34.2" r="1.6"/><circle cx="2.8" cy="46.4" r="1.6"/><circle cx="16.9" cy="46.4" r="1.6"/><circle cx="30.9" cy="46.4" r="1.6"/><circle cx="45.0" cy="46.4" r="1.6"/><circle cx="59.1" cy="46.4" r="1.6"/><circle cx="73.1" cy="46.4" r="1.6"/><circle cx="87.2" cy="46.4" r="1.6"/></g><polyline class="hexpat-base" points="9.8,34.2 16.9,46.4 30.9,46.4 38.0,34.2 30.9,22.0 16.9,22.0 9.8,34.2 23.9,34.2 38.0,34.2 52.0,34.2 59.1,22.0 66.1,9.8 80.2,9.8 73.1,22.0 66.1,34.2 52.0,34.2 45.0,46.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:4.5s" points="9.8,34.2 16.9,46.4 30.9,46.4 38.0,34.2 30.9,22.0 16.9,22.0 9.8,34.2 23.9,34.2 38.0,34.2 52.0,34.2 59.1,22.0 66.1,9.8 80.2,9.8 73.1,22.0 66.1,34.2 52.0,34.2 45.0,46.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.8" y1="34.2" x2="12.1" y2="38.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="13.7,40.9 10.8,38.9 13.4,37.3" fill="#fc77be"/><circle cx="9.8" cy="34.2" r="2.6" fill="#fc77be"/></svg>

`—` → `trigger`

Creates a trigger that activates whenever I strike another entity.

## [[Trigger Reflection Break|Trigger Reflection: Break]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 66.1,22.0 73.1,9.8 59.1,9.8 52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:239;--dur:4.8s" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 66.1,22.0 73.1,9.8 59.1,9.8 52.0,22.0 59.1,34.2 73.1,34.2 80.2,22.0 73.1,9.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.8" y1="22.0" x2="12.1" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="13.7,28.7 10.8,26.7 13.4,25.1" fill="#fc77be"/><circle cx="9.8" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`—` → `trigger`

Creates a trigger that activates whenever I break a block.

## [[Trigger Reflection Jump|Trigger Reflection: Jump]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="10.7,23.9 18.3,37.1 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 25.9,23.9 41.2,23.9 56.4,23.9 64.1,10.7 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 56.4,23.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:229;--dur:4.2s" points="10.7,23.9 18.3,37.1 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 25.9,23.9 41.2,23.9 56.4,23.9 64.1,10.7 71.7,23.9 79.3,37.1 64.1,37.1 48.8,37.1 56.4,23.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="10.7" y1="23.9" x2="13.1" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="14.9,31.2 11.7,29.0 14.6,27.3" fill="#fc77be"/><circle cx="10.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`—` → `trigger`

Creates a trigger that activates whenever I jump.

## [[Trigger Reflection Teleport|Trigger Reflection: Teleport]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,34.2 66.1,22.0 59.1,9.8 52.0,22.0 66.1,22.0 80.2,22.0 73.1,9.8 66.1,22.0 73.1,34.2 80.2,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:5.3s" points="9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 23.9,22.0 38.0,22.0 52.0,22.0 59.1,34.2 66.1,22.0 59.1,9.8 52.0,22.0 66.1,22.0 80.2,22.0 73.1,9.8 66.1,22.0 73.1,34.2 80.2,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="9.8" y1="22.0" x2="12.1" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="13.7,28.7 10.8,26.7 13.4,25.1" fill="#fc77be"/><circle cx="9.8" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`—` → `trigger`

Creates a trigger that activates whenever I teleport. For reasons I cannot discern, this will not detect teleports shorter than 1.5 blocks in length.

## [[Inversion Purification|Inversion Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="22.0,28.8 31.2,12.9 12.9,12.9 22.0,28.8 31.2,44.7 12.9,44.7 22.0,28.8 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 58.8,28.8 77.1,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:276;--dur:4.2s" points="22.0,28.8 31.2,12.9 12.9,12.9 22.0,28.8 31.2,44.7 12.9,44.7 22.0,28.8 40.4,28.8 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 58.8,28.8 77.1,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="28.8" x2="25.0" y2="23.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.1,20.0 26.7,24.7 23.2,22.7" fill="#fc77be"/><circle cx="22.0" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`trigger` → `trigger`

Inverts the threshold on a numerical trigger. For example, 'When velocity exceeds 5.0' will become 'When velocity drops below 5.0'. Has no effect on other types of triggers.

---
### Patterns on this page

- [[Trigger Reflection Damage]] · Pattern · Reflection
- [[Trigger Reflection Damage Type]] · Pattern · Reflection
- [[Trigger Purification Health]] · Pattern · Purification
- [[Trigger Purification Breath]] · Pattern · Purification
- [[Trigger Purification Hunger]] · Pattern · Purification
- [[Trigger Purification Velocity]] · Pattern · Purification
- [[Trigger Purification Fall]] · Pattern · Purification
- [[Trigger Reflection Drop]] · Pattern · Reflection
- [[Trigger Reflection Attack]] · Pattern · Reflection
- [[Trigger Reflection Break]] · Pattern · Reflection
- [[Trigger Reflection Jump]] · Pattern · Reflection
- [[Trigger Reflection Teleport]] · Pattern · Reflection
- [[Inversion Purification]] · Pattern · Purification

**Sources:** 🧩 [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
