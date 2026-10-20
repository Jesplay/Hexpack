---
aliases: ["Matrices"]
tags: [hexbook/entry, mod/moreiotas, category/patterns, topic/math]
mod: "MoreIotas"
category: "Patterns"
entry_id: "hexcasting:patterns/matrices"
---

# Matrices

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[MoreIotas]]

Matrices are a helpful tool for manipulating vectors to do as I wish, able to represent any linear transformation that can be applied to a vector. Of note with these patterns is that many are forgiving with their inputs, accepting in place of matrices things that can be easily mapped to matrices, namely numbers and vectors.

For example, attempting to multiply a 3x3 matrix with a vector, rather than mishapping, will return a new vector. For more information on how to make use of matrices to their fullest potential, the below is a helpful resource.

🔗 [3blue1brown](https://www.youtube.com/watch?v=kYB8IZa5AuE&list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab&index=3)

## [[Transformation Purification|Transformation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,14.3 45.0,32.0 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,14.3 45.0,32.0 55.2,49.7 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,41.8 19.3,36.6 23.2,38.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`num | vec | list` → `mat`

Converts the [[Iotas|iota]] on top of the [[The Stack|stack]] into a matrix. This iota can be a number, vector, or list.

If it is a number, it becomes a 1x1 matrix. If it's a vector, it becomes a 3x1 matrix. If it's a list, it must be a list of purely numbers, purely vectors, or purely lists of numbers (each of which must be the same length). If it's a list of numbers it'll be 1 by n, if it's a list of vectors it'll be 3 by n, and if it's a list of lists it'll be n by m, where n is the inner list length and m is the outer list length (each inner list becomes a column).

## [[Restoration Purification|Restoration Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,14.3 45.0,32.0 34.8,49.7 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,14.3 45.0,32.0 34.8,49.7 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="68.7" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,41.8 66.8,38.8 70.7,36.6" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`mat` → `num | vec | list`

Converts the iota on top of the stack back from a matrix into a number, vector, or list depending on the vector's size.

A 1x1 matrix will become a number. A 1x3 or 3x1 matrix will become a vector. A 3 by n or n by 3 matrix will become a list of vectors, and any matrix not covered by the previous will become a list of lists of numbers, with each inner list being one column of the matrix.

## [[Restoration Purification II|Restoration Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="68.7" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,41.8 66.8,38.8 70.7,36.6" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`mat` → `[[num]]`

As [[Restoration Purification]], except it always returns a list of lists of numbers.

## [[Identity Purification|Identity Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 45.0,32.0 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 45.0,32.0 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,41.8 19.3,36.6 23.2,38.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`int ≥ 0` → `mat`

Accepts a positive integer n, and returns an n by n matrix with 1 along the diagonal and 0 elsewhere.

## [[Zero Distillation|Zero Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,41.8 19.3,36.6 23.2,38.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`int ≥ 0, int ≥ 0` → `mat`

Accepts positive integers n and m, and returns an n by m matrix of zeros.

## [[Rotation Distillation|Rotation Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 45.0,32.0 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:327;--dur:4.5s" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 45.0,32.0 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,41.8 19.3,36.6 23.2,38.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `mat`

Accepts a vec v and number θ, and returns a matrix representing rotating by θ radians around v.

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Adds two matrices together; they must have the same number of columns and rows.

## [[Multiplicative Distillation|Multiplicative Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,20.1 34.5,45.0 48.8,69.9 63.2,45.0 48.8,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="24.7" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,33.8 22.0,29.7 27.4,26.5" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Multiplies the first n by m by with an m by p matrix to get an n by p matrix.

## [[Division Distillation|Division Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="20.1,69.9 34.5,45.0 48.8,20.1 63.2,45.0 48.8,69.9 34.5,45.0 20.1,20.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Divides the first n by m matrix with an m by m matrix to get an n by m matrix.

## [[Power Distillation|Power Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,74.2 27.0,54.7 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="38.3,74.2 27.0,54.7 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="74.2" x2="34.7" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,63.5 36.8,66.8 32.5,69.2" fill="#fc77be"/><circle cx="38.3" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`mat, int` → `mat`

Accepts a square matrix and an integer, and raises the matrix to the power of the integer.

## [[Multiplicative Distillation II|Multiplicative Distillation II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="14.3" x2="17.6" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,24.1 15.6,21.1 19.5,18.9" fill="#fc77be"/><circle cx="14.3" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Multiplies the first n by m matrix elementwise with another n by m matrix.

## [[Division Distillation II|Division Distillation II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,49.7 24.5,32.0 34.8,14.3 45.0,32.0 34.8,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="14.3,49.7 24.5,32.0 34.8,14.3 45.0,32.0 34.8,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="49.7" x2="17.6" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,40.0 19.5,45.2 15.6,43.0" fill="#fc77be"/><circle cx="14.3" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Divides the first n by m matrix elementwise by another n by m matrix.

## [[Power Distillation II|Power Distillation II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,49.7 45.0,32.0 55.2,14.3 75.7,14.3 65.5,32.0 45.0,32.0 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:307;--dur:4.2s" points="34.8,49.7 45.0,32.0 55.2,14.3 75.7,14.3 65.5,32.0 45.0,32.0 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="49.7" x2="38.0" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,40.0 40.0,45.2 36.1,43.0" fill="#fc77be"/><circle cx="34.8" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Raises the first n by m matrix elementwise to the power of another n by m matrix.

## [[Retrograde Purification|Retrograde Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="31.8,64.4 58.2,64.4 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5 31.8,64.4 18.5,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="40.2" y2="64.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.3,64.4 40.2,67.3 40.2,61.5" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`mat` → `mat`

Accepts an n by m matrix and returns an m by n matrix where every entry has been flipped along the diagonal.

## [[Inverse Purification|Inverse Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="68.0,44.7 49.6,44.7 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 49.6,12.9 68.0,12.9 58.8,28.8 68.0,44.7 77.1,28.8 68.0,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:3.4s" points="68.0,44.7 49.6,44.7 31.2,44.7 12.9,44.7 22.0,28.8 12.9,12.9 31.2,12.9 49.6,12.9 68.0,12.9 58.8,28.8 68.0,44.7 77.1,28.8 68.0,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="68.0" y1="44.7" x2="62.1" y2="44.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="57.9,44.7 62.1,42.6 62.1,46.7" fill="#fc77be"/><circle cx="68.0" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`mat` → `mat`

Accepts a matrix of any shappe and returns its inverse. For square matrices with a nonzero determinant, the inverse is the matrix such that in\*out = identity.

For non-square matrices or those with a determinant of 0, this pattern computes the pseudoinverse: the matrix such that in\*out\*in = in, out\*in\*out = out, and both in\*out and out\*in are symmetrical.

For more information on the pseudoinverse, see the link below.

🔗 [Pseudoinverse](https://en.wikipedia.org/wiki/Moore%E2%80%93Penrose_inverse)

## [[Determinant Purification|Determinant Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 45.0,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="28.2" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.5,14.3 28.2,12.1 28.2,16.6" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`mat` → `num`

Accepts a square matrix and returns the determinant of that matrix.

## [[Tower Distillation|Tower Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:368;--dur:5.0s" points="24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,41.8 19.3,36.6 23.2,38.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Accepts an n by m matrix and a p by m matrix and produces a (n+p) by m matrix by putting the first matrix on top of the second.

## [[Sprawling Distillation|Sprawling Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:368;--dur:5.0s" points="65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="68.7" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="71.1,41.8 66.8,38.8 70.7,36.6" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`mat, mat` → `mat`

Accepts an n by m matrix and an n by p matrix and produces an n by (m+p) matrix by putting the first matrix to the left of the second.

## [[Toppling Gambit|Toppling Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:368;--dur:5.0s" points="24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 14.3,49.7 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 75.7,14.3 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="27.8" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,41.8 25.9,38.8 29.8,36.6" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`mat, num` → `mat, mat`

Accepts an n by m matrix and an int from 0 to n, and returns two matrices created by splitting the input matrix vertically at the inputted int.

## [[Mitosis Gambit|Mitosis Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:368;--dur:5.0s" points="65.5,32.0 55.2,49.7 45.0,32.0 34.8,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 14.3,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="32.0" x2="62.2" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.8,41.8 60.2,36.6 64.1,38.8" fill="#fc77be"/><circle cx="65.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`mat, num` → `mat, mat`

Accepts an n by m matrix and an int from 0 to m, and returns two matrices created by splitting the input matrix horizontally at the inputted int.

---
### Patterns on this page

- [[Transformation Purification]] · Pattern · Purification
- [[Restoration Purification]] · Pattern · Purification
- [[Restoration Purification II]] · Pattern · Purification
- [[Identity Purification]] · Pattern · Purification
- [[Zero Distillation]] · Pattern · Distillation
- [[Rotation Distillation]] · Pattern · Distillation
- [[Additive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Multiplicative Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Division Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Power Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Multiplicative Distillation II]] · Pattern · Distillation
- [[Division Distillation II]] · Pattern · Distillation
- [[Power Distillation II]] · Pattern · Distillation
- [[Retrograde Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Inverse Purification]] · Pattern · Purification
- [[Determinant Purification]] · Pattern · Purification
- [[Tower Distillation]] · Pattern · Distillation
- [[Sprawling Distillation]] · Pattern · Distillation
- [[Toppling Gambit]] · Pattern · Gambit
- [[Mitosis Gambit]] · Pattern · Gambit

**Sources:** 🌐 [Official online docs](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/matrices) · 🧩 [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
