# AXONE Verifiable Credential Contract (`axone-vc`)

The AXONE Verifiable Credential contract attaches **verifiable credential capabilities**
to a resource represented by an **Abstract Account (AA)**.

It is the AXONE contract responsible for binding a credential authority to that resource
and structuring the credential lifecycle around submission, revocation, suspension,
reinstatement and verification.

It targets credentials represented as RDF datasets.

## InstantiateMsg

| variant        | description |
| -------------- | ----------- |
| InstantiateMsg | **object**. |

## ExecuteMsg

### ExecuteMsg::foo

| parameter   | description                |
| ----------- | -------------------------- |
| `foo`       | _(Required.) _ **object**. |
| `foo.value` | _(Required.) _ **string**. |

## QueryMsg

### QueryMsg::foo

| parameter | description                |
| --------- | -------------------------- |
| `foo`     | _(Required.) _ **object**. |

## MigrateMsg

### MigrateMsg::MigrateMsg

| parameter | description |
| --------- | ----------- |

## Responses

### foo

| property | description                |
| -------- | -------------------------- |
| `value`  | _(Required.) _ **string**. |

---

_Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-vc.json` (`a9d17d807a89662b`)_
