# SHA384 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/3
```

Represents the SHA-384 algorithm. SHA-384 is a variant of the SHA-2 family of hash functions that produces a 384-bit hash value. It is similar to SHA-512, but with a shorter output size. The computational cost of SHA-384 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-384 hashes are stored on-chain as 96 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 3 Type

`string` ([SHA384](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha384.md))

## 3 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha384"` |             |
