---
aliases: ["Meta-Evaluation"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/control-flow]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/meta"
---

# Meta-Evaluation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

## [[Hermes' Gambit|Hermes' Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="58.2,18.5 71.5,41.5 45.0,41.5 31.8,18.5 18.5,41.5 31.8,64.4 58.2,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="58.2,18.5 71.5,41.5 45.0,41.5 31.8,18.5 18.5,41.5 31.8,64.4 58.2,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.2" y1="18.5" x2="62.5" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="65.5,31.1 59.9,27.3 65.0,24.4" fill="#fc77be"/><circle cx="58.2" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`evaluatable` → `many`

Remove something evaluatable (usually a pattern or an [[What is a Hex|entire Hex]]) from the [[The Stack|stack]], then cast it as if I had drawn the relevant patterns with my [[Staves|Staff]].

This can be very powerful in tandem with [[Focus|Foci]], and according to one esoteric scroll I found it also makes the bureaucracy of Nature a "Turing-complete" system.

Note that any [[Iotas|iotas]] other than patterns in a **Hex** cast this way will cause a [[Mishaps|mishap]] unless [[Escaping Patterns|escaped]]. In addition, with the energies of the patterns occurring without me to guide them, *any* mishap will cause the remaining actions in the **Hex** to immediately unravel.

## [[Iris' Gambit|Iris' Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1 39.2,56.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1 39.2,56.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`evaluatable` → `many`

Cast something evaluatable from the stack exactly like [[Hermes' Gambit]], except that a unique "Jump" iota is pushed to the stack beforehand.

When the "Jump"-iota is executed, it'll jump directly to the end of the evaluatable, skipping any remaining actions.

While this may seem redundant given [[Charon's Gambit]] exists, this allows you to exit nested [[Hermes' Gambit|Hermes']] invocations in a controlled way, where Charon only allows you to exit one.

The "Jump" iota will apparently stay on the stack even after execution is finished... better not think about the implications of that.

## [[Thoth's Gambit|Thoth's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 24.5,14.3 34.8,32.0 45.0,14.3 55.2,32.0 65.5,14.3 75.7,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:123;--dur:1.7s" points="14.3,32.0 24.5,14.3 34.8,32.0 45.0,14.3 55.2,32.0 65.5,14.3 75.7,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="17.6" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,22.3 19.5,27.5 15.6,25.2" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`evaluatable, list` → `list`

Remove something evaluatable and a list from the stack, then cast the evaluatable over each element of the list.

More specifically, for each element in the provided list, it will:
- Create a new stack, with everything on the current stack plus that element
- Cast the evaluatable iota
- Save all the iotas remaining on the stack to a list

Then, after all is said and done, pushes the list of saved iotas onto the main stack.

No wonder all the practitioners of this art go mad.

## [[Charon's Gambit|Charon's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,18.5 18.5,41.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 31.8,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="31.8,18.5 18.5,41.5 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 31.8,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="18.5" x2="27.5" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,31.1 25.0,24.4 30.1,27.3" fill="#fc77be"/><circle cx="31.8" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

This pattern forcibly halts a **Hex**. This is mostly useless on its own, as I could simply just stop writing patterns, or put down my [[Staves|staff]].

But when combined with [[Hermes' Gambit|Hermes']] or [[Thoth's Gambit|Thoth's Gambits]], it becomes far more interesting. Those patterns serve to 'contain' that halting, and rather than ending the entire **Hex**, those gambits end instead. This can be used to cause [[Thoth's Gambit]] not to operate on every iota it's given. An escape from the madness, as it were.

## [[Thanatos' Reflection|Thanatos' Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5 45.0,41.5 31.8,18.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5 45.0,41.5 31.8,18.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="22.8" y2="48.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,54.1 20.2,50.2 25.3,47.3" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`—` → `number`

It seems there is a limit to how many patterns can be evaluated within a single **Hex**. I can use this pattern to deterime how many patterns remain before I reach this limit.

---
### Patterns on this page

- [[Hermes' Gambit]] · Pattern · Gambit
- [[Iris' Gambit]] · Pattern · Gambit
- [[Thoth's Gambit]] · Pattern · Gambit
- [[Charon's Gambit]] · Pattern · Gambit
- [[Thanatos' Reflection]] · Pattern · Reflection

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/meta) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
