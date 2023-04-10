# MD5 Schema

```txt
undefined#/responses/bucket/definitions/HashAlgorithm/oneOf/0
```

Represents the MD5 algorithm. MD5 is a widely used cryptographic hash function that produces a 128-bit hash value. The computational cost of MD5 is relatively low compared to other hash functions, but its short hash length makes it easier to find hash collisions. It is now considered insecure for cryptographic purposes, but can still used in non-security contexts.

MD5 hashes are stored on-chain as 32 hexadecimal characters.

See [the MD5 Wikipedia page](https://en.wikipedia.org/wiki/MD5) for more information.

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                                     |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :----------------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [okp4-objectarium.json\*](schema/okp4-objectarium.json "open original schema") |

## 0 Type

`string` ([MD5](okp4-objectarium-responses-bucketresponse-definitions-hashalgorithm-oneof-md5.md))

## 0 Constraints

**enum**: the value of this property must be equal to one of the following values:

| Value    | Explanation |
| :------- | :---------- |
| `"m_d5"` |             |
