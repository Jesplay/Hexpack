---
aliases: ["Executing Regions"]
tags: [hexbook/entry, mod/hexwright, category/bounded-geometry, topic/blocks, topic/hexwright]
mod: "Hexwright"
category: "Bounded Geometry"
entry_id: "hexcasting:hex_wright/geometry/region_world"
---

# Executing Regions

> **Category:** Hex Wright › [[Bounded Geometry (Category)|Bounded Geometry]]  
> **Source mod:** [[Hexwright]]

These patterns take a region and run something against it. Unlike shaping a region, these patterns rely on ambit.

## [[Reach Purification|Reach Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 36.7,26.1 45.0,40.5 53.3,55.0 61.7,69.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:367;--dur:6.0s" points="28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 36.7,26.1 45.0,40.5 53.3,55.0 61.7,69.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.3" y1="11.7" x2="25.7" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.8,19.6 24.1,15.4 27.3,17.2" fill="#fc77be"/><circle cx="28.3" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`region` → `boolean`

Returns true if the whole region lies within your ambit, false if any part of it is out of reach.

## [[Entities Within|Entities Within]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 45.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.0" cy="11.0" r="1.6"/><circle cx="26.7" cy="11.0" r="1.6"/><circle cx="42.4" cy="11.0" r="1.6"/><circle cx="3.1" cy="24.6" r="1.6"/><circle cx="18.8" cy="24.6" r="1.6"/><circle cx="34.6" cy="24.6" r="1.6"/><circle cx="11.0" cy="38.2" r="1.6"/><circle cx="26.7" cy="38.2" r="1.6"/><circle cx="42.4" cy="38.2" r="1.6"/><circle cx="3.1" cy="51.8" r="1.6"/><circle cx="18.8" cy="51.8" r="1.6"/><circle cx="34.6" cy="51.8" r="1.6"/><circle cx="11.0" cy="65.4" r="1.6"/><circle cx="26.7" cy="65.4" r="1.6"/><circle cx="42.4" cy="65.4" r="1.6"/><circle cx="3.1" cy="79.0" r="1.6"/><circle cx="18.8" cy="79.0" r="1.6"/><circle cx="34.6" cy="79.0" r="1.6"/></g><polyline class="hexpat-base" points="11.0,11.0 26.7,11.0 34.6,24.6 26.7,38.2 11.0,38.2 18.8,51.8 26.7,65.4 18.8,79.0 11.0,65.4 18.8,51.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:141;--dur:2.5s" points="11.0,11.0 26.7,11.0 34.6,24.6 26.7,38.2 11.0,38.2 18.8,51.8 26.7,65.4 18.8,79.0 11.0,65.4 18.8,51.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.0" y1="11.0" x2="16.0" y2="11.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.6,11.0 16.0,12.7 16.0,9.3" fill="#fc77be"/><circle cx="11.0" cy="11.0" r="2.6" fill="#fc77be"/></svg>

`region` → `[entities]`

Returns a list of all entities within the specified region, ordered from centre out.

## [[Region Thoth|Region Thoth]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,13.0 59.2,13.0 68.5,29.0 59.2,45.0 40.7,45.0 50.0,61.0 31.5,61.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.8s" points="40.7,13.0 59.2,13.0 68.5,29.0 59.2,45.0 40.7,45.0 50.0,61.0 31.5,61.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="13.0" x2="46.6" y2="13.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="50.9,13.0 46.6,15.0 46.6,10.9" fill="#fc77be"/><circle cx="40.7" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`region, [pattern]` → `list`

Runs a pattern over the region exactly as a thoth would, seeds the [[The Stack|stack]] with each vector in a region.
Not a real thoth.

### One Vector At A Time

Using a Region Thoth turns a purely mathematical representation of a region into a list of a vectors that's iterated over. This means the stack length limit comes into effect.

### Bulk Edits

Region Fill and Region Break skip the per-position walk of a Region Thoth entirely, changing every block in the region in one go. Cost is the same as if you used a Thoth and individually broke or placed blocks.

## [[Region Fill|Region Fill]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="68.0,28.8 58.8,44.7 40.4,44.7 31.2,28.8 40.4,12.9 58.8,12.9 68.0,28.8 49.6,28.8 40.4,12.9 22.0,12.9 12.9,28.8 22.0,44.7 31.2,60.6 49.6,60.6 68.0,60.6 77.1,44.7 68.0,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:294;--dur:4.5s" points="68.0,28.8 58.8,44.7 40.4,44.7 31.2,28.8 40.4,12.9 58.8,12.9 68.0,28.8 49.6,28.8 40.4,12.9 22.0,12.9 12.9,28.8 22.0,44.7 31.2,60.6 49.6,60.6 68.0,60.6 77.1,44.7 68.0,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="68.0" y1="28.8" x2="65.0" y2="33.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="62.9,37.5 63.3,32.8 66.8,34.9" fill="#fc77be"/><circle cx="68.0" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`region` → `—`

Fills every replaceable block in the region and operates the same as place block.

## [[Region Break|Region Break]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="22.0,28.8 40.4,28.8 49.6,12.9 31.2,12.9 22.0,28.8 31.2,44.7 49.6,44.7 58.8,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 58.8,60.6 40.4,60.6 22.0,60.6 12.9,44.7 22.0,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:294;--dur:4.5s" points="22.0,28.8 40.4,28.8 49.6,12.9 31.2,12.9 22.0,28.8 31.2,44.7 49.6,44.7 58.8,28.8 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7 58.8,60.6 40.4,60.6 22.0,60.6 12.9,44.7 22.0,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="28.8" x2="27.9" y2="28.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,28.8 27.9,30.8 27.9,26.7" fill="#fc77be"/><circle cx="22.0" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`region` → `—`

Breaks every block in the region, dropping what each would drop. Air and unbreakable blocks are skipped and not charged for.

---
### Patterns on this page

- [[Reach Purification]] · Pattern · Purification
- [[Entities Within]] · Pattern · Other
- [[Region Thoth]] · Pattern · Other
- [[Region Fill]] · Pattern · Other
- [[Region Break]] · Pattern · Other

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
