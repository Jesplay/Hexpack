---
aliases: ["A Primer On Vectors"]
tags: [hexbook/entry, mod/hex-casting, category/hex-casting]
mod: "Hex Casting"
category: "Hex Casting"
entry_id: "hexcasting:casting/vectors"
---

# A Primer On Vectors

> **Category:** [[Hex Casting (Category)|Hex Casting]]  
> **Source mod:** [[Hex Casting]]

It seems I will need to be adroit with vectors if I am to get anywhere in my studies. A vector is a set of three numbers that can be used to represent a variety of things, usually either a position in the world, an offset from one position to another, or a direction in space. The following pages provide a basic summary of how vectors work and what I can do with them.

All the vectors I will be working with as a **Hexcaster** are three-dimensional, since the world I inhabit has three dimensions.

Each vector has an X component, a Y component, and a Z component in that order, corresponding to the three coordinate axes of the world: the X axis runs east and west, the Y axis runs up and down, and the Z axis runs north and south.

The most basic way to use a vector is as a *position*, representing a fixed point in the world - it could be the location of a block, or an entity, or perhaps some empty space that I wish to target with a spell.

A position vector is effectively just a set of coordinates pinpointing the exact location to which it refers. However, not all vectors represent positions.

If I have a vector for the position of a block, and I want to target the block above it, I need to increase the Y component of the vector by 1. I can do this by taking a second vector with the value (0, 1, 0) and adding it to the first one. Likewise, if I wanted to target something two blocks to the east, I could add a second vector with the value (2, 0, 0). Both of these new vectors represent *offsets* from one position to a new position.

Note that while position vectors and offset vectors represent different things, there is no fundamental difference between them - all vectors are just groups of numbers. In fact, I could even consider all my vectors to be one type or the other if I so desired: positions are just offsets from the world origin to the specified coordinates, and offsets are just positions relative to an arbitrary starting point.

Two important properties of vectors are *magnitude* (length) and *direction*. These may be easiest to imagine when thinking of vectors as offsets.

If I were to choose a starting point, and then move in a straight line based on an offset vector, the distance I travelled would be the length of the vector, and the direction I travelled would be its direction.

There many ways to manipulate vectors. As described earlier, I can add them together, which adds the corresponding X, Y, and Z components of each vector. The same applies with subtraction - conveniently, subtracting one position vector from another results in the offset from the first position to the second.

Multiplying and dividing two vectors is not so simple; I can find more information on that topic [here](https://www.mathsisfun.com/algebra/vectors-dot-product.html) and [here](https://www.mathsisfun.com/algebra/vectors-cross-product.html).

I can also multiply or divide a vector by a single number to *scale* it. This multiplies or divides all three components of the vector by that number.

So long as the number is positive, this will multiply or divide the length of the vector without affecting its direction. If the number is negative, the length will be multiplied or divided as usual, but the direction will be reversed.

Dividing a vector by its own length produces a unit vector, which has the same direction as the original but a length of exactly 1. These can be useful if the direction of the vector is all that matters. They are also useful in that I can multiply a unit vector by some number to get a new vector with exactly that length.

Many patterns that calculate a direction of some kind, such as [[Alidade's Purification]], provide it as a unit vector.

All the operations described thus far, and many more, can be performed using patterns found in the [[Mathematics]] section.

I can create new vectors by gathering information from entities using [[Basic Patterns]], by using [[Constants]] to create unit vectors for basic directions, or by directly combining three numbers with [[Vector Exaltation]].

To understand more advanced concepts related to vectors, I may want to do further research outside the bounds of this notebook. The video below may also serve as a useful reference.

🔗 [3blue1brown](https://www.youtube.com/watch?v=fNk_zzaMoSs)


**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#casting/vectors) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
