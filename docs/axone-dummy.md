# Axone Dummy Contract

This is a simple dummy contract kept as a placeholder during the refactoring of the AXONE protocol contracts. It does not implement any real functionality.

## InstantiateMsg

Instantiate message

| type        |
| ----------- |
| **object**. |

## ExecuteMsg

Execute messages

### ExecuteMsg::Foo

| literal |
| ------- |
| `"foo"` |

## QueryMsg

Query messages

### QueryMsg::Bar

| parameter | description                |
| --------- | -------------------------- |
| `bar`     | _(Required.) _ **object**. |
| `bar.msg` | _(Required.) _ **string**. |

## Responses

### bar

| property | description                                  |
| -------- | -------------------------------------------- |
| `msg`    | _(Required.) _ **string**. The message value |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-dummy.json` (`7cfdeffbb29ab213`)_
