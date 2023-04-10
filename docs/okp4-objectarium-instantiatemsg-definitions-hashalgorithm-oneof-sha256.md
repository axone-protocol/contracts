# SHA256 Schema

```txt
undefined#/instantiate/definitions/HashAlgorithm/oneOf/2
```

Represents the SHA-256 algorithm. SHA-256 is a member of the SHA-2 family of hash functions that produces a 256-bit hash value. It is widely used in cryptography and other security-related applications. The computational cost of SHA-256 is moderate, and its hash length strikes a good balance between security and convenience.

SHA-256 hashes are stored on-chain as 64 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 2 Type

`string` ([SHA256](okp4-objectarium-instantiatemsg-definitions-hashalgorithm-oneof-sha256.md))

## 2 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha256"` |             |
