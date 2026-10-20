---
aliases: ["Meshes"]
tags: [hexbook/entry, mod/hexical, category/great-spells, topic/visuals]
mod: "Hexical"
category: "Great Spells"
entry_id: "hexcasting:patterns/great_spells/meshes"
---

# Meshes

> **Category:** Patterns › [[Great Spells (Category)|Great Spells]]  
> **Source mod:** [[Hexical]]

Meshes are the natural evolution of [[Specks|specks]]. Rather than being restricted to [[Iotas|iotas]] and patterns on a two-dimensional plane, meshes accept a list of vectors in three-dimensional space and connect them with lines, allowing for infinite creative constructions. It's also responsive to most patterns used for manipulation of specks.

## [[Conjure Mesh|Conjure Mesh]] ⭐ *Great Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="87" viewBox="0 0 90.0 87.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.7" cy="10.7" r="1.6"/><circle cx="25.9" cy="10.7" r="1.6"/><circle cx="41.2" cy="10.7" r="1.6"/><circle cx="56.4" cy="10.7" r="1.6"/><circle cx="71.7" cy="10.7" r="1.6"/><circle cx="86.9" cy="10.7" r="1.6"/><circle cx="3.1" cy="23.9" r="1.6"/><circle cx="18.3" cy="23.9" r="1.6"/><circle cx="33.6" cy="23.9" r="1.6"/><circle cx="48.8" cy="23.9" r="1.6"/><circle cx="64.1" cy="23.9" r="1.6"/><circle cx="79.3" cy="23.9" r="1.6"/><circle cx="10.7" cy="37.1" r="1.6"/><circle cx="25.9" cy="37.1" r="1.6"/><circle cx="41.2" cy="37.1" r="1.6"/><circle cx="56.4" cy="37.1" r="1.6"/><circle cx="71.7" cy="37.1" r="1.6"/><circle cx="86.9" cy="37.1" r="1.6"/><circle cx="3.1" cy="50.3" r="1.6"/><circle cx="18.3" cy="50.3" r="1.6"/><circle cx="33.6" cy="50.3" r="1.6"/><circle cx="48.8" cy="50.3" r="1.6"/><circle cx="64.1" cy="50.3" r="1.6"/><circle cx="79.3" cy="50.3" r="1.6"/><circle cx="10.7" cy="63.5" r="1.6"/><circle cx="25.9" cy="63.5" r="1.6"/><circle cx="41.2" cy="63.5" r="1.6"/><circle cx="56.4" cy="63.5" r="1.6"/><circle cx="71.7" cy="63.5" r="1.6"/><circle cx="86.9" cy="63.5" r="1.6"/><circle cx="3.1" cy="76.7" r="1.6"/><circle cx="18.3" cy="76.7" r="1.6"/><circle cx="33.6" cy="76.7" r="1.6"/><circle cx="48.8" cy="76.7" r="1.6"/><circle cx="64.1" cy="76.7" r="1.6"/><circle cx="79.3" cy="76.7" r="1.6"/></g><polyline class="hexpat-base" points="33.6,50.3 48.8,50.3 56.4,37.1 41.2,37.1 33.6,50.3 41.2,63.5 56.4,63.5 64.1,50.3 56.4,37.1 48.8,23.9 33.6,23.9 25.9,37.1 33.6,50.3 18.3,50.3 10.7,37.1 18.3,23.9 25.9,10.7 41.2,10.7 56.4,10.7 64.1,23.9 56.4,37.1 71.7,37.1 79.3,50.3 71.7,63.5 64.1,76.7 48.8,76.7 33.6,76.7 25.9,63.5 33.6,50.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:427;--dur:6.0s" points="33.6,50.3 48.8,50.3 56.4,37.1 41.2,37.1 33.6,50.3 41.2,63.5 56.4,63.5 64.1,50.3 56.4,37.1 48.8,23.9 33.6,23.9 25.9,37.1 33.6,50.3 18.3,50.3 10.7,37.1 18.3,23.9 25.9,10.7 41.2,10.7 56.4,10.7 64.1,23.9 56.4,37.1 71.7,37.1 79.3,50.3 71.7,63.5 64.1,76.7 48.8,76.7 33.6,76.7 25.9,63.5 33.6,50.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="33.6" y1="50.3" x2="38.4" y2="50.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.9,50.3 38.4,52.0 38.4,48.6" fill="#fc77be"/><circle cx="33.6" cy="50.3" r="2.6" fill="#fc77be"/></svg>

