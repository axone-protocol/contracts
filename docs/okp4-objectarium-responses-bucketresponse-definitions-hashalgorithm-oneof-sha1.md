# SHA1 Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm/oneOf/1
```

Represents the SHA-224 algorithm. SHA-224 is a variant of the SHA-2 family of hash functions that produces a 224-bit hash value. It is similar to SHA-256, but with a shorter output size. The computational cost of SHA-224 is moderate, and its relatively short hash length makes it easier to store and transmit.

SHA-224 hashes are stored on-chain as 56 hexadecimal characters.

See [the SHA-2 Wikipedia page](https://en.wikipedia.org/wiki/SHA-2) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 1 Type

`string` ([SHA1](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-sha1.md))

## 1 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value      | Explanation |
| :--------- | :---------- |
| `"sha224"` |             |
