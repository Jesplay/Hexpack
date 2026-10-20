---
aliases: ["Hextra Vector Math"]
tags: [hexbook/entry, mod/hextra-patterns, category/patterns, topic/math]
mod: "Hextra Patterns"
category: "Patterns"
entry_id: "hexcasting:patterns/hextrapats_vec_math"
---

# Hextra Vector Math

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hextra Patterns]]

## [[Norm Purification|Norm Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="75.7" y1="32.0" x2="72.4" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="70.1,41.8 70.5,36.6 74.4,38.8" fill="#fc77be"/><circle cx="75.7" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vec` → `vec`

Sets the length of the vector to one. Does nothing if the vector is (0, 0, 0).

## [[Length Distillation|Length Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 38.3,54.7 49.5,35.3 38.3,15.8 27.0,35.3 38.3,54.7 49.5,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="27.0,74.2 49.5,74.2 60.8,54.7 49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 38.3,54.7 49.5,35.3 38.3,15.8 27.0,35.3 38.3,54.7 49.5,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="74.2" x2="34.2" y2="74.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,74.2 34.2,76.7 34.2,71.8" fill="#fc77be"/><circle cx="27.0" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `vec`

Sets the length of a vector. Does nothing if the vector is (0, 0, 0).

## [[Incrementation Purification|Incrementation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,64.4 31.8,41.5 45.0,18.5 18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:265;--dur:2.8s" points="18.5,64.4 31.8,41.5 45.0,18.5 18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="64.4" x2="22.8" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,51.8 25.3,58.5 20.2,55.6" fill="#fc77be"/><circle cx="18.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`vec` → `vec`

Increases length by 1. Does nothing if the vector is (0, 0, 0).

## [[Decrementation Purification|Decrementation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="71.5,64.4 58.2,41.5 45.0,18.5 71.5,18.5 58.2,41.5 45.0,64.4 31.8,41.5 18.5,18.5 45.0,18.5 31.8,41.5 18.5,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:265;--dur:2.8s" points="71.5,64.4 58.2,41.5 45.0,18.5 71.5,18.5 58.2,41.5 45.0,64.4 31.8,41.5 18.5,18.5 45.0,18.5 31.8,41.5 18.5,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.5" y1="64.4" x2="67.2" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="64.2,51.8 69.8,55.6 64.7,58.5" fill="#fc77be"/><circle cx="71.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`vec` → `vec`

Decreases length by 1. Does nothing if the vector is (0, 0, 0).

## [[Chaos Reflection|Chaos Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 77.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="11.0" r="1.6"/><circle cx="18.8" cy="11.0" r="1.6"/><circle cx="34.6" cy="11.0" r="1.6"/><circle cx="50.3" cy="11.0" r="1.6"/><circle cx="66.0" cy="11.0" r="1.6"/><circle cx="11.0" cy="24.6" r="1.6"/><circle cx="26.7" cy="24.6" r="1.6"/><circle cx="42.4" cy="24.6" r="1.6"/><circle cx="58.1" cy="24.6" r="1.6"/><circle cx="73.8" cy="24.6" r="1.6"/><circle cx="3.1" cy="38.2" r="1.6"/><circle cx="18.8" cy="38.2" r="1.6"/><circle cx="34.6" cy="38.2" r="1.6"/><circle cx="50.3" cy="38.2" r="1.6"/><circle cx="66.0" cy="38.2" r="1.6"/><circle cx="11.0" cy="51.8" r="1.6"/><circle cx="26.7" cy="51.8" r="1.6"/><circle cx="42.4" cy="51.8" r="1.6"/><circle cx="58.1" cy="51.8" r="1.6"/><circle cx="73.8" cy="51.8" r="1.6"/><circle cx="3.1" cy="65.4" r="1.6"/><circle cx="18.8" cy="65.4" r="1.6"/><circle cx="34.6" cy="65.4" r="1.6"/><circle cx="50.3" cy="65.4" r="1.6"/><circle cx="66.0" cy="65.4" r="1.6"/><circle cx="11.0" cy="79.0" r="1.6"/><circle cx="26.7" cy="79.0" r="1.6"/><circle cx="42.4" cy="79.0" r="1.6"/><circle cx="58.1" cy="79.0" r="1.6"/><circle cx="73.8" cy="79.0" r="1.6"/></g><polyline class="hexpat-base" points="42.4,51.8 58.1,51.8 66.0,65.4 58.1,79.0 42.4,79.0 34.6,65.4 42.4,51.8 34.6,38.2 42.4,24.6 34.6,11.0 18.8,11.0 11.0,24.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:173;--dur:3.1s" points="42.4,51.8 58.1,51.8 66.0,65.4 58.1,79.0 42.4,79.0 34.6,65.4 42.4,51.8 34.6,38.2 42.4,24.6 34.6,11.0 18.8,11.0 11.0,24.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="42.4" y1="51.8" x2="47.4" y2="51.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="51.0,51.8 47.4,53.5 47.4,50.1" fill="#fc77be"/><circle cx="42.4" cy="51.8" r="2.6" fill="#fc77be"/></svg>

