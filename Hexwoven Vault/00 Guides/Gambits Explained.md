---
tags: [guide, family/gambit]
---

# Gambits Explained

A **Gambit** is a pattern that doesn't have a fixed "takes X, gives Y" shape like Reflections or Distillations. Gambits do one of three things:

1. **Rearrange the stack**: swap, rotate, copy or delete iotas, so the next pattern finds its arguments in the right order.
2. **Control how the hex runs**: run a list of patterns as code, loop over a list, stop early, jump.
3. **Save or load data** outside the stack: in an item, an entity, the ravenmind, an akashic library.

Every other pattern is a straight line: arguments in, result out. Gambits are what let hexes **branch, loop, be stored and reused**. They're the difference between a single spell and a program.

> [!tip] Reading the notation
> Stacks are written bottom → top, so `[0, 1, 2]` has `2` on top. "Pop" = take off the top, "push" = put on the top.

---

## 1. Stack gambits: putting things in order

You'll need these constantly, because patterns always take arguments in a fixed order, and your values rarely arrive in that order.

| Gambit | Effect | When you'd use it |
|---|---|---|
| [[Jester's Gambit]] | `a, b → b, a` | The two things on top are in the wrong order |
| [[Rotation Gambit]] | `a, b, c → b, c, a` | Dig the 3rd iota up to the top |
| [[Rotation Gambit II]] | `a, b, c → c, a, b` | Bury the top iota two places down |
| [[Prospector's Gambit]] | `a, b → a, b, a` | Need the 2nd iota again without losing the top |
| [[Undertaker's Gambit]] | `a, b → b, a, b` | Keep a copy of the top *below* the 2nd iota |
| [[Dioscuri Gambit]] | `a, b → a, b, a, b` | Duplicate a pair (e.g. origin + direction for two raycasts) |
| [[Gemini Gambit]] | `a, n → a × n` | Need many copies of one value |
| [[Fisherman's Gambit]] | `…, n → …` | Pull the iota *n* places down up to the top (negative *n* pushes the top down instead) |
| [[Fisherman's Gambit II]] | same, but copies | Like Fisherman's but leaves the original in place |
| [[Flock's Reflection]] | `… → …, size` | How many iotas are on the stack right now? |

### Bookkeeper's Gambit: delete selectively
An **infinite family** of patterns. You draw it left to right as a row of segments, one segment per stack slot, starting from the deepest affected iota:
- a **flat line** = *keep* that iota
- a **dip (a little "v")** = *remove* that iota

So for a stack `0, 1, 2` (2 on top):
- `—v` (flat, dip) → removes the top: `0, 1`
- `v—` (dip, flat) → removes the 2nd, keeps the top: `0, 2`
- `v` alone is [[Novice's Gambit]]: drop the top iota.

**Useful for:** cleaning up leftovers so a hex ends with exactly what you want, especially inside [[Thoth's Gambit]] where leftovers end up in the results list.

### Swindler's Gambit: any permutation
Takes a **number code** and rearranges the top few iotas into *any* order. The code is the permutation's index ("Lehmer code"). **Useful for:** replacing a long chain of Jester's / Rotation gambits with one pattern.

Codes for the top three iotas, starting from `c, b, a` (a on top):

| Code | Result | Same as |
|---|---|---|
| 0 | `a` unchanged | nothing |
| 1 | `…, a, b` | [[Jester's Gambit]] |
| 2 | `b, c, a` | swap the two below the top |
| 3 | `b, a, c` | [[Rotation Gambit]] |
| 4 | `a, c, b` | [[Rotation Gambit II]] |
| 5 | `a, b, c` | reverse all three |

Full table up to 6 iotas: [Table of Lehmer Codes (HexMod wiki)](https://github.com/gamma-delta/HexMod/wiki/Table-of-Lehmer-Codes-for-Swindler's-Gambit).

---

## 2. Evaluation gambits: hexes as data

These treat a **list of patterns** as code. To get a list of patterns onto the stack without running it, use [[Introspection]] … [[Retrospection]] (or [[Consideration]] for one pattern). See [[Escaping Patterns]].

### Hermes' Gambit: run it
`code → (whatever the code leaves)`. Pops a pattern or list of patterns and runs it **as if you had drawn it**.
- Combined with [[Scribe's Reflection]] it runs a hex saved in a [[Focus]].
- Combined with [[Augur's Exaltation]] it gives you **if/else**: push two code lists and a condition, let Augur's pick one, then Hermes' it.
- A mishap inside stops the rest of that code.

### Thoth's Gambit: for each
`code, list → results`. For **every element** of the list, Thoth's:
1. makes a **copy of the current stack** and pushes the element on top,
2. runs the code,
3. collects *everything left on that stack* into the results list.

> [!warning] The stack copy matters
> Anything already on the stack is visible to every iteration *and ends up in the results too*. Usually you want the stack to hold only the code and the list when you cast Thoth's, or clean up with [[Bookkeeper's Gambit]] inside the loop.

**Example**: double every number: code `[Numerical Reflection 2, Multiplicative Distillation]`, list `[1, 2, 3]` → `[2, 4, 6]`.
**Typical use:** [[Zone Distillation Any]] → list of entities, then Thoth's with a spell, which hits every entity.

### Charon's Gambit: stop
Ends the hex immediately. Inside Hermes' or Thoth's it only ends *that* gambit, so in Thoth's it means "skip the rest of this element". Use it as an early exit after a check.

### Iris' Gambit: run with an escape hatch
Like Hermes', but first pushes a special **Jump** iota. Running that Jump later (e.g. via Hermes') leaps straight to the end of the Iris' code, even from deep inside nested Hermes' calls. Charon's can only exit one level; Iris' Jump exits exactly where you chose. Advanced: think "return from function". (The [[Caduceus]] addon adds a lot more jump manipulation.)

### Thanatos' Reflection
Each hex can only run a limited number of patterns (to stop infinite loops). This tells you how many you have left. **Useful for:** recursive hexes that should stop safely before the limit.

---

## 3. Storage gambits: memory outside the stack

| Gambit | Saves into | Read back with |
|---|---|---|
| [[Scribe's Gambit]] | the item in your other hand ([[Focus]], [[Spellbook]], scroll…) | [[Scribe's Reflection]] |
| [[Chronicler's Gambit]] | an entity (e.g. an item frame or another player's held focus) | [[Chronicler's Purification]] |
| [[Huginn's Gambit]] | the **ravenmind**: one invisible slot that lasts until the hex ends | [[Muninn's Reflection]] |
| [[Akasha's Gambit]] | an [[Akashic Libraries|akashic library]], keyed by a pattern | [[Akasha's Distillation]] |

**The ravenmind** is the hex's own scratch variable. It survives between [[Thoth's Gambit]] iterations, so it's how you **accumulate** something across a loop (a running total, the closest entity so far…).

---

## 4. List gambits
- [[Flock's Gambit]]: `…, n → [last n iotas]`: packs the top *n* iotas into a list.
- [[Flock's Disintegration]]: the reverse, unpacking a list onto the stack.
- [[Speaker's Distillation]] / [[Speaker's Decomposition]]: add / remove an element at the **front** of a list.
- [[Integration Distillation]] / [[Derivation Decomposition]]: the same at the **end** of a list.

---

## Every gambit in the pack
Auto-generated: all patterns whose name contains "Gambit", grouped by topic, with the first sentence of their book description. Click through for the full text, drawing and stack effect.

### Stack Manipulation

- **[[Bookkeeper's Gambit]]** *(Hex Casting)* `many → many`: An infinite family of actions that keep or remove elements at the top of the stack based on the sequence of dips and lines.
- **[[Dioscuri Gambit]]** *(Hex Casting)* `any, any → any, any, any, any`: Copy the top two iotas of the stack.
- **[[Fisherman's Gambit]]** *(Hex Casting)* `number → any`: Grabs the element in the stack indexed by the number and brings it to the top.
- **[[Fisherman's Gambit II]]** *(Hex Casting)* `number → any`: Like Fisherman's Gambit, but instead of moving the iota, copies it.
- **[[Gemini Gambit]]** *(Hex Casting)* `any, number → many`: Removes the number at the top of the stack, then copies the top iota of the stack that number of times.
- **[[Jester's Gambit]]** *(Hex Casting)* `any, any → any, any`: Swaps the top two iotas of the stack.
- **[[Novice's Gambit]]** *(Hex Casting)* `any → —`: Removes the first iota from the stack.
- **[[Prospector's Gambit]]** *(Hex Casting)* `any, any → any, any, any`: Copy the second-to-last iota of the stack to the top.
- **[[Rotation Gambit]]** *(Hex Casting)* `any, any, any → any, any, any`: Yanks the iota third from the top of the stack to the top.
- **[[Rotation Gambit II]]** *(Hex Casting)* `any, any, any → any, any, any`: Yanks the top iota to the third position.
- **[[Swindler's Gambit]]** *(Hex Casting)* `many, number → many`: Rearranges the top elements of the stack based on the given numerical code, which is the index of the permutation wanted.
- **[[Undertaker's Gambit]]** *(Hex Casting)* `any, any → any, any, any`: Copy the top iota of the stack, then put it under the second iota.
- **[[Angler's Gambit]]** *(Hextra Patterns)* `many → many`: A family of patterns that takes the Nth iota from the top of the stack, determined by the length of the head, and duplicates it in place.
- **[[Capsizing Gambit]]** *(Hextra Patterns)* `any → any`: Sends the iota on the top of the stack to the bottom of the stack.
- **[[Capsizing Gambit II]]** *(Hextra Patterns)* `any → any`: Like Capsizing Gambit, but copies the iota instead of moving it.
- **[[Dredging Gambit]]** *(Hextra Patterns)* `any → any`: Pulls the iota at the bottom of the stack to the top of the stack.
- **[[Dredging Gambit II]]** *(Hextra Patterns)* `any → any`: Like Dredging Gambit, but copies the iota instead of moving it.
- **[[Bubbling Gambit]]** *(Overevaluate)* `any, any, any → any, any, any`: Causes the third-from-the-top iota to bubble up over the second-from-the-top iota, essentially swapping them.
- **[[Dioscuri Gambit II]]** *(Overevaluate)* `many, number → many`: Takes a number and duplicates that many elements of the stack while preserving order.
- **[[Geb's Gambit]]** *(Overevaluate)* `many → many`: Yanks the iota n from the top up to the top, determined by tail length.
- **[[Jumbling Gambit]]** *(Overevaluate)* `number, list → jumble`: Creates a jumble iota corresponding to the argument count and the list of numbers.
- **[[Nut's Gambit]]** *(Overevaluate)* `many → many`: Sinks the top iota n from the top, determined by tail length.
- **[[Reflecting Gambit]]** *(Overevaluate)* `any, any, any → any, any, any`: Reflects the order of the top three iota, turning A, B, C into C, B, A.
- **[[Sekhmet's Gambit]]** *(Overevaluate)* `— → varies`: Voids the entire stack except for the top n iota, determined by tail length.

### Lists

- **[[Flock's Gambit]]** *(Hex Casting)* `many, num → list`: Remove num elements from the stack, then add them to a list at the top of the stack.
- **[[Excisor's Gambit]]** *(Hexic)* `[a], int → [a], [a], a`: Extracts an iota from the given list, and breaks the list apart around it.
- **[[Moses's Gambit]]** *(Hexic)* `[a], (a, a → bool) → [[a]]`: Casts the supplied Hex between each pair of elements in the given list, and splits the list at locations that return False.
- **[[Birdkeeper's Gambit]]** *(Hextra Patterns)* `many → many, list`: An infinite family of patterns that creates a list out of iotas on the stack based on a sequence of dips and lines.
- **[[Elimination Gambit]]** *(Hextra Patterns)* `list, any → list, bool`: Remove the iota at the top of the stack, then remove the first instance of it from the list.
- **[[Elimination Gambit II]]** *(Hextra Patterns)* `list, any → list, num`: Remove the iota at the top of the stack, then remove all instances of it from the list.
- **[[Separation Gambit]]** *(Hextra Patterns)* `list, num → list, list`: Splits a list at the given index.
- **[[Viewer's Gambit]]** *(Hextra Patterns)* `list, num → list, any`: Gets the Nth iota of the list without removing it from the list.

### Math & Vectors

- **[[Mitosis Gambit]]** *(MoreIotas)* `mat, num → mat, mat`: Accepts an n by m matrix and an int from 0 to m, and returns two matrices created by splitting the input matrix horizontally at the inputted int.
- **[[Toppling Gambit]]** *(MoreIotas)* `mat, num → mat, mat`: Accepts an n by m matrix and an int from 0 to n, and returns two matrices created by splitting the input matrix vertically at the inputted int.

### Logic & Comparison

- **[[Coalescing Gambit]]** *(Hextra Patterns)* `any, [pattern] | pattern → any`: If the first argument is not Null, returns it.
- **[[Retention Gambit Augur's]]** *(Hextra Patterns)* `A → A, bool`: Convert an argument to a boolean.
- **[[Retention Gambit Equality]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the first argument equals the second (within a small tolerance), return True.
- **[[Retention Gambit Inequality]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the first argument does not equal the second (outside a small tolerance), return True.
- **[[Retention Gambit Length Equality]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the two vectors are the same length (within small tolerance), return True.
- **[[Retention Gambit Length Inequality]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the two vectors are not the same length (outside small tolerance), return True.
- **[[Retention Gambit Maximus]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the first argument is greater than the second, return True.
- **[[Retention Gambit Maximus II]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the first argument is greater than or equal to the second, return True.
- **[[Retention Gambit Minimus]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the first argument is less than the second, return True.
- **[[Retention Gambit Minimus II]]** *(Hextra Patterns)* `A, B → A, B, bool`: If the first argument is less than or equal to the second, return True.

### Control Flow & Meta-evaluation

- **[[Charon's Gambit]]** *(Hex Casting)* `— → —`: This pattern forcibly halts a Hex.
- **[[Hermes' Gambit]]** *(Hex Casting)* `evaluatable → many`: Remove something evaluatable (usually a pattern or an entire Hex) from the stack, then cast it as if I had drawn the relevant patterns with my Staff.
- **[[Iris' Gambit]]** *(Hex Casting)* `evaluatable → many`: Cast something evaluatable from the stack exactly like Hermes' Gambit, except that a unique "Jump" iota is pushed to the stack beforehand.
- **[[Thoth's Gambit]]** *(Hex Casting)* `evaluatable, list → list`: Remove something evaluatable and a list from the stack, then cast the evaluatable over each element of the list.
- **[[Arke's Gambit]]** *(Caduceus)* `[pattern] | pattern → many`: Cast a pattern or list of patterns from the stack similarly to Iris' Gambit.
- **[[Arke's Gambit II]]** *(Caduceus)* `[pattern] | pattern, any → many`: Like Arke's Gambit, but only patterns within an enclosing Thetis' Gambit II with a jump tag matching the given iota are captured.
- **[[Elbrus' Gambit]]** *(Caduceus)* `[pattern] → many`: Cast a list of patterns from the stack exactly like Hermes' Gambit, except that a Prometheus' Gambit within this cast will jump to an index within this list of patterns.
- **[[Ewer's Gambit]]** *(Caduceus)* `any → —`: Remove the top iota from the stack, and write it to the jump tag of the current evaluation.
- **[[Prometheus' Gambit]]** *(Caduceus)* `number → many`: Remove a number from the stack, then jump to that index in the list cast by the enclosing Elbrus' Gambit.
- **[[Thetis' Gambit]]** *(Caduceus)* `[pattern] | pattern → many`: Cast a pattern or list of patterns from the stack exactly like Hermes' Gambit, except that anything outside of this cast will not be captured by an Arke's Gambit within it.
- **[[Thetis' Gambit II]]** *(Caduceus)* `[pattern] | pattern, any → many`: Like Thetis' Gambit, but also sets the jump tag of the cast to the given iota.
- **[[Seshat's Gambit]]** *(Ephemera)* `— → any | null`: Pops an iota from the accumulator and pushes it to the current stack.
- **[[Lani's Gambit]]** *(Hexic)* `[… → …] → —`: Casts the given Hex as if from my staff, using its stack, etc.
- **[[Thoth's Pseudogambit]]** *(Hexic)* `— → —`: 
- **[[Acheron's Gambit]]** *(Hextra Patterns)* `any → —`: Forcibly halts a Hex like Charon's Gambit if the iota at the top of the stack is considered True by Nature.
- **[[Anubis' Gambit]]** *(Hextra Patterns)* `many, [pattern], list → many`: Casts like Thoth's Gambit, however each sub-execution operates on the main stack instead of copies of the stack.
- **[[Djehuty's Gambit]]** *(Hextra Patterns)* `[pattern], list → list`: Casts like Thoth's Gambit, however the new stack for each element additionally has the index of that element added to the top.
- **[[Helios' Gambit]]** *(Hextra Patterns)* `any, ([pattern] | pattern) → many`: Casts a pattern or list of patterns like Hermes' Gambit if the next iota from the top of the stack is considered True by Nature.
- **[[Ra's Gambit]]** *(Hextra Patterns)* `many, [pattern], list → many`: Casts like Djehuty's Gambit, however each sub-execution operates on the main stack instead of copies of the stack.
- **[[Selene's Gambit]]** *(Hextra Patterns)* `any, ([pattern] | pattern) → many`: Casts a pattern or list of patterns like Hermes' Gambit if the next iota from the top of the stack is considered False by Nature.
- **[[Styx's Gambit]]** *(Hextra Patterns)* `any → —`: Forcibly halts a Hex like Charon's Gambit if the iota at the top of the stack is considered False by Nature.
- **[[Erebus' Gambit]]** *(Lapisworks)* `positive num → —`: Charon's Gambit, if it could break through more than one layer.
- **[[Judge's Gambit]]** *(Lapisworks)* `[pattern] → —`: Pops a list of iotas and sets the instruction list of the current Thoth's-like to that list.
- **[[Kitkat's Gambit]]** *(Lapisworks)* `num, [pattern] → —`: Do you know the definition of insanity?
- **[[Mintiest Gambit]]** *(Lapisworks)* `num, num, [pattern] → [any]`: A oney and a twoey and a you-know-what-to-doey!
- **[[Queue Gambit]]** *(Lapisworks)* `[any] → —`: Pops a list of iotas and sets the datum left to execute by the current Thoth's-like to that list.
- **[[Apep's Gambit]]** *(Overevaluate)* `pattern/list, list → any`: Folds across the second list using the Hex or pattern in the first argument.
- **[[Atalanta's Gambit]]** *(Overevaluate)* `— → —`: Should only be run inside Thoth's or Sisyphus' Gambit.
- **[[Athena's Gambit]]** *(Overevaluate)* `list of patterns → boolean`: Casts a list of patterns similar to Hermes' Gambit but allows for handling of mishaps.
- **[[Castor's Gambit]]** *(Overevaluate)* `any → —`: Should only be run within a Thoth's.
- **[[Janus' Gambit]]** *(Overevaluate)* `— → —`: Instantly terminates the Hex regardless of current circumstances.
- **[[Ma'at's Gambit]]** *(Overevaluate)* `boolean, any → —`: Takes a boolean and any iota.
- **[[Nephthys' Gambit]]** *(Overevaluate)* `pattern/list → varies`: Pops a pattern or pattern list and dives down an amount corresponding to the tail length to cast the pattern list.
- **[[Pollux's Gambit]]** *(Overevaluate)* `any → —`: Sibling pattern to Castor's Gambit, this pattern schedules the additional iteration for the end instead.
- **[[Sisyphus' Gambit]]** *(Overevaluate)* `list → —`: Takes a pattern list and casts it over and over forever until it encounters either a Charon's Gambit or a Janus' Gambit.
- **[[Themis' Gambit]]** *(Overevaluate)* `pattern/list, list → list`: Takes a pattern or pattern list and casts over every element the second list similar to Thoth's Gambit in order to sort it.
- **[[Tutu's Gambit]]** *(Overevaluate)* `— → —`: Does nothing.

### Reading & Writing

- **[[Akasha's Gambit]]** *(Hex Casting)* `vector, pattern, any → —`: Associate the iota with the given pattern in the Akashic Library with its Record at the given position.
- **[[Chronicler's Gambit]]** *(Hex Casting)* `entity, any → —`: Like Scribe's Gambit, but the iota is written to an entity instead of my other hand.
- **[[Huginn's Gambit]]** *(Hex Casting)* `any → —`: Removes the top iota from the stack, and saves it to my ravenmind, storing it there until I stop casting the Hex.
- **[[Scribe's Gambit]]** *(Hex Casting)* `any → —`: Remove the top iota from the stack, and save it into the item in my other hand.
- **[[Resonance Gambit]]** *(HexIntent)* `memory, any → none`: Stores the top of the stack in a held or embedded Memory Crystal's memory.
- **[[Brigh's Gambit]]** *(Hexal)* `pattern → —`: Takes a pattern and marks that entry of your Everbook as a Pattern that when drawn by your hand will be treated as any other of Nature's patterns.
- **[[Elysium's Gambit]]** *(Hexal)* `vector, pattern → —`: Takes the position of an Akashic Record, as well as a Pattern as a key, and stores the iota in that record at that key to your Everbook at the same key.
- **[[Lethe's Gambit]]** *(Hexal)* `pattern → —`: Takes a pattern and removes the entry with that pattern key from your Everbook.
- **[[Mnemosyne's Gambit]]** *(Hexal)* `vector, pattern → —`: Takes the position of an Akashic Record, as well as a Pattern as a key, and retrieves the iota at that key from your Everbook, storing it in the given Akashic Record if able.
- **[[Grok Gambit]]** *(Hexical)* `list, list → —`: Takes two lists of iotas and sets them as my stack and my parenthesized list respectively.
- **[[Librarian's Gambit]]** *(Hexical)* `vector, pattern, any → —`: Writes an iota under a pattern key to an Akashic Bookshelf.
- **[[Librarian's Gambit II]]** *(Hexical)* `vector → —`: Clears an Akashic Bookshelf.
- **[[Tachygrapher's Gambit]]** *(IoticBlocks)* `entity | vec, num, any → —`: Read a list iota out of an entity or block, then set the element at the specified index of that list to the given iota, then write the resulting list back to the entity or block.
- **[[Amanuensis' Gambit]]** *(Lapisworks)* `any, num → —`: Specific hand version of Scribe's Gambit.
- **[[Soroban Gambit]]** *(Overevaluate)* `— → —`: Resets the soroban to 0.
- **[[Soroban Gambit II]]** *(Overevaluate)* `number → —`: Pops a number from the stack and sets the soroban to it.

### Items & Crafting

- **[[Decorator's Gambit]]** *(Ephemera)* `item frame, number → —`: Accepts an item frame entity and an integer from 0 to 7, and sets its rotation to the number.
- **[[Depot Gambit]]** *(Hexal)* `mote, vec → mote`: Move the record referenced by the passed mote to the Mote Nexus at the passed position.
- **[[Splitting Gambit]]** *(Hexal)* `mote, int → mote, mote`: Splits the specified number of items off of the passed mote, creating a new mote pointing to a new mote record.
- **[[Jeweller's Gambit]]** *(Lapisworks)* `any → —`: Writes the iota to the necklace I'm wearing.

### Wisps & Links

- **[[Pathfinder's Gambit]]** *(Hexal)* `vec → —`: Removes a position vector from the stack; The Cyclic Wisp that executed this pattern will move in a straight line towards that position (colliding with terrain in its path) until it reaches it.
- **[[Unclogging Gambit]]** *(Hexal)* `— → —`: Removes all unread iota from the caster's received iota queue.

### Text, Strings & Chat

- **[[Postman's Gambit]]** *(Ephemera)* `Player, any → —`: Accepts a player and any iota less than 256 characters long, and shows that iota to them as if they had cast Reveal, also telling them who sent the message.
- **[[Postman's Gambit II]]** *(Ephemera)* `— → —`: Resets the cost of sending me a message via Postman's Gambit back to 1/10 of a dust.
- **[[Censor's Gambit]]** *(Hexpose)* `text, boolean → text`: Takes a text iota and sets whether it is obfuscated.
- **[[Editor's Gambit]]** *(Hexpose)* `text, boolean → text`: Takes a text iota and sets whether it has been struck through.
- **[[Gothic Gambit]]** *(Hexpose)* `text, boolean → text`: Takes a text iota and sets whether it is bold.
- **[[Lumière Gambit]]** *(Hexpose)* `text, vector → text`: Takes a text iota and a color encoded as a vector whose components range from 0 to 1, and produces a text iota of that color.
- **[[Manutius' Gambit]]** *(Hexpose)* `text, boolean → text`: Takes a text iota and sets whether it is in italics.
- **[[Notetaker's Gambit]]** *(Hexpose)* `text, boolean → text`: Takes a text iota and sets whether it is underlined.
- **[[Sifter's Gambit]]** *(MoreIotas)* `str | null → —`: Accepts a string; All future chat messages starting with that string won't be seen by others, and only messages prefixed with that string can be read by Whisper Reflection.

### Sets, Maps & Special Iotas

- **[[Schrödinger's Gambit]]** *(Hexcellular)* `property, any → —`: Sets the property iota.
- **[[Bucketing Gambit]]** *(Hextra Patterns)* `many, num → set`: Remove num iota from the top of the stack, then add them to a set at the top of the stack.
- **[[Insertion Gambit]]** *(Hextra Patterns)* `set, (num|vec|entity) → set, bool`: Remove the iota at the top of the stack, then add it to the set.
- **[[Removal Gambit]]** *(Hextra Patterns)* `set, (num|vec|entity) → set, bool`: Remove the iota from top of the stack, then remove it set.

### Dimensions & Worlds

- **[[Heimdall's Gambit]]** *(Oneironaut)* `Imprint, executable → many`: A strange pattern that appears to be a variant of Hermes' Gambit.

### Casting Devices

- **[[Saturn's Gambit]]** *(HexIntent)* `pattern → none`: Like Hermes' Gambit but executes it asynchronously, splitting up large lists of work over multiple ticks that execute synchronously.
- **[[Archgenie Gambit]]** *(Hexical)* `any → —`: Can be cast by a casting device, Staff, or within the Archgenie Lamp to remember an iota.
- **[[Charmed Gambit]]** *(Hexical)* `any → —`: Writes an iota to the charmed item’s internal storage.
- **[[Charmed Gambit II]]** *(Hexical)* `any → —`: If the item charged features an iota storage by default, such as a Focus, writes an iota into it.
- **[[Genie Gambit]]** *(Hexical)* `any → —`: Asks the genie to remember an iota for me.
- **[[Metronome Gambit]]** *(Oneironaut)* `number → —`: Accepts a positive integer.
- **[[Williams' Gambit]]** *(Oneironaut)* `non-list → —`: Accepts a non-list iota, and stores it inside the current cast loop.
- **[[Williams' Gambit II]]** *(Oneironaut)* `non-list → —`: As Williams' Gambit, but can be cast from non-rod sources while a rod is in use.

### Minds & Hierophantics

- **[[Payload Gambit]]** *(Hierophantics)* `mind, [pattern] | null → —`: Stores the provided Hex into the mind.
- **[[Trigger Gambit]]** *(Hierophantics)* `mind, trigger | null → —`: Stores the provided trigger into the mind.
- **[[Amalgamation Gambit]]** *(Lapisworks)* `num → —`: Removes the amalgamation on my person with that index.

### Hex Wright Magic

- **[[Cantor's Gambit]]** *(Hexwright)* `slot num, any → —`: Huginn's Gambit, but for one of your additional minds.
- **[[Chorister's Gambit]]** *(Hexwright)* `resonance or null, number, any → —`: This is Publishing.
- **[[Chorister's Gambit (Hexwright)]]** *(Hexwright)* `resonance or null, number, any → —`: This is Setting State.
- **[[Emitter's Gambit]]** *(Hexwright)* `vector, number, [pattern] | null → —`: Tunes the Emitter at vec to num channel with pattern to cast.
- **[[Engraver's Gambit]]** *(Hexwright)* `any → —`: Scribe's Gambit for staffs and weapons.
- **[[Manifold's Gambit]]** *(Hexwright)* `item entity, num → —`: Put the item entity in the Manifold Vault at the slot given.
- **[[Ring Bearer's Gambit]]** *(Hexwright)* `number, [pattern] → —`: Sets a ring in the off-hand to listen to a harmonic channel and execute the pattern when data is received.
- **[[Talisman's Gambit]]** *(Hexwright)* `num, num → —`: Sets the conditions of the Talisman.
- **[[Teal's Gambit]]** *(Hexwright)* `[vec, vec], [vec, vec], num → —`: Takes two sets of vectors forming two planes, folding them to occupy the same space time for the
- **[[Tuner's Gambit]]** *(Hexwright)* `vector, number | null → —`: Tells the block at that position which harmonic channel to transmit on.
- **[[Vault's Gambit]]** *(Hexwright)* `— → —`: Toggles the held key's Vault between Anyone and Allowed only.
- **[[Vault's Relocation Gambit]]** *(Hexwright)* `vec → —`: Can only be cast from inside a Masterwork vault.
- **[[Warden's Gambit]]** *(Hexwright)* `[vec,vec] → —`: Sets the size of the wards cuboid.
- **[[Watcher's Gambit]]** *(Hexwright)* `vec, entity/item type → —`: Sets the Warding Box trigger to a specific entitytype or itemtype.
- **[[Wielder's Gambit]]** *(Hexwright)* `num → —`: Affects staffs with specific cores and has a different effect depending on the core.

### Debugging

- **[[Compositor's Gambit]]** *(HexDebug)* `vec, any → —`: Remove a vector and iota from the stack, then save the iota into the secondary item in the Splicing Table at that position.
- **[[Projectionist's Gambit]]** *(HexDebug)* `vec, num → —`: Remove a vector and number from the stack, then flip the Spellbook in the main slot of the Splicing Table or Focal Frame at that position to the given page number.
- **[[Projectionist's Gambit II]]** *(HexDebug)* `vec, num → —`: Like Projectionist's Gambit, but the page number of the Spellbook in the secondary slot is written instead of the main slot.
- **[[Splicer's Gambit]]** *(HexDebug)* `vec, num | null, num | null → —`: Remove a vector and two values from the stack, then set the selection of the Splicing Table at that position as previously described.
- **[[Sprocket's Gambit]]** *(HexDebug)* `vec, num → —`: Remove a vector and number from the stack, then move the view of the Splicing Table at that position so that the given index is the leftmost one visible.

### Miscellaneous

- **[[Vulpine Gambit]]** *(Hexic)* `— → —`: 
- **[[Patter's Gambit]]** *(Hexpats)* `entity → —`: This pattern appears to conjure a projection of the casters hand about the head of some entity.
