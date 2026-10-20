---
tags: [moc]
---

# 🏠 Hexwoven Grimoire

An Obsidian vault built from the in-game guidebook (*The Hex Notebook*) of the **Hexwoven** modpack (v6.3.0, Minecraft 1.20.1, Fabric): Hex Casting **plus every hex addon installed in the pack**.

> [!tip] New to Hex Casting?
> Read **[[Start Here - How Hexcasting Works]]** first. It explains the whole system in one page and links into everything else.

> [!question] Looking for something? Stuck?
> - **Ctrl + O** and type any pattern, item or block name (e.g. `mote nexus`, `impetus`, `thoth`). See [[Find Anything]].
> - Can't progress / Great Spells won't cast / need Flay Mind → **[[Progression - Enlightenment & Great Spells]]**
> - Mote Nexus → **[[Mote Nexus & Motes]]**

## 🧭 Main indexes
- [[Start Here - How Hexcasting Works]]: the core concepts, explained in order
- [[Example Hexes]]: six complete hexes with step-by-step stack traces
- [[Reading Order]]: the whole book as a checklist, in in-game order
- [[Pattern Naming Guide]]: how to read pattern names (Reflection, Purification, Gambit…)
- [[Gambits Explained]]: what gambits are, when to use them, and the tricky ones (Bookkeeper's, Thoth's, Iris'…) in depth
- [[Graph Recipes]]: ready-made graph view filters, and how the graph is laid out
- [[Topics]]: browse by what you want to do: movement, lists, stack, math, combat, storage…
- [[Patterns by Family]]: every pattern grouped by Reflection / Purification / Gambit …
- **Pattern Browser** (`05 Indexes/Pattern Browser.base`): a sortable, filterable table of every pattern (needs Obsidian 1.9+)
- [[All Spells]]: every spell, grouped by mod, with its cost
- [[All Great Spells]]: the world-altering per-world spells
- [[All Patterns]]: every non-spell pattern (math, stack, lists, meta-evaluation…)
- [[Iotas]]: the data types that live on the stack
- [[Components]]: items, blocks and devices
- [[Hexwoven Modpack]]: which addon adds what

## 📖 The book, chapter by chapter
- [[Getting Started (Category)|Getting Started]]: media, amethyst, geodes
- [[Hex Casting (Category)|Hex Casting]]: the stack, what a hex is, mishaps
- [[Items (Category)|Items]]: staves, foci, scrolls, phials, trinkets…
- [[Patterns (Category)|Patterns]]: every action you can draw
  - [[Spells (Category)|Spells]] · [[Great Spells (Category)|Great Spells]]
- [[The Great Work (Category)|The Great Work]]: spell circles, akashic libraries, flaying minds, enlightenment
- [[Cross-Mod Compatibility (Category)|Cross-Mod Compatibility]]
- [[Lore (Category)|Lore]]
- [[Not in the Book (Category)|Not in the Book]]: 41 patterns the mods contain but the book never mentions
- Addon chapters: [[Hex Wright (Category)|Hex Wright]] · [[Lapisworks (Category)|Lapisworks]] · [[Genie Lamp (Category)|Genie Lamp (Hexical)]]

## 🕸️ How the notes relate
| Note type | Folder | Links to |
|---|---|---|
| **Book entry** | `01 Book/<chapter>/` | its category, its mod, every pattern on the page, any entry it references |
| **Pattern / Spell** | `02 Patterns/<mod>/` (spells in `Spells/`, `Great Spells/`) | the entry describing it, its mod, the [[Iotas]] it takes and returns, sibling patterns, any pattern named in its text |
| **Iota** | `03 Iotas/` | every pattern that produces or consumes it |
| **Mod** | `04 Mods/` | all of its entries, patterns, spells and great spells |

**Topic tags** on every pattern: `#topic/movement`, `#topic/stack`, `#topic/lists`, `#topic/math`, `#topic/logic`, `#topic/control-flow`, `#topic/storage`, `#topic/sensing`, `#topic/blocks`, `#topic/combat`, `#topic/effects`, `#topic/items`, `#topic/creatures`, `#topic/visuals`, `#topic/wisps`, … (full list in [[Topics]]). Also `#family/gambit`, `#family/reflection`, … for the name families.

Tags you can filter by (and colours in the graph view): `#hexcasting/spell` 🩷 · `#hexcasting/great-spell` 🟡 · `#hexcasting/pattern` 🟣 · `#hexbook/entry` 🔵 · `#hexcasting/iota` 🟢 · `#hexcasting/mod` 🟠

> [!info] Animated patterns
> Pattern drawings trace themselves from the pink start dot. On book pages, hover a drawing to animate it. This needs the bundled CSS snippet **hexwoven** (Settings → Appearance → CSS snippets). It's switched on already; if you turn it off you just get still drawings.

Every pattern note's frontmatter holds `kind`, `family`, `mod`, `op_id`, `signature`, `start_direction`, `input`, `output`, `cost`, `iotas_in`, `iotas_out` and `entry`, so you can query them with Dataview or Bases.
