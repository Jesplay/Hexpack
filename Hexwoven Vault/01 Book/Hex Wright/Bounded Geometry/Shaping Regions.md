---
aliases: ["Shaping Regions"]
tags: [hexbook/entry, mod/hexwright, category/bounded-geometry, topic/blocks, topic/hexwright]
mod: "Hexwright"
category: "Bounded Geometry"
entry_id: "hexcasting:hex_wright/geometry/region_shaping"
---

# Shaping Regions

> **Category:** Hex Wright › [[Bounded Geometry (Category)|Bounded Geometry]]  
> **Source mod:** [[Hexwright]]

Regions can be combined, intersected, diffed and sliced and the result is a new region representing those changes. This makes creating complex shapes efficient, by keeping the representation of a region as maths, rather than lists of vectors.

## [[Region Union|Region Union]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 20.0,11.7 36.7,11.7 45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,40.5 20.0,40.5 11.7,26.1 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:283;--dur:4.8s" points="11.7,26.1 20.0,11.7 36.7,11.7 45.0,26.1 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 45.0,26.1 36.7,40.5 20.0,40.5 11.7,26.1 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="14.3" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.3,18.2 15.9,22.4 12.7,20.6" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`region, region` → `region`

Combines both regions into one.

## [[Region Intersection|Region Intersection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="11.7,40.5 20.0,26.1 36.7,26.1 45.0,40.5 53.3,55.0 70.0,55.0 78.3,40.5 70.0,26.1 53.3,26.1 45.0,40.5 36.7,55.0 20.0,55.0 11.7,40.5 28.3,40.5 36.7,26.1 45.0,11.7 53.3,26.1 61.7,40.5 78.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:5.0s" points="11.7,40.5 20.0,26.1 36.7,26.1 45.0,40.5 53.3,55.0 70.0,55.0 78.3,40.5 70.0,26.1 53.3,26.1 45.0,40.5 36.7,55.0 20.0,55.0 11.7,40.5 28.3,40.5 36.7,26.1 45.0,11.7 53.3,26.1 61.7,40.5 78.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="40.5" x2="14.3" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.3,32.6 15.9,36.8 12.7,35.0" fill="#fc77be"/><circle cx="11.7" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`region, region` → `region`

Creates a region that only has blocks existing in both regions.

## [[Region Difference|Region Difference]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="11.7,40.5 20.0,26.1 36.7,26.1 45.0,40.5 53.3,55.0 70.0,55.0 78.3,40.5 70.0,26.1 53.3,26.1 45.0,40.5 36.7,55.0 20.0,55.0 11.7,40.5 28.3,40.5 36.7,26.1 28.3,11.7 45.0,11.7 61.7,11.7 53.3,26.1 61.7,40.5 78.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:333;--dur:5.6s" points="11.7,40.5 20.0,26.1 36.7,26.1 45.0,40.5 53.3,55.0 70.0,55.0 78.3,40.5 70.0,26.1 53.3,26.1 45.0,40.5 36.7,55.0 20.0,55.0 11.7,40.5 28.3,40.5 36.7,26.1 28.3,11.7 45.0,11.7 61.7,11.7 53.3,26.1 61.7,40.5 78.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="40.5" x2="14.3" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.3,32.6 15.9,36.8 12.7,35.0" fill="#fc77be"/><circle cx="11.7" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`region, region` → `region`

Returns the first region with any blocks overlapping with region two removed.

## [[Region Cut|Region Cut]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,49.7 45.0,32.0 65.5,32.0 75.7,49.7 65.5,67.5 45.0,67.5 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="34.8,49.7 45.0,32.0 65.5,32.0 75.7,49.7 65.5,67.5 45.0,67.5 34.8,49.7 55.2,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="49.7" x2="38.0" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,40.0 40.0,45.2 36.1,43.0" fill="#fc77be"/><circle cx="34.8" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`region, vector, vector` → `region, region`

Splits a region along the plane through a point with a normal. Produces the positive and negative sides.

## [[Region Query|Region Query]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,15.8 38.3,15.8 49.5,35.3 38.3,54.7 15.8,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:113;--dur:1.6s" points="15.8,15.8 38.3,15.8 49.5,35.3 38.3,54.7 15.8,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="15.8" x2="23.0" y2="15.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.1,15.8 23.0,18.2 23.0,13.3" fill="#fc77be"/><circle cx="15.8" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`region, vector` → `boolean`

Queries if the vector exists in the region and returns a bool. Does not need ambit.

---
### Patterns on this page

- [[Region Union]] · Pattern · Other
- [[Region Intersection]] · Pattern · Other
- [[Region Difference]] · Pattern · Other
- [[Region Cut]] · Pattern · Other
- [[Region Query]] · Pattern · Other

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