`vector` → `entity`

Conjures a mesh with the location and leaves a mesh entity on the [[The Stack|stack]]. Costs about one [[Amethyst|Amethyst Dust]].

When initally conjured, the mesh is invisible and must be woven to gain form. The constraints for the list of vectors provided to a mesh are quite minimal. The list must be no more than 32 vectors long, and those vectors must have magnitudes less than 10. The mesh will then connect each of those points.

## [[Weave Mesh|Weave Mesh]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="89" viewBox="0 0 90.0 89.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/><circle cx="12.9" cy="76.5" r="1.6"/><circle cx="31.2" cy="76.5" r="1.6"/><circle cx="49.6" cy="76.5" r="1.6"/><circle cx="68.0" cy="76.5" r="1.6"/><circle cx="86.3" cy="76.5" r="1.6"/></g><polyline class="hexpat-base" points="40.4,60.6 58.8,60.6 68.0,44.7 49.6,44.7 40.4,60.6 49.6,76.5 68.0,76.5 77.1,60.6 68.0,44.7 58.8,28.8 40.4,28.8 31.2,44.7 40.4,60.6 22.0,60.6 12.9,44.7 22.0,28.8 31.2,12.9 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:367;--dur:5.6s" points="40.4,60.6 58.8,60.6 68.0,44.7 49.6,44.7 40.4,60.6 49.6,76.5 68.0,76.5 77.1,60.6 68.0,44.7 58.8,28.8 40.4,28.8 31.2,44.7 40.4,60.6 22.0,60.6 12.9,44.7 22.0,28.8 31.2,12.9 49.6,12.9 68.0,12.9 77.1,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="60.6" x2="46.3" y2="60.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="50.5,60.6 46.3,62.6 46.3,58.6" fill="#fc77be"/><circle cx="40.4" cy="60.6" r="2.6" fill="#fc77be"/></svg>

`entity, list of vectors` → `—`

Takes a list of vectors, with constraints defined on the prior page, and weaves the mesh into that shape. Free.

## [[Tangle Purification|Tangle Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="89" viewBox="0 0 90.0 89.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/><circle cx="12.9" cy="76.5" r="1.6"/><circle cx="31.2" cy="76.5" r="1.6"/><circle cx="49.6" cy="76.5" r="1.6"/><circle cx="68.0" cy="76.5" r="1.6"/><circle cx="86.3" cy="76.5" r="1.6"/></g><polyline class="hexpat-base" points="68.0,44.7 58.8,60.6 40.4,60.6 49.6,44.7 68.0,44.7 77.1,60.6 68.0,76.5 49.6,76.5 40.4,60.6 31.2,44.7 40.4,28.8 58.8,28.8 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 31.2,12.9 22.0,28.8 12.9,44.7 22.0,60.6 40.4,60.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:367;--dur:5.6s" points="68.0,44.7 58.8,60.6 40.4,60.6 49.6,44.7 68.0,44.7 77.1,60.6 68.0,76.5 49.6,76.5 40.4,60.6 31.2,44.7 40.4,28.8 58.8,28.8 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 31.2,12.9 22.0,28.8 12.9,44.7 22.0,60.6 40.4,60.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="68.0" y1="44.7" x2="65.0" y2="49.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="62.9,53.4 63.3,48.8 66.8,50.8" fill="#fc77be"/><circle cx="68.0" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`entity` → `list of vectors`

Pushes the shape of a mesh as a list of vectors.

---
### Patterns on this page

- [[Conjure Mesh]] · Great Spell · Great Spell
- [[Weave Mesh]] · Pattern · Other
- [[Tangle Purification]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/great_spells/meshes)
