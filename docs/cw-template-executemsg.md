# ExecuteMsg Schema

```txt
undefined#/execute
```

Execute messages

| Abstract            | Extensible | Status         | Identifiable            | Custom Properties | Additional Properties | Access Restrictions | Defined In                                                           |
| :------------------ | :--------- | :------------- | :---------------------- | :---------------- | :-------------------- | :------------------ | :------------------------------------------------------------------- |
| Can be instantiated | No         | Unknown status | Unknown identifiability | Forbidden         | Allowed               | none                | [cw-template.json\*](schema/cw-template.json "open original schema") |

## execute Type

merged type ([ExecuteMsg](cw-template-executemsg.md))

one (and only one) of

*   [Increment](cw-template-executemsg-oneof-increment.md "check type definition")

*   [Reset](cw-template-executemsg-oneof-reset.md "check type definition")
