# BreakStone Schema

```txt
undefined#/execute/oneOf/0
```

Break the stone making this contract unusable, by clearing all the related resources: - Unpin all the pinned objects on `cw-storage` contracts, if any. - Forget the main program (i.e. or at least unpin it). Only the contract admin is authorized to break it, if any. If already broken, this is a no-op.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                             |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :--------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [cw-law-stone.json\*](schema/cw-law-stone.json "open original schema") |

## 0 Type

`string` ([BreakStone](cw-law-stone-executemsg-oneof-breakstone.md))

## 0 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value           | Explanation |
| :-------------- | :---------- |
| `"break_stone"` |             |
