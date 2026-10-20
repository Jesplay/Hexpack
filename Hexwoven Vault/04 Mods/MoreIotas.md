---
tags: [hexcasting/mod]
version: "0.1.2"
---

# MoreIotas

> Addon mod for Hex Casting to add new Iota types.

🧩 [Modrinth page](https://modrinth.com/mod/Jmt7p37B) · 🌐 [Official online docs](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/)

**6** book entries · **43** patterns · **2** spells · **0** great spells

## Book entries

- [[Item Iotas]] — Hex Casting
- [[Items]] — Patterns
- [[Matrices]] — Patterns
- [[Strings]] — Patterns
- [[Types (MoreIotas)]] — Patterns
- [[Undocumented MoreIotas Patterns]] — Not in the Book

## Spells

- [[Write]] `vec, str | [str] → —`
- [[Name]] `str, entity → —`

## Patterns

- [[Sorter's Purification]] `itemtypable → itemtype`
- [[Classifier's Purification]] `any → iotatype`
- [[Physician's Purification]] `entity → entitytype`
- [[Sorter's Reflection]] `— → itemtype`
- [[Entity Distillation Type]] `entitytype, pos → entity | null`
- [[Zone Exaltation Type]] `entitytype, pos, num → [entity]`
- [[Zone Exaltation Not-Type]] `entitytype, pos, num → [entity]`
- [[Duelist's Purification]] `entity → item stack`
- [[Shieldbearer's Purification]] `entity → item stack`
- [[Hoarder's Distillation]] `vector, vector → list`
- [[Collector's Distillation]] `vector, vector → list`
- [[Treasurer's Distillation]] `itemtype|item stack, int → item stack`
- [[Transformation Purification]] `num | vec | list → mat`
- [[Restoration Purification]] `mat → num | vec | list`
- [[Restoration Purification II]] `mat → [[num]]`
- [[Identity Purification]] `int ≥ 0 → mat`
- [[Zero Distillation]] `int ≥ 0, int ≥ 0 → mat`
- [[Rotation Distillation]] `vec, num → mat`
- [[Multiplicative Distillation II]] `mat, mat → mat`
- [[Division Distillation II]] `mat, mat → mat`
- [[Power Distillation II]] `mat, mat → mat`
- [[Inverse Purification]] `mat → mat`
- [[Determinant Purification]] `mat → num`
- [[Tower Distillation]] `mat, mat → mat`
- [[Sprawling Distillation]] `mat, mat → mat`
- [[Toppling Gambit]] `mat, num → mat, mat`
- [[Mitosis Gambit]] `mat, num → mat, mat`
- [[Separation Distillation]] `str, str → [str]`
- [[Input Purification]] `str → num | null`
- [[Blank Reflection]] `— → str`
- [[Spacing Reflection]] `— → str`
- [[Comma Reflection]] `— → str`
- [[Breaking Reflection]] `— → str`
- [[Whisper Reflection]] `— → str`
- [[Listener's Reflection]] `— → str`
- [[Sifter's Gambit]] `str | null → —`
- [[Sifter's Reflection]] `— → str | null`
- [[Reader's Purification]] `vec → str | [str]`
- [[Scrivener's Purification]] `any → str`
- [[Patternmaster's Purification]] `pattern → str | null`
- [[Moniker Purification]] `entity → str`
- [[Case Distillation]] `str, bool | null → str`
- [[Additive Distillation II]] `— → —`
