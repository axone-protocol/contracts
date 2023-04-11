# SHA512 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/4
```

Represents the SHA-512 algorithm. SHA-512 is a member of the SHA-2 family of hash functions that produces a 512-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-512 is relatively high, but its longer hash length provides better security against hash collisions.

SHA-512 hashes are stored on-chain as 128 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 4 Type

`string` ([SHA512](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha512.md))

## 4 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha512"` |             |
