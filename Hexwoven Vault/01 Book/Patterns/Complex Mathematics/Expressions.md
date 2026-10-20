---
aliases: ["Expressions"]
tags: [hexbook/entry, mod/complex-hex, category/complex-mathematics, topic/math]
mod: "Complex Hex"
category: "Complex Mathematics"
entry_id: "hexcasting:patterns/maths/expressions"
---

# Expressions

> **Category:** Patterns › [[Complex Mathematics (Category)|Complex Mathematics]]  
> **Source mod:** [[Complex Hex]]

## [[Symbolic Purification|Symbolic Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,18.5 58.2,41.5 71.5,64.4 45.0,64.4 18.5,64.4 31.8,41.5 45.0,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,18.5 58.2,41.5 71.5,64.4 45.0,64.4 18.5,64.4 31.8,41.5 45.0,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="18.5" x2="49.2" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="52.3,31.1 46.7,27.3 51.8,24.4" fill="#fc77be"/><circle cx="45.0" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`str` → `expression`

Creates a new symbol with the given (single character) label

Many operations that work on numbers can also apply to expressions.
The full list is: Addition, Subtraction, Multiplication, Division, Powers, Absolute, Floor, Ceiling, Sine, Cosine, Tangent, Arcsin, Arccos, Arctan, Sinh, Cosh, Tanh, ArcSinh, ArcCosh, ArcTanh, Vector Pack, Logarithms, Modulo, Minimus, Minimus II, Maximus, Maximus II, And, Or, and Not
In the case of operators that work on booleans, 0 is treated as "false" and any non-zero value is treated as "true".

## [[Substitution Exaltation|Substitution Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,15.8 60.8,35.3 72.0,54.7 49.5,54.7 27.0,54.7 38.3,35.3 49.5,54.7 60.8,74.2 38.3,74.2 15.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="49.5,15.8 60.8,35.3 72.0,54.7 49.5,54.7 27.0,54.7 38.3,35.3 49.5,54.7 60.8,74.2 38.3,74.2 15.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="15.8" x2="53.1" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="55.7,26.5 51.0,23.2 55.3,20.8" fill="#fc77be"/><circle cx="49.5" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`expr, expr, expr | num` → `expr`

Substitutes the third expression/number in place of the second expression within the first.

## [[Equality Purification|Equality Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 45.0,32.0 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,49.7 45.0,67.5 65.5,67.5 55.2,49.7 75.7,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,14.3 45.0,32.0 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,49.7 45.0,67.5 65.5,67.5 55.2,49.7 75.7,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="38.0" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,24.1 36.1,21.1 40.0,18.9" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`expr, expr | num` → `expr`

Creates an expression that equals 1 if the two expressions are equal, and 0 otherwise.

## [[Piecewise Exaltation|Piecewise Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 45.0,32.0 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,49.7 45.0,67.5 55.2,49.7 65.5,32.0 75.7,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,14.3 45.0,32.0 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,49.7 45.0,67.5 55.2,49.7 65.5,32.0 75.7,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="38.0" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,24.1 36.1,21.1 40.0,18.9" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`expr, expr|num, expr|num` → `expr`

Creates a piecewise expression that simplifies to the second argument if the first simplifies to 1.0, otherwise simplifies to the third argument.

## [[Derivation Purification|Derivation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 46.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="23.2,9.6 30.0,21.4 36.8,33.2 23.2,33.2 9.6,33.2 16.4,21.4 23.2,33.2 30.0,45.0 23.2,56.8 16.4,68.6 9.6,56.8 23.2,56.8 36.8,56.8 30.0,68.6 23.2,80.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:191;--dur:3.9s" points="23.2,9.6 30.0,21.4 36.8,33.2 23.2,33.2 9.6,33.2 16.4,21.4 23.2,33.2 30.0,45.0 23.2,56.8 16.4,68.6 9.6,56.8 23.2,56.8 36.8,56.8 30.0,68.6 23.2,80.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.2" y1="9.6" x2="25.4" y2="13.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.9,16.1 24.1,14.1 26.7,12.6" fill="#fc77be"/><circle cx="23.2" cy="9.6" r="2.6" fill="#fc77be"/></svg>

`expr, expr` → `expr`

Takes in an expression and a lone symbol and returns the partial derivative of the expression with respect to the symbol.

## [[Neo's Exaltation|Neo's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="28.3,11.7 36.7,26.1 45.0,40.5 28.3,40.5 11.7,40.5 20.0,26.1 28.3,40.5 36.7,55.0 28.3,69.4 45.0,69.4 61.7,69.4 78.3,69.4 70.0,55.0 78.3,40.5 61.7,40.5 45.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:4.2s" points="28.3,11.7 36.7,26.1 45.0,40.5 28.3,40.5 11.7,40.5 20.0,26.1 28.3,40.5 36.7,55.0 28.3,69.4 45.0,69.4 61.7,69.4 78.3,69.4 70.0,55.0 78.3,40.5 61.7,40.5 45.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.3" y1="11.7" x2="31.0" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.9,19.6 29.4,17.2 32.6,15.4" fill="#fc77be"/><circle cx="28.3" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`num, num, expr` → `matrix`

Takes in a width, height and expression and creates a matrix by substituting i & j-values into the expression corresponding to a position in the matrix and putting the result in the matrix.

## [[Parametric Line|Parametric Line]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,15.8 49.5,35.3 60.8,54.7 38.3,54.7 15.8,54.7 27.0,35.3 38.3,54.7 49.5,74.2 72.0,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:180;--dur:2.2s" points="38.3,15.8 49.5,35.3 60.8,54.7 38.3,54.7 15.8,54.7 27.0,35.3 38.3,54.7 49.5,74.2 72.0,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="15.8" x2="41.9" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,26.5 39.7,23.2 44.0,20.8" fill="#fc77be"/><circle cx="38.3" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`vec, expr` → `entity`

Summons a parametric line at the given position. The expression given will have a t-value (0-1), x/y/z position, and time (w) substituted in and must resolve to a vector on the line.

## [[Parametric Surface|Parametric Surface]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 45.0,32.0 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,49.7 45.0,67.5 65.5,67.5 75.7,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="34.8,14.3 45.0,32.0 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 34.8,49.7 45.0,67.5 65.5,67.5 75.7,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="38.0" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,24.1 36.1,21.1 40.0,18.9" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`vec, expr` → `entity`

Summons a parametric surface at the given position. The expression given will have a u&v-value (0-1), x/y/z position, and time (w) substituted in and must resolve to a vector on the surface

Both Parametric objects can be killed by use of the [[Kill BIT|Kill Bit]] pattern

---
### Patterns on this page

- [[Symbolic Purification]] · Pattern · Purification
- [[Substitution Exaltation]] · Pattern · Exaltation
- [[Equality Purification]] · Pattern · Purification
- [[Piecewise Exaltation]] · Pattern · Exaltation
- [[Derivation Purification]] · Pattern · Purification
- [[Neo's Exaltation]] · Pattern · Exaltation
- [[Parametric Line]] · Pattern · Other
- [[Parametric Surface]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/expressions)
