---
aliases: ["Mathematics"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/math]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/math"
---

# Mathematics

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

Many mathematical operations function on both numbers and vectors. Such arguments are written as "num|vec".

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`num|vec, num|vec` → `num|vec`

Perform addition.

As such:
- With two numbers at the top of the [[The Stack|stack]], combines them into their sum.
- With a number and a vector, removes the number from the stack and adds it to each element of the vector.
- With two vectors, combines them by summing corresponding components into a new vector (i.e. (1, 2, 3) + (0, 4, -1) = (1, 6, 2)).

## [[Subtractive Distillation|Subtractive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="69.9" x2="44.3" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.9,56.2 47.0,60.3 41.5,63.5" fill="#fc77be"/><circle cx="48.8" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`num|vec, num|vec` → `num|vec`

Perform subtraction.

As such:
- With two numbers at the top of the stack, combines them into their difference.
- With a number and a vector, removes the number from the stack and subtracts it from each element of the vector.
- With two vectors, combines them by subtracting each component.

In all cases, the top of the stack or its components are subtracted from the second-from-the-top.

## [[Multiplicative Distillation|Multiplicative Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="24.7" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,33.8 22.0,29.7 27.4,26.5" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`num|vec, num|vec` → `num|vec`

Perform multiplication or the dot product.

As such:
- With two numbers, combines them into their product.
- With a number and a vector, removes the number from the stack and multiplies each component of the vector by that number.
- With two vectors, combines them into their [dot product](https://www.mathsisfun.com/algebra/vectors-dot-product.html).

## [[Division Distillation|Division Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`num|vec, num|vec` → `num|vec`

Perform division or the cross product.

As such:
- With two numbers, combines them into their quotient.
- With a number and a vector, removes the number and divides each element of the vector by it.
- With two vectors, combines them into their [cross product](https://www.mathsisfun.com/algebra/vectors-cross-product.html).

In the first and second cases, the second-from-the-top of the stack is divided by the top of the stack.

WARNING: Never divide by zero!

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`num|vec` → `number`

Compute the absolute value or length.

Replaces a number with its absolute value, or a vector with its length.

## [[Power Distillation|Power Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,74.2 27.0,54.7 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="38.3,74.2 27.0,54.7 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="74.2" x2="34.7" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,63.5 36.8,66.8 32.5,69.2" fill="#fc77be"/><circle cx="38.3" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`num|vec, num|vec` → `num|vec`

Perform exponentiation or vector projection.

With two numbers, combines them by raising the first to the power of the second.
- With a number and a vector, removes the number and raises each component of the vector to the number's power.
- With two vectors, combines them into the [vector projection](https://en.wikipedia.org/wiki/Vector_projection) of the second-from-the-top onto the top of the stack.

In the first and second cases, the first argument or its components are the base, and the second argument or its components are the exponent.

## [[Floor Purification|Floor Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 75.7,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:82;--dur:1.6s" points="14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7 75.7,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="14.3" x2="20.9" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,14.3 20.9,16.6 20.9,12.1" fill="#fc77be"/><circle cx="14.3" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`num|vec` → `num|vec`

"Floors" a number, cutting off the fractional component and leaving an integer value. If passed a vector, instead floors each of its components.

## [[Ceiling Purification|Ceiling Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,49.7 34.8,49.7 45.0,32.0 55.2,14.3 75.7,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:82;--dur:1.6s" points="14.3,49.7 34.8,49.7 45.0,32.0 55.2,14.3 75.7,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="49.7" x2="20.9" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,49.7 20.9,52.0 20.9,47.5" fill="#fc77be"/><circle cx="14.3" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`num|vec` → `num|vec`

"Ceilings" a number, raising it to the next integer value if it has a fractional component. If passed a vector, instead ceils each of its components.

## [[Vector Exaltation|Vector Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:143;--dur:2.0s" points="14.3,32.0 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="20.9" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,32.0 20.9,34.3 20.9,29.8" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`num, num, num` → `vector`

Combine three numbers at the top of the stack into a vector's X, Y, and Z components (bottom to top).

## [[Vector Disintegration|Vector Disintegration]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:143;--dur:2.0s" points="14.3,32.0 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="20.9" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,32.0 20.9,34.3 20.9,29.8" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vector` → `num, num, num`

Split a vector into its X, Y, and Z components (bottom to top).

## [[Modulus Distillation|Modulus Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5 58.2,64.4 71.5,41.5 45.0,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="31.8,64.4 45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5 58.2,64.4 71.5,41.5 45.0,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="36.0" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.0,51.8 38.5,58.5 33.5,55.6" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`num|vec, num|vec` → `num|vec`

Takes the modulus of two numbers. This is the amount remaining after division - for example, 5 %% 2 is 1, and 5 %% 3 is 2. When applied on vectors, performs the above operation elementwise.

## [[Axial Purification|Axial Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="75.7" y1="32.0" x2="72.4" y2="26.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="70.1,22.3 74.4,25.2 70.5,27.5" fill="#fc77be"/><circle cx="75.7" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vec|num` → `vec|num`

For a vector, coerce it to its nearest axial direction, a unit vector. For a number, return the sign of the number; 1 if positive, -1 if negative. In both cases, zero is unaffected.

## [[Entropy Reflection|Entropy Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="60.8,74.2 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:113;--dur:1.6s" points="60.8,74.2 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.8" y1="74.2" x2="57.2" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.6,63.5 59.3,66.8 55.0,69.2" fill="#fc77be"/><circle cx="60.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Creates a random number between 0 and 1.

---
### Patterns on this page

- [[Additive Distillation]] · Pattern · Distillation
- [[Subtractive Distillation]] · Pattern · Distillation
- [[Multiplicative Distillation]] · Pattern · Distillation
- [[Division Distillation]] · Pattern · Distillation
- [[Length Purification]] · Pattern · Purification
- [[Power Distillation]] · Pattern · Distillation
- [[Floor Purification]] · Pattern · Purification
- [[Ceiling Purification]] · Pattern · Purification
- [[Vector Exaltation]] · Pattern · Exaltation
- [[Vector Disintegration]] · Pattern · Disintegration
- [[Modulus Distillation]] · Pattern · Distillation
- [[Axial Purification]] · Pattern · Purification
- [[Entropy Reflection]] · Pattern · Reflection

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
