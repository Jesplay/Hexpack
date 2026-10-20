---
aliases: ["An Example"]
tags: [hexcasting/pattern, mod/hex-casting, family/other, topic/sensing]
kind: "Pattern"
family: "Other"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: ""
signature: "qaq"
start_direction: "NORTH_EAST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Hex Casting 101]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# An Example

◇ **Pattern** · [[Hex Casting]] · 📖 [[Hex Casting 101]] · Hex Casting

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,132.0 92.3,85.0 65.1,38.0 38.0,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="65.1,132.0 92.3,85.0 65.1,38.0 38.0,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="132.0" x2="73.8" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="80.1,106.2 79.0,119.9 68.6,114.0" fill="#fc77be"/><circle cx="65.1" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)

## Description

It's interesting to note that the rotation of a pattern doesn't seem to matter at all. These two patterns both perform an action called [[Mind's Reflection]], for example.

A **Hex** is cast by drawing (valid) actions in sequence. Each action might do one of a few things:
- Gather some information about the environment, leaving it on the top of the [[The Stack|stack]];
- manipulate the info gathered (e.g. adding two numbers); or
- perform some magical effect, like summoning lightning or an explosion. (These actions are called "spells.")

When I start casting a **Hex**, it creates an empty stack. Actions manipulate the top of that stack.

For example, [[Mind's Reflection]] will create an [[Iotas|iota]] representing me, the caster, and add it to the top of the stack. [[Compass' Purification]] will take the iota at the top the stack, if it represents an entity, and transform it into an iota representing that entity's location.

So, drawing those patterns in that order would result in an iota on the stack representing my position.

**Iotas** can represent things like myself or my position, but there are several other types I can manipulate with **Actions**. Here's a comprehensive list:
- Numbers (which some legends called "doubles");
- Vectors, a collection of three numbers representing a position, movement, or direction in the world;
- Booleans or "bools" for short, representing an abstract True or False;

- Entities, like myself, chickens, and minecarts;
- Influences, iotas that represent various abstract ideas such as the absence of data (**Null**) or complete nonsense (**Garbage**);
- Patterns themselves, used for crafting magic items and truly mind-boggling feats like spells that cast other spells;
- A list of several of the above, gathered into a single iota.

Of course, there's no such thing as a free lunch. All spells, and certain other actions, require **[[Media|media]]** as payment.

The best I can figure, a **Hex** is a little bit like a plan of action presented to Nature-- in this analogy, the **media** is used to provide the arguments to back it up, so Nature will accept your plan and carry it out.

That aside, it doesn't seem like anyone has done much research on exactly how much any particular piece of [[Amethyst|amethyst]] is valued. The best I can tell, an [[Amethyst|Amethyst Shard]] is worth about five pieces of [[Amethyst|Amethyst Dust]], and a [[Amethyst|Charged Amethyst Crystal]] is worth about ten.

Strangely enough, it seems like no other form of [[Amethyst|amethyst]] is suitable to be used in the casting of a **Hex**. I suspect that whole blocks or crystals are too solid to be easily unraveled into **media**.

It's also worth noting that each action will consume the **media** it needs immediately, rather than all at once when the Hex finishes. Also, an action will always consume entire items-- an action that only requires one [[Amethyst|Amethyst Dust]]'s worth of **media** will consume an entire [[Amethyst|Charged Amethyst Crystal]], if that's all that's present in my inventory.

Thus, it might be a good idea to bring dust for spellcasting too-- waste not, want not...

I should also be careful to make sure I actually have enough Amethyst in my inventory-- some old texts say that Nature is happy to use one's own mind as payment instead. They describe the feeling as awful but strangely euphoric, "\[...\] an effervescent dissolution into light and energy..." Perhaps that's why all the old practitioners of the art went mad. I can't imagine burning pieces of my mind for power is healthy.

Maybe something's changed, though. In my experiments, I've never managed to do it; if I run out of **media**, the spell will simply fail to cast, as if some barrier is blocking it from harming me.

It would be interesting to get to the bottom of that mystery, but for now I suppose it'll keep me safe.

## Sources

- 📖 **Guidebook:** [[Hex Casting 101]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetCaster` is not a `SpellAction`