`—` → `vec`

Returns a unit vector facing a random direction.

## [[Distance Distillation|Distance Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="44" viewBox="0 0 90.0 44.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/></g><polyline class="hexpat-base" points="23.9,22.0 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 52.0,22.0 59.1,9.8 73.1,9.8 80.2,22.0 73.1,34.2 59.1,34.2 52.0,22.0 66.1,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:211;--dur:4.2s" points="23.9,22.0 38.0,22.0 30.9,9.8 16.9,9.8 9.8,22.0 16.9,34.2 30.9,34.2 38.0,22.0 52.0,22.0 59.1,9.8 73.1,9.8 80.2,22.0 73.1,34.2 59.1,34.2 52.0,22.0 66.1,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.9" y1="22.0" x2="28.4" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="31.6,22.0 28.4,23.6 28.4,20.5" fill="#fc77be"/><circle cx="23.9" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`vec, vec` → `num`

Returns the distance between two vectors.

## [[Approach Exaltation|Approach Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0 55.2,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,32.0 24.5,49.7 14.3,32.0 24.5,14.3 34.8,32.0 55.2,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="31.5" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,41.8 29.6,36.6 33.4,38.8" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vec, vec, num` → `vec`

Moves the first vector towards the second by a given distance. Will not overshoot.

## [[Theta Distillation|Theta Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="65.5,67.5 75.7,49.7 55.2,49.7 34.8,49.7 45.0,32.0 24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7 45.0,32.0 55.2,14.3 34.8,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="65.5,67.5 75.7,49.7 55.2,49.7 34.8,49.7 45.0,32.0 24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7 45.0,32.0 55.2,14.3 34.8,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="67.5" x2="68.7" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,57.7 70.7,62.9 66.8,60.7" fill="#fc77be"/><circle cx="65.5" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`vec, vec` → `num`

Returns the shortest angular distance on a sphere between two vectors in radians

## [[Turning Exaltation|Turning Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="65.5,67.5 75.7,49.7 55.2,49.7 34.8,49.7 45.0,32.0 24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7 45.0,32.0 34.8,14.3 55.2,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="65.5,67.5 75.7,49.7 55.2,49.7 34.8,49.7 45.0,32.0 24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7 45.0,32.0 34.8,14.3 55.2,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="67.5" x2="68.7" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,57.7 70.7,62.9 66.8,60.7" fill="#fc77be"/><circle cx="65.5" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`vec, vec, num` → `vec`

Rotates the first vector towards the direction of the second vector by the given number of radians. [[Mishaps|Mishaps]] if the two directions are directly opposite to each other.

For example:
- (1, 0, 0), (0, 1, 0), pi/4

→ (0.71, 0.71, 0)
- (2, 0, 0), (0, 5, 0), pi/4

→ (1.41, 1.41, 0)
- (1, 1, 1), (0, 5, 5), pi/6

→ (0.16, 1.22, 1.22)

