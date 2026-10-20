---
tags: [guide]
---

# Graph Recipes

The graph view's default filter hides the hub notes (guides, indexes, mods, iota types, and pages like *The Stack* or *Media* that everything links to). What's left is a tree: **chapter → book page → patterns**.

Colours: ⭐ **bright gold** = Great Spell · 🩷 **hot pink** = Spell · muted grey-purple = other patterns · muted blue = book page · cyan = chapter. Spells are deliberately the loudest colours so they stand out.

> [!note] Node size
> Obsidian sizes graph nodes only by their number of links; it can't be set per tag. To make spells *easier to find*, use colour (above) or the **Spells only** recipe below.

To use a recipe, copy it into the graph view's **Filters → search** box (keep or remove the default `-path:` parts).

| What you want to see | Filter |
|---|---|
| Only Hex Casting itself | `path:"01 Book" -path:"Hex Wright" -path:"Lapisworks" -path:"Genie Lamp"` |
| One addon's patterns + pages | `path:"02 Patterns/Hexal" OR path:"04 Mods/Hexal"` |
| Everything about movement | `tag:#topic/movement` |
| All gambits | `tag:#family/gambit` |
| All spells and great spells | `tag:#hexcasting/spell OR tag:#hexcasting/great-spell` |
| **Spells only, with their book pages and chapters** | `tag:#hexcasting/spell OR tag:#hexcasting/great-spell OR tag:#hexbook/entry OR tag:#hexbook/category` |
| How patterns share data types | `path:"03 Iotas" OR path:"02 Patterns/Hex Casting"` |
| Book structure only (no patterns) | `path:"01 Book"` |

> [!tip] Local graph
> For a single note, the **local graph** (right-click a tab → *Open local graph*) is usually more useful than the global one: set depth to 1–2 to see a pattern's book page, siblings and data types.

> [!tip] If the graph still looks like a hairball
> Open the graph settings → **Forces** and raise *Repel force*, or lower *Link distance*. *Text fade threshold* controls when labels appear while zooming.
