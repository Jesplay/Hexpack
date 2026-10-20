---
aliases: ["Talismans", "Talisman", "Talisman of Reprieve"]
tags: [hexbook/entry, mod/hexwright, category/casting-tools, topic/hexwright]
mod: "Hexwright"
category: "Casting Tools"
entry_id: "hexcasting:hex_wright/implements/talisman"
---

# Talismans

> **Category:** Hex Wright › [[Casting Tools (Category)|Casting Tools]]  
> **Source mod:** [[Hexwright]]

### Talisman

*Item:* **Talisman**

Talisman are small pieces of parchment that can be worn and can carry a hex. The contained hex will cast when the given conditions are met.

Crafted blank, they can have a hex written to them using [[Scribe's Gambit]]. Costs 5 dust.

Talisman cast as the holder, with the holders ambit, using their [[Media|media]].
[[Mishaps|Mishaps]] also occur for the holder. They are only active when worn in the appropriate Talisman slot. You have 3 Talisman slots.

### Visual Identifier

The final part of a creating a Talisman is making it distinct so it's easy to tell apart.
Sneak + right-click will let you draw your own Talisman identifier, but only when it's blank.

You have 16 colours to choose from when drawing.

## [[Talisman's Gambit|Talisman's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,35.3 38.3,15.8 27.0,35.3 38.3,54.7 27.0,74.2 15.8,54.7 27.0,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="49.5,35.3 38.3,15.8 27.0,35.3 38.3,54.7 27.0,74.2 15.8,54.7 27.0,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="35.3" x2="45.9" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="43.3,24.5 48.1,27.8 43.8,30.3" fill="#fc77be"/><circle cx="49.5" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`num, num` → `—`

Sets the conditions of the Talisman. First number is the trigger, second is the context.
Costs 5 Dust

### Triggers

The trigger is when the hex casts.
- 0 Struck:you are hit
- 1 On Kill:you land a killing blow
- 2 Mishap: staffcast of yours errors
- 3 Use:you right-click it
- 4 Hurt:you deal damage to something
- 5 Wound: you take damage from a hit. If a hit doesn't do damage due to armour, wound is not triggered.

### Triggers

- 6 Death: you die
- 7 Break:an item of yours wears out
- 8 Mine:you break a block
- 9 Traverse:you cross into another dimension
- 10 Interact: you right-click a block or a creature
- 11 Command: you type /xts &lt;words&gt;. Hex starts with the words as a string.

### Contexts

Context is the [[Iotas|iota]] at the start of the [[The Stack|stack]].
- 0 Self:your entity
- 1 Other:entity relating to trigger
- 2 Magnitude:number related to event
- 3 Gaze:what you see (max 16 blocks). Entity or Vector.
- 4 Position:your position or events position

### Pairing Them

All combos of trigger and context work. Struck with Other seeds the hex with who hurt you; Hurt with Other gives it whom you hurt.
Other is your attacker under Struck and Wound, and is your target under Hurt and On Kill. It's also your killer under Death.

### Magnitude

- Struck: the damage thrown
- Wound: hp lost
- Hurt: the damage you dealt
- On Kill: Killed entities max health
- Death: last damage you took
- Mine: blocks hardness level

### Null

Some triggers can generate Null. For example, Mishap, Use, Break and Traverse  all return Null if the context is Magnitude. Struck can also return Null if the damage source is a fall, or fire, for example.

### Talisman of Reprieve

*Item:* **Talisman of Reprieve**

This Talisman was the pinnacle of Hex Wright skill. It casts on the brink of death, giving you two seconds of total immortality, before casting the inscribed Hex. The cost is a 5 minute cooldown.

---
### Patterns on this page

- [[Talisman's Gambit]] · Spell · Gambit

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
