---
aliases: ["Hex Casting 101"]
tags: [hexbook/entry, mod/hex-casting, category/hex-casting, topic/sensing]
mod: "Hex Casting"
category: "Hex Casting"
entry_id: "hexcasting:casting/101"
---

# Hex Casting 101

> **Category:** [[Hex Casting (Category)|Hex Casting]]  
> **Source mod:** [[Hex Casting]]

Casting a **Hex** is quite difficult-- no wonder this art was lost to time! I'll have to re-read my notes carefully.

I can start a **Hex** by pressing **Right-Click** with a [[Staves|Staff]] in my hand-- this will cause a hexagonal grid of dots to appear in front of me. Then I can click and drag from dot to dot to draw patterns in the **[[Media|media]]** of the grid; finishing a pattern will run its corresponding action (more on that later).

Once I've drawn enough patterns to cast a spell, the grid will disappear as the **media** I've stored up is released. Holding **Sneak** while using my [[Staves|staff]] will also clear the grid.

So how do patterns work? In short:
- Patterns will execute...
- Actions, which manipulate...
- [[The Stack]], which is a list of...
- [[Iotas|Iotas]], which are simply units of information.

First, **patterns**. These are essential-- they're what I use to manipulate the **media** around me. Certain patterns, when drawn, will cause **actions** to happen. Actions are what actually do the magic; all patterns influence **media** in particular ways, and when those influences end up doing something useful, we call it an action.

**Media** can be fickle: if I draw an invalid pattern, I'll get some **garbage** result somewhere on my [[The Stack|stack]] (read on...)

## [[An Example|An Example]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,69.9 48.8,45.0 34.5,20.1 20.1,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="34.5,69.9 48.8,45.0 34.5,20.1 20.1,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="69.9" x2="39.1" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="42.4,56.2 41.8,63.5 36.3,60.3" fill="#fc77be"/><circle cx="34.5" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

It's interesting to note that the rotation of a pattern doesn't seem to matter at all. These two patterns both perform an action called [[Mind's Reflection]], for example.

A **Hex** is cast by drawing (valid) actions in sequence. Each action might do one of a few things:
- Gather some information about the environment, leaving it on the top of the stack;
- manipulate the info gathered (e.g. adding two numbers); or
- perform some magical effect, like summoning lightning or an explosion. (These actions are called "spells.")

When I start casting a **Hex**, it creates an empty stack. Actions manipulate the top of that stack.

For example, [[Mind's Reflection]] will create an iota representing me, the caster, and add it to the top of the stack. [[Compass' Purification]] will take the iota at the top the stack, if it represents an entity, and transform it into an iota representing that entity's location.

So, drawing those patterns in that order would result in an iota on the stack representing my position.

**Iotas** can represent things like myself or my position, but there are several other types I can manipulate with **Actions**. Here's a comprehensive list:
- Numbers (which some legends called "doubles");
- Vectors, a collection of three numbers representing a position, movement, or direction in the world;
- Booleans or "bools" for short, representing an abstract True or False;

- Entities, like myself, chickens, and minecarts;
- Influences, iotas that represent various abstract ideas such as the absence of data (**Null**) or complete nonsense (**Garbage**);
- Patterns themselves, used for crafting magic items and truly mind-boggling feats like spells that cast other spells;
- A list of several of the above, gathered into a single iota.

Of course, there's no such thing as a free lunch. All spells, and certain other actions, require **media** as payment.

The best I can figure, a **Hex** is a little bit like a plan of action presented to Nature-- in this analogy, the **media** is used to provide the arguments to back it up, so Nature will accept your plan and carry it out.

That aside, it doesn't seem like anyone has done much research on exactly how much any particular piece of [[Amethyst|amethyst]] is valued. The best I can tell, an [[Amethyst|Amethyst Shard]] is worth about five pieces of [[Amethyst|Amethyst Dust]], and a [[Amethyst|Charged Amethyst Crystal]] is worth about ten.

Strangely enough, it seems like no other form of [[Amethyst|amethyst]] is suitable to be used in the casting of a **Hex**. I suspect that whole blocks or crystals are too solid to be easily unraveled into **media**.

It's also worth noting that each action will consume the **media** it needs immediately, rather than all at once when the Hex finishes. Also, an action will always consume entire items-- an action that only requires one [[Amethyst|Amethyst Dust]]'s worth of **media** will consume an entire [[Amethyst|Charged Amethyst Crystal]], if that's all that's present in my inventory.

Thus, it might be a good idea to bring dust for spellcasting too-- waste not, want not...

I should also be careful to make sure I actually have enough Amethyst in my inventory-- some old texts say that Nature is happy to use one's own mind as payment instead. They describe the feeling as awful but strangely euphoric, "\[...\] an effervescent dissolution into light and energy..." Perhaps that's why all the old practitioners of the art went mad. I can't imagine burning pieces of my mind for power is healthy.

Maybe something's changed, though. In my experiments, I've never managed to do it; if I run out of **media**, the spell will simply fail to cast, as if some barrier is blocking it from harming me.

It would be interesting to get to the bottom of that mystery, but for now I suppose it'll keep me safe.

I have also found an amusing tidbit on why so many practitioners of magic in general seem to go mad, which I may like as some light and flavorful reading not canonical to my world.

Content Warning: some body horror and suggestive elements.

🔗 [Goblin Punch](https://goblinpunch.blogspot.com/2014/05/a-digression-about-wizards.html)

Finally, it seems spells have a maximum range of influence, about 32 blocks from my position. Trying to affect anything outside of that will cause the spell to fail.

Despite this, if I have a player's reference, I can affect them from anywhere. This only applies to affecting them directly, though; I cannot use this to affect the world around them if they're outside of my range.
I ought to be careful when giving out a reference like that. While friendly **Hexcasters** could use them to great effect and utility, I shudder to think of what someone malicious might do with this.

---
### Patterns on this page

- [[An Example]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#casting/101) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