## [[Polar Distillation|Polar Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 65.5,14.3 45.0,14.3 34.8,32.0 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="14.3,32.0 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 65.5,14.3 45.0,14.3 34.8,32.0 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="20.9" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,32.0 20.9,34.3 20.9,29.8" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`num, num` → `vec`

Creates a unit vector from two polar angles, pitch and yaw, in radians. +Y is up. 0 yaw is +Z, pi/2 yaw is -X. Yet again, mentions of [[Sextant Decomposition|F3]] turn up.

## [[Polar Decomposition|Polar Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 34.8,32.0 45.0,14.3 65.5,14.3 55.2,32.0 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="14.3,32.0 34.8,32.0 45.0,14.3 65.5,14.3 55.2,32.0 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="20.9" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,32.0 20.9,34.3 20.9,29.8" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vec` → `num, num`

Converts a vector's direction into two polar angles, pitch and yaw, in radians. +Y is up. 0 yaw is +Z, pi/2 yaw is -X. Yet again, mentions of [[Sextant Decomposition|F3]] turn up.

## [[Pitch Distillation|Pitch Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="45.0,14.3 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="41.7" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,24.1 39.8,18.9 43.7,21.1" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `vec`

Rotates the vector by the given angle in radians around the X axis.

## [[Yaw Distillation|Yaw Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="36.7,11.7 28.3,26.1 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:167;--dur:2.8s" points="36.7,11.7 28.3,26.1 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="11.7" x2="34.0" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,19.6 32.4,15.4 35.6,17.2" fill="#fc77be"/><circle cx="36.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `vec`

Rotates the vector by the given angle in radians around the Y axis.

## [[Roll Distillation|Roll Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="45.0,14.3 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="41.7" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,24.1 39.8,18.9 43.7,21.1" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `vec`

Rotates the vector by the given angle in radians around the Z axis.

## [[Pitch Purification|Pitch Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,49.7 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="45.0,49.7 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="49.7" x2="41.7" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,40.0 43.7,43.0 39.8,45.2" fill="#fc77be"/><circle cx="45.0" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`num` → `vec`

Converts an angle in radians to a unit vector where X=0. +Z at 0 radians and +Y at pi/4 radians.

## [[Yaw Purification|Yaw Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="36.7,40.5 28.3,26.1 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:167;--dur:2.8s" points="36.7,40.5 28.3,26.1 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="40.5" x2="34.0" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,32.6 35.6,35.0 32.4,36.8" fill="#fc77be"/><circle cx="36.7" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`num` → `vec`

Converts an angle in radians to a unit vector where Y=0. +Z at 0 radians and -X at pi/4 radians. Yet again, mentions of [[Sextant Decomposition|F3]] turn up.

## [[Roll Purification|Roll Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,49.7 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="45.0,49.7 34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0 65.5,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="49.7" x2="41.7" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,40.0 43.7,43.0 39.8,45.2" fill="#fc77be"/><circle cx="45.0" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`num` → `vec`

Converts an angle in radians to a unit vector where Z=0. -X at 0 radians and +Y at pi/4 radians.

---
### Patterns on this page

- [[Norm Purification]] · Pattern · Purification
- [[Length Distillation]] · Pattern · Distillation
- [[Incrementation Purification]] · Pattern · Purification
- [[Decrementation Purification]] · Pattern · Purification
- [[Chaos Reflection]] · Pattern · Reflection
- [[Distance Distillation]] · Pattern · Distillation
- [[Approach Exaltation]] · Pattern · Exaltation
- [[Theta Distillation]] · Pattern · Distillation
- [[Turning Exaltation]] · Pattern · Exaltation
- [[Polar Distillation]] · Pattern · Distillation
- [[Polar Decomposition]] · Pattern · Decomposition
- [[Pitch Distillation]] · Pattern · Distillation
- [[Yaw Distillation]] · Pattern · Distillation
- [[Roll Distillation]] · Pattern · Distillation
- [[Pitch Purification]] · Pattern · Purification
- [[Yaw Purification]] · Pattern · Purification
- [[Roll Purification]] · Pattern · Purification

**Sources:** 🧩 [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
