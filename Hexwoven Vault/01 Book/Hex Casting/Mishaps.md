---
aliases: ["Mishaps"]
tags: [hexbook/entry, mod/hex-casting, category/hex-casting]
mod: "Hex Casting"
category: "Hex Casting"
entry_id: "hexcasting:casting/mishaps"
---

# Mishaps

> **Category:** [[Hex Casting (Category)|Hex Casting]]  
> **Source mod:** [[Hex Casting]]

Unfortunately, I am not (yet) a perfect being. I make mistakes from time to time in my study and casting of **Hexes**; for example, misdrawing a pattern, or trying to an invoke an action with the wrong [[Iotas|iotas]]. And Nature usually doesn't look too kindly on my mistakes-- causing what is called a mishap.

A pattern that causes a mishap will glow red in my grid. Depending on the type of mistake, I can also expect a certain deleterious effect and a spray of red and colorful sparks as the mishandled **[[Media|media]]** curdles into light of a given color.

Fortunately, although the bad effects of mishaps are certainly annoying, none of them are especially destructive in the long term. Nothing better to do than dust myself off and try again ... but I should strive for better anyways.

Following is a list of mishaps I have compiled.

### Invalid Pattern

The pattern drawn is not associated with any action.

Causes yellow sparks, and a **garbage** will be pushed to the top of my [[The Stack|stack]].

### Not Enough Iotas

The action required more iotas than were on the stack.

Causes light gray sparks, and as many **Garbages** as would be required to fill up the argument count will be pushed.

### Too Many Iotas

I tried to cast a spell which exceeded the size limit of the stack.

Causes black sparks, and my entire stack will be replaced with a single **garbage**.

### Incorrect Iota

The action that was executed expected an iota of a certain type for an argument, but it got something invalid. If multiple iotas are invalid, the error message will only tell me about the error deepest in the stack.

Causes dark gray sparks, and the invalid iota will be replaced with **garbage**.

### Vector Out of Ambit

The action tried to affect the world at a point that was out of my range.

Causes magenta sparks, and the items in my hands will be yanked out and flung towards the offending location.

### Entity Out of Ambit

The action tried to affect an entity that was out of my range.

Causes pink sparks, and the items in my hands will be yanked out and flung towards the offending entity.

### Entity is Immune

The action tried to affect an entity that cannot be altered by it.

Causes blue sparks, and the items in my hands will be yanked out and flung towards the offending entity.

### Mathematical Error

The action did something offensive to the laws of mathematics, such as dividing by zero.

Causes red sparks, pushes a **garbage** to my stack, and my mind will be ablated, stealing half the vigor I have remaining. It seems that Nature takes offense to such operations, and divides me in retaliation.

### Incorrect Item

The action requires some sort of item, but the item I supplied was not suitable.

Causes brown sparks. If the offending item was in my hand, it will be flung to the floor. If it was in entity form, it will be flung in the air.

### Incorrect Block

The action requires some sort of block at a target location, but the block supplied was not suitable.

Causes bright green sparks, and causes an ephemeral explosion at the given location. The explosion doesn't seem to harm me, the world, or anything else though; it's just startling.

### Absent Introspection

I attempted to draw a pattern that only works while [[Introspection|introspecting]] without first drawing [[Introspection]].

Causes orange sparks, and pushes the pattern I tried to draw to the stack as an iota.

### Lost in Thought

I attempted to evaluate too many patterns in one **Hex**. Often, this happens because I've accidentally created an infinite loop.

Causes dark blue sparks, and chokes all the air out of me.

### Transgress Other

I attempted to [[Scribe's Gambit|save a reference]] to another player to a permanent medium.

Causes black sparks, and robs me of my sight for approximately one minute.

### Disallowed Action

I tried to cast an action that has been disallowed by a server administrator.

Causes black sparks.

### Catastrophic Failure

A bug in the mod caused an iota of an invalid type or otherwise caused the spell to crash. [Please open a bug report!](https://github.com/gamma-delta/HexMod/issues)

Causes black sparks.


**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#casting/mishaps) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
