---
aliases: ["Regions"]
tags: [hexbook/entry, mod/hexwright, category/bounded-geometry, topic/blocks, topic/hexwright]
mod: "Hexwright"
category: "Bounded Geometry"
entry_id: "hexcasting:hex_wright/geometry/regions"
---

# Regions

> **Category:** Hex Wright › [[Bounded Geometry (Category)|Bounded Geometry]]  
> **Source mod:** [[Hexwright]]

A region is a bounded piece of space, represented as an [[Iotas|iota]].

Instead of it being a list of vectors, it's stored as a mathematical representation of the space, opening up the possibility of areas larger than a list of 1024 vectors.

### Ambit

Making and combining regions requires no ambit and costs no [[Media|media]], it's a pure mathematical construct. You can draw a sphere a thousand blocks wide, a thousand blocks away and cut it to pieces.

Ambit only matters when interacting with the world in that region.

## [[Sphere Region|Sphere Region]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 53.3,26.1 61.7,40.5 53.3,55.0 36.7,55.0 28.3,40.5 36.7,26.1 53.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:317;--dur:5.3s" points="61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 53.3,26.1 61.7,40.5 53.3,55.0 36.7,55.0 28.3,40.5 36.7,26.1 53.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="61.7" y1="11.7" x2="56.3" y2="11.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="52.5,11.7 56.3,9.8 56.3,13.5" fill="#fc77be"/><circle cx="61.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `region`

Vector marks the centre and the number is the radius. A radius of 0 will give 1.

## [[Box Region|Box Region]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 53.3,26.1 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 45.0,40.5 28.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:5.9s" points="61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 53.3,26.1 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 45.0,40.5 28.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="61.7" y1="11.7" x2="56.3" y2="11.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="52.5,11.7 56.3,9.8 56.3,13.5" fill="#fc77be"/><circle cx="61.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`vector, vector` → `region`

The axis-aligned box between two opposite corners. Which corner you give first makes no difference.

## [[Cylinder Region|Cylinder Region]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 53.3,26.1 45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 20.0,55.0 36.7,55.0 45.0,69.4 53.3,55.0 70.0,55.0 61.7,40.5 70.0,26.1 53.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:417;--dur:6.0s" points="61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 53.3,26.1 45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 20.0,55.0 36.7,55.0 45.0,69.4 53.3,55.0 70.0,55.0 61.7,40.5 70.0,26.1 53.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="61.7" y1="11.7" x2="56.3" y2="11.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="52.5,11.7 56.3,9.8 56.3,13.5" fill="#fc77be"/><circle cx="61.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`vector, vector, number` → `region`

Vectors are two points the cylinder runs between and the number is the radius

A cylinder is not confined to the three axes. Give it the endpoints of a diagonal and you get a diagonal cylinder.

Two endpoints at the same place results in a [[Mishaps|mishap]] rather than a very short cylinder.

---
### Patterns on this page

- [[Sphere Region]] · Pattern · Other
- [[Box Region]] · Pattern · Other
- [[Cylinder Region]] · Pattern · Other

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
