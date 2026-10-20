---
aliases: ["Quaternionics"]
tags: [hexbook/entry, mod/complex-hex, category/complex-mathematics, topic/math]
mod: "Complex Hex"
category: "Complex Mathematics"
entry_id: "hexcasting:patterns/maths/quaternionics"
---

# Quaternionics

> **Category:** Patterns › [[Complex Mathematics (Category)|Complex Mathematics]]  
> **Source mod:** [[Complex Hex]]

## [[Quaternionic Exal|Quaternionic Exal.]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 38.3,35.3 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="27.0,15.8 38.3,35.3 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="30.6" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.2,26.5 28.5,23.2 32.8,20.8" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`num, vec` → `quaternion`

Pushes a Quaternion with num as the real coefficient and the vector's x, y, & z as the coefficients of i, j & k respectively.

## [[Quaternionic Dntg|Quaternionic Dntg.]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 38.3,35.3 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="27.0,15.8 38.3,35.3 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="30.6" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.2,26.5 28.5,23.2 32.8,20.8" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `num, vec`

Pushes the real coefficient and a vector where the x, y, & z components are the coefficients of i, j, & k respectively.

## [[Quaternionic Rfln. 1|Quaternionic Rfln.: 1]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 38.3,35.3 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2 72.0,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="27.0,15.8 38.3,35.3 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2 72.0,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="30.6" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.2,26.5 28.5,23.2 32.8,20.8" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`—` → `quaternion`

Pushes 1 + 0i + 0j + 0k to the [[The Stack|stack]].

## [[Quaternionic Rfln. i|Quaternionic Rfln.: i]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="24.5,14.3 34.8,32.0 45.0,49.7 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,67.5 75.7,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="24.5,14.3 34.8,32.0 45.0,49.7 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,67.5 75.7,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="14.3" x2="27.8" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,24.1 25.9,21.1 29.8,18.9" fill="#fc77be"/><circle cx="24.5" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`—` → `quaternion`

Pushes 0 + 1i + 0j + 0k to the stack.

## [[Quaternionic Rfln j|Quaternionic Rfln: j]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,13.0 31.5,29.0 40.7,45.0 50.0,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 50.0,61.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.8s" points="22.2,13.0 31.5,29.0 40.7,45.0 50.0,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 50.0,61.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="13.0" x2="25.2" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.3,21.8 23.4,19.1 26.9,17.1" fill="#fc77be"/><circle cx="22.2" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`—` → `quaternion`

Pushes 0 + 0i + 1j + 0k to the stack.

## [[Quaternionic Rfln k|Quaternionic Rfln: k]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 38.3,35.3 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="27.0,15.8 38.3,35.3 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="30.6" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.2,26.5 28.5,23.2 32.8,20.8" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`—` → `quaternion`

Pushes 0 + 0i + 0j + 1k to the stack.

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`qtrn, qtrn` → `quaternion`

Performs Addition

## [[Subtractive Distillation|Subtractive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="48.8,69.9 34.5,45.0 20.1,20.1 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="69.9" x2="44.3" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.9,56.2 47.0,60.3 41.5,63.5" fill="#fc77be"/><circle cx="48.8" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`qrtn, qtrn` → `quaternion`

Performs Subtraction

## [[Multiplicative Distillation|Multiplicative Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="24.7" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,33.8 22.0,29.7 27.4,26.5" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`quaternion, num` → `quaternion`

Performs Multiplication, Scalar if a num & quat are given, Hamiltonian if 2 quaternions are given.

## [[Division Distillation|Division Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`quaternion, num` → `quaternion`

Performs Scalar Division.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `num`

Pushes the Argument (the length).

## [[Quaterionic Prfn. 1|Quaterionic Prfn.: 1]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 38.3,35.3 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2 72.0,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="27.0,15.8 38.3,35.3 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2 72.0,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="30.6" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.2,26.5 28.5,23.2 32.8,20.8" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `num`

Pushes the coefficient of 1.

## [[Quaterionic Prfn. i|Quaterionic Prfn.: i]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="24.5,14.3 34.8,32.0 45.0,49.7 24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 55.2,67.5 75.7,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="24.5,14.3 34.8,32.0 45.0,49.7 24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 55.2,67.5 75.7,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="14.3" x2="27.8" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,24.1 25.9,21.1 29.8,18.9" fill="#fc77be"/><circle cx="24.5" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `num`

Pushes the coefficient of i.

## [[Quaterionic Prfn. j|Quaterionic Prfn.: j]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,13.0 31.5,29.0 40.7,45.0 22.2,45.0 13.0,29.0 22.2,13.0 40.7,13.0 50.0,29.0 40.7,45.0 50.0,61.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.8s" points="22.2,13.0 31.5,29.0 40.7,45.0 22.2,45.0 13.0,29.0 22.2,13.0 40.7,13.0 50.0,29.0 40.7,45.0 50.0,61.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="13.0" x2="25.2" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.3,21.8 23.4,19.1 26.9,17.1" fill="#fc77be"/><circle cx="22.2" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `num`

Pushes the coefficient of j.

## [[Quaterionic Prfn k|Quaterionic Prfn: k]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 38.3,35.3 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="27.0,15.8 38.3,35.3 49.5,54.7 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 60.8,35.3 49.5,54.7 60.8,74.2 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="30.6" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.2,26.5 28.5,23.2 32.8,20.8" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `num`

Pushes the coefficient of k.

