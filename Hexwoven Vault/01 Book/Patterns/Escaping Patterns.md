---
aliases: ["Escaping Patterns"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/control-flow]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/patterns_as_iotas"
---

# Escaping Patterns

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

One of the many peculiarities of this art is that patterns themselves can act as [[Iotas|iotas]]-- I can even put them onto my [[The Stack|stack]] when casting.

This raises a fairly obvious question: how do I express them? If I simply drew a pattern, it would hardly tell Nature to add it to my stack-- rather, it would simply be matched to an action.

Fortunately, Nature has provided me with a set of special patterns that I can use to work with other patterns directly.

In short, [[Contemplation]] and [[Consideration]] let me add one pattern to the stack, [[Introspection]] and [[Retrospection]] let me add a whole list, and [[Evanition]] lets me undo mistakes during the creation of such a list.

## [[Contemplation|Contemplation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="77.6" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,20.1 48.8,45.0 63.2,69.9 34.5,69.9 20.1,45.0 34.5,20.1 63.2,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="34.5,20.1 48.8,45.0 63.2,69.9 34.5,69.9 20.1,45.0 34.5,20.1 63.2,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="20.1" x2="39.1" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="42.4,33.8 36.3,29.7 41.8,26.5" fill="#fc77be"/><circle cx="34.5" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

To use [[Contemplation]], I draw it, then another arbitrary pattern. That second pattern is added to the stack.

One may find it helpful to think of this as "escaping" the pattern onto the stack, if they happen to be familiar with the science of computers.

The usual use for this is to copy the pattern to a [[Scrolls|Scroll]] or [[Slates|Slate]] using [[Scribe's Gambit]], and then perhaps decorating with them.

## [[Introspection|Introspection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="77.6" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="63.2,20.1 34.5,20.1 20.1,45.0 34.5,69.9 63.2,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="63.2,20.1 34.5,20.1 20.1,45.0 34.5,69.9 63.2,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.2" y1="20.1" x2="54.0" y2="20.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="47.4,20.1 54.0,17.0 54.0,23.3" fill="#fc77be"/><circle cx="63.2" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Drawing [[Introspection]] makes my drawing of patterns act differently, for a time. Until I draw [[Retrospection]], the patterns I draw are saved. Then, when I draw [[Retrospection]], they are added to the stack as a list iota.

## [[Retrospection|Retrospection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="29.3" y2="20.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.9,20.1 29.3,23.3 29.3,17.0" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

If I draw another [[Retrospection|Introspection]], it'll still be saved to the list, but I'll then have to draw two [[Retrospection|Retrospections]] to get back to normal casting.

## [[Evanition|Evanition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 48.8,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,20.1 48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 48.8,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="29.3" y2="20.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.9,20.1 29.3,23.3 29.3,17.0" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Finally, if I make a mistake while drawing patterns inside [[Introspection|Intro-]] and [[Retrospection]], I can draw [[Evanition]] to remove the last pattern that I drew from the pattern list that is being constructed.

## [[Consideration|Consideration]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="77.6" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="63.2,20.1 34.5,20.1 20.1,45.0 34.5,69.9 63.2,69.9 48.8,45.0 34.5,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="63.2,20.1 34.5,20.1 20.1,45.0 34.5,69.9 63.2,69.9 48.8,45.0 34.5,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.2" y1="20.1" x2="54.0" y2="20.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="47.4,20.1 54.0,17.0 54.0,23.3" fill="#fc77be"/><circle cx="63.2" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Similar to [[Contemplation]], but acts even when drawn between [[Introspection|Intro-]] and [[Retrospection]]. This can "escape" other patterns that work between [[Introspection|Intro-]] and [[Retrospection]], putting them into the list without any other effects.

### Further Notes

Because [[Consideration]] acts immediately within [[Introspection|Intro-]] and [[Retrospection]], I need to draw two of them if I want to actually put [[Consideration]] into a pattern list. The first [[Consideration]] escapes the special behavior of the second, allowing it to go into the list.

[[Contemplation]], on the other hand, only needs to be drawn once no matter the depth.

If an iota other than a pattern is present in a list to be executed by [[Hermes' Gambit]] or any other meta-evaluation pattern, it will normally result in a [[Mishaps|mishap]]. However, this can be avoided using the patterns described in this section.

Just as with pattern iotas, the patterns described here can be used to "escape" *any other* kind of iota, causing it to be pushed to the stack when something tries to evaluate it instead of causing a mishap.

This technique may be useful if I want a **Hex** to be able to reference a specific iota, such as a complicated vector or an entity reference, without having to construct or obtain it each time.

To get such an iota into a list of patterns in the first place, I can either use [[Interjection]] while creating the list, or draw a placeholder pattern and then use [[Surgeon's Exaltation]] to replace it once the list has been created.

---
### Patterns on this page

- [[Contemplation]] · Pattern · Escaping
- [[Introspection]] · Pattern · Escaping
- [[Retrospection]] · Pattern · Escaping
- [[Evanition]] · Pattern · Escaping
- [[Consideration]] · Pattern · Escaping

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/patterns_as_iotas) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
