---
aliases: ["Imbuing Equipment", "Amethyst Axe", "Amethyst Hoe", "Amethyst Pickaxe", "Amethyst Shovel", "Amethyst Sword", "Amethyst Arrow", "Amethyst Helmet", "Amethyst Chestplate", "Amethyst Leggings", "Amethyst Boots"]
tags: [hexbook/entry, mod/hexchanting, category/items, topic/enchanting, topic/items]
mod: "Hexchanting"
category: "Items"
entry_id: "hexcasting:imbuing"
---

# Imbuing Equipment

> **Category:** [[Items (Category)|Items]]  
> **Source mod:** [[Hexchanting]]

I have found uses for many of Nature's resources outside of my work with **Hexes**, such as the construction of tools and armaments. It follows that I am able to do the same with [[Amethyst]]. If I can align the crystalline structure correctly they can even channel **[[Media|media]]** to cast **Hexes**.

After experimentation I have found that [[Amethyst|Charged Amethyst]] works best when crafting equipment. By substituting it for a material like iron or diamond I can craft items with comparable performance to their diamond equivalents, though they are less durable. Their true strength is how they interact with the world. When used they resonate with the latent **media** in the surrounding environment, providing a guide for **Hexes** channeled through them.

Compromises have had to be made in the creation of these tools. They must be fed with external **media** when casting else they will cannibalize their own materials, possibly to the point of destruction. This is similar to a [[Casting Items|Cypher]], though these tools can be recharged.

Regular use also slowly drains this reservoir - I believe the **media** is being expended to heal damage to the structure of the [[Amethyst]]. Conventional means of equipment repair have proved ineffective due to difficulties in integrating a new matrix into the existing crystal.

**Amethyst Axe**: shaped crafting

|   |   |
|---|---|
| `X` | `X` |
| `X` | `#` |
|   | `#` |

`X` = Charged Amethyst · `#` = Stick

**Amethyst Hoe**: shaped crafting

|   |   |
|---|---|
| `X` | `X` |
|   | `#` |
|   | `#` |

`X` = Charged Amethyst · `#` = Stick

**Amethyst Pickaxe**: shaped crafting

|   |   |   |
|---|---|---|
| `X` | `X` | `X` |
|   | `#` |   |
|   | `#` |   |

`X` = Charged Amethyst · `#` = Stick

**Amethyst Shovel**: shaped crafting

|   |
|---|
| `X` |
| `#` |
| `#` |

`X` = Charged Amethyst · `#` = Stick

Axes, hoes, pickaxes and shovels cast after breaking a block. The [[The Stack|stack]] is initialized with the location of the broken block. Break block spells cast through these tools behave as if broken by the tool, which includes any enchantment effects.

**Amethyst Sword**: shaped crafting

|   |
|---|
| `X` |
| `X` |
| `#` |

`X` = Charged Amethyst · `#` = Stick

Swords cast on hit and push the targeted entity to the stack.

My attempts to craft a bow on similar principles have failed. [[Amethyst]] simply doesn't have the flexibility required. I have instead resorted to crafting arrows with [[Amethyst|Amethyst Shards]], in place of flint. These can be imbued like my other amethyst equipment but they can only draw on their internal **media** and affect a limited area. I am sure I could craft an equivalent out of **media** alone, but there are niches where I may find this useful.

**Amethyst Arrow** ×4: shaped crafting

|   |
|---|
| `X` |
| `#` |
| `Y` |

`X` = Amethyst Shard · `#` = Stick · `Y` = Feather

Arrows cast when striking an entity or a block. In the first case the entity is pushed to the stack, in the latter the block position. Arrows have an [[Amethyst|Amethyst Shard]] worth of **media** and a 4 block ambit radius.

**Amethyst Helmet**: shaped crafting

|   |   |   |
|---|---|---|
| `X` | `X` | `X` |
| `X` |   | `X` |

`X` = Charged Amethyst

Helmets cast when a mob, hostile or otherwise, begins to target me. The identity of the mob is pushed to the stack.

**Amethyst Chestplate**: shaped crafting

|   |   |   |
|---|---|---|
| `X` |   | `X` |
| `X` | `X` | `X` |
| `X` | `X` | `X` |

`X` = Charged Amethyst

Chestplates cast when I take damage. Two nullable entities and the amount of damage are pushed to the stack. The first entity is the direct source of the damage, such as an arrow, and the second is the indirect source, such as whoever fired the arrow.

**Amethyst Leggings**: shaped crafting

|   |   |   |
|---|---|---|
| `X` | `X` | `X` |
| `X` |   | `X` |
| `X` |   | `X` |

`X` = Charged Amethyst

Leggings cast in the unfortunate event of my death. The source of my demise is pushed to the stack, in the same form as the chestplate. Because I am no longer alive to serve as a conduit this will always use the legging's internal **media**.

**Amethyst Boots**: shaped crafting

|   |   |   |
|---|---|---|
| `X` |   | `X` |
| `X` |   | `X` |

`X` = Charged Amethyst

Boots cast whenever I fall about 1.5 blocks, pushing the precise distance to the stack. This trigger is sensitive and can be fooled into resetting by going up.

## [[Imbue Equipment|Imbue Equipment]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="61.7,40.5 45.0,40.5 53.3,26.1 45.0,11.7 36.7,26.1 45.0,40.5 28.3,40.5 20.0,55.0 36.7,55.0 45.0,40.5 53.3,55.0 70.0,55.0 61.7,40.5 70.0,26.1 53.3,26.1 61.7,11.7 45.0,11.7 28.3,11.7 36.7,26.1 20.0,26.1 28.3,40.5 11.7,40.5 20.0,55.0 28.3,69.4 36.7,55.0 45.0,69.4 53.3,55.0 61.7,69.4 70.0,55.0 78.3,40.5 61.7,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:500;--dur:6.0s" points="61.7,40.5 45.0,40.5 53.3,26.1 45.0,11.7 36.7,26.1 45.0,40.5 28.3,40.5 20.0,55.0 36.7,55.0 45.0,40.5 53.3,55.0 70.0,55.0 61.7,40.5 70.0,26.1 53.3,26.1 61.7,11.7 45.0,11.7 28.3,11.7 36.7,26.1 20.0,26.1 28.3,40.5 11.7,40.5 20.0,55.0 28.3,69.4 36.7,55.0 45.0,69.4 53.3,55.0 61.7,69.4 70.0,55.0 78.3,40.5 61.7,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="61.7" y1="40.5" x2="56.3" y2="40.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="52.5,40.5 56.3,38.7 56.3,42.4" fill="#fc77be"/><circle cx="61.7" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`[pattern]` → `—`

Imbues the supplied hexes into the [[Amethyst]] equipment in my other hand, so that they will be cast whenever the item is used. Costs 9 [[Amethyst|Charged Amethyst]] except on arrows, which cost 5 [[Amethyst|Shards]].

---
### Patterns on this page

- [[Imbue Equipment]] · Spell · Spell

**Sources:** 🧩 [Hexchanting on Modrinth](https://modrinth.com/mod/UlOmzEJF)