## [[Quaternionic Inverse|Quaternionic Inverse]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,13.0 31.5,29.0 40.7,45.0 50.0,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 50.0,61.0 59.2,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.8s" points="22.2,13.0 31.5,29.0 40.7,45.0 50.0,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 50.0,61.0 59.2,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="13.0" x2="25.2" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.3,21.8 23.4,19.1 26.9,17.1" fill="#fc77be"/><circle cx="22.2" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `quat`

Negates the coefficients of i, j, & k

## [[Matrixification|Matrixification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="74" viewBox="0 0 90.0 74.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/><circle cx="10.7" cy="50.3" r="1.6"/><circle cx="25.9" cy="50.3" r="1.6"/><circle cx="41.2" cy="50.3" r="1.6"/><circle cx="56.4" cy="50.3" r="1.6"/><circle cx="71.7" cy="50.3" r="1.6"/><circle cx="86.9" cy="50.3" r="1.6"/><circle cx="3.1" cy="63.5" r="1.6"/><circle cx="18.3" cy="63.5" r="1.6"/><circle cx="33.6" cy="63.5" r="1.6"/><circle cx="48.8" cy="63.5" r="1.6"/><circle cx="64.1" cy="63.5" r="1.6"/><circle cx="79.3" cy="63.5" r="1.6"/></g><polyline class="hexpat-base" points="18.3,10.7 25.9,23.9 33.6,37.1 18.3,37.1 10.7,23.9 18.3,10.7 33.6,10.7 41.2,23.9 33.6,37.1 41.2,50.3 33.6,63.5 48.8,63.5 64.1,63.5 79.3,63.5 71.7,50.3 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:275;--dur:5.0s" points="18.3,10.7 25.9,23.9 33.6,37.1 18.3,37.1 10.7,23.9 18.3,10.7 33.6,10.7 41.2,23.9 33.6,37.1 41.2,50.3 33.6,63.5 48.8,63.5 64.1,63.5 79.3,63.5 71.7,50.3 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.3" y1="10.7" x2="20.7" y2="14.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,17.9 19.3,15.7 22.2,14.1" fill="#fc77be"/><circle cx="18.3" cy="10.7" r="2.6" fill="#fc77be"/></svg>

`quaternion` → `matrix`

Pushes a matrix that (should) represent the same rotation as the quaternion

## [[Quaterniation|Quaterniation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="74" viewBox="0 0 90.0 74.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/><circle cx="10.7" cy="50.3" r="1.6"/><circle cx="25.9" cy="50.3" r="1.6"/><circle cx="41.2" cy="50.3" r="1.6"/><circle cx="56.4" cy="50.3" r="1.6"/><circle cx="71.7" cy="50.3" r="1.6"/><circle cx="86.9" cy="50.3" r="1.6"/><circle cx="3.1" cy="63.5" r="1.6"/><circle cx="18.3" cy="63.5" r="1.6"/><circle cx="33.6" cy="63.5" r="1.6"/><circle cx="48.8" cy="63.5" r="1.6"/><circle cx="64.1" cy="63.5" r="1.6"/><circle cx="79.3" cy="63.5" r="1.6"/></g><polyline class="hexpat-base" points="18.3,10.7 25.9,23.9 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 18.3,37.1 33.6,37.1 41.2,50.3 33.6,63.5 48.8,63.5 64.1,63.5 79.3,63.5 71.7,50.3 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:275;--dur:5.0s" points="18.3,10.7 25.9,23.9 33.6,37.1 41.2,23.9 33.6,10.7 18.3,10.7 10.7,23.9 18.3,37.1 33.6,37.1 41.2,50.3 33.6,63.5 48.8,63.5 64.1,63.5 79.3,63.5 71.7,50.3 79.3,37.1 64.1,37.1 48.8,37.1 33.6,37.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.3" y1="10.7" x2="20.7" y2="14.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,17.9 19.3,15.7 22.2,14.1" fill="#fc77be"/><circle cx="18.3" cy="10.7" r="2.6" fill="#fc77be"/></svg>

`matrix` → `quaternion`

Pushes a quaternion that (should) reperesent the same rotation as the matrix

## [[Axis Angle Distillation|Axis Angle Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,13.0 31.5,29.0 40.7,45.0 50.0,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 50.0,61.0 31.5,61.0 40.7,77.0 59.2,77.0 68.5,61.0 59.2,45.0 50.0,61.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="22.2,13.0 31.5,29.0 40.7,45.0 50.0,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 50.0,61.0 31.5,61.0 40.7,77.0 59.2,77.0 68.5,61.0 59.2,45.0 50.0,61.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="13.0" x2="25.2" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.3,21.8 23.4,19.1 26.9,17.1" fill="#fc77be"/><circle cx="22.2" cy="13.0" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `quaternion`

Pushes a quaternion that reperesents a rotation by the given angle num radias around the given vector.

---
### Patterns on this page

- [[Quaternionic Exal]] · Pattern · Exaltation
- [[Quaternionic Dntg]] · Pattern · Disintegration
- [[Quaternionic Rfln. 1]] · Pattern · Reflection
- [[Quaternionic Rfln. i]] · Pattern · Reflection
- [[Quaternionic Rfln j]] · Pattern · Reflection
- [[Quaternionic Rfln k]] · Pattern · Reflection
- [[Additive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Subtractive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Multiplicative Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Division Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Length Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Quaterionic Prfn. 1]] · Pattern · Purification
- [[Quaterionic Prfn. i]] · Pattern · Purification
- [[Quaterionic Prfn. j]] · Pattern · Purification
- [[Quaterionic Prfn k]] · Pattern · Purification
- [[Quaternionic Inverse]] · Pattern · Other
- [[Matrixification]] · Pattern · Other
- [[Quaterniation]] · Pattern · Other
- [[Axis Angle Distillation]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/quaternionics)
