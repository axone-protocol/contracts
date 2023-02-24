use crate::error::BucketError;
use crate::ContractError::NotImplemented;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdError, StdResult,
};
use cw2::set_contract_version;

use crate::crypto::sha256_hash;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ObjectId, QueryMsg};
use crate::state::{objects, pins, Bucket, Object, Pin, BUCKET, DATA};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:storage";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let bucket = Bucket::new(msg.bucket, msg.limits.into())?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    BUCKET.save(deps.storage, &bucket)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StoreObject { data, pin } => execute::store_object(deps, info, data, pin),
        _ => Err(NotImplemented {}),
    }
}

pub mod execute {
    use super::*;
    use crate::state::Limits;

    pub fn store_object(
        deps: DepsMut,
        info: MessageInfo,
        data: Binary,
        pin: bool,
    ) -> Result<Response, ContractError> {
        let size = data.len() as u128;
        // TODO: store object count in bucket instead of computing it?
        let object_count = objects()
            .keys_raw(deps.storage, None, None, Order::Ascending)
            .count();
        BUCKET.update(deps.storage, |mut bucket| -> Result<_, ContractError> {
            bucket.size += size;
            match bucket.limits {
                Limits {
                    max_object_size: Some(max),
                    ..
                } if size > max.u128() => Err(BucketError::MaxObjectSizeLimitExceeded.into()),
                Limits {
                    max_objects: Some(max),
                    ..
                } if object_count as u128 >= max.u128() => {
                    Err(BucketError::MaxObjectsLimitExceeded.into())
                }
                Limits {
                    max_object_pins: Some(max),
                    ..
                } if pin && max.u128() < 1u128 => {
                    Err(BucketError::MaxObjectPinsLimitExceeded.into())
                }
                Limits {
                    max_total_size: Some(max),
                    ..
                } if bucket.size > max.u128() => Err(BucketError::MaxTotalSizeLimitExceeded.into()),
                _ => Ok(bucket),
            }
        })?;

        let object = &Object {
            id: sha256_hash(&data.0),
            owner: info.sender.clone(),
            size,
        };
        let res = Response::new()
            .add_attribute("action", "store_object")
            .add_attribute("id", object.id.clone());

        let data_path = DATA.key(object.id.clone());
        if data_path.has(deps.storage) {
            // TODO: maybe throw an error if the owner is different? Or if object already exists?
            return Ok(res);
        }

        data_path.save(deps.storage, &data.0)?;
        objects().save(deps.storage, object.id.clone(), object)?;

        if pin {
            pins().save(
                deps.storage,
                (object.id.clone(), info.sender.clone()),
                &Pin {
                    id: object.id.clone(),
                    address: info.sender,
                },
            )?;
        }

        Ok(res)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Bucket {} => to_binary(&query::bucket(deps)?),
        QueryMsg::Object { id } => to_binary(&query::object(deps, id)?),
        _ => Err(StdError::generic_err("Not implemented")),
    }
}

pub mod query {
    use super::*;
    use crate::msg::{BucketResponse, ObjectResponse};

    pub fn bucket(deps: Deps) -> StdResult<BucketResponse> {
        let bucket = BUCKET.load(deps.storage)?;

        Ok(BucketResponse {
            name: bucket.name,
            limits: bucket.limits.into(),
        })
    }

    pub fn object(deps: Deps, id: ObjectId) -> StdResult<ObjectResponse> {
        objects()
            .load(deps.storage, id)
            .map(|object| ObjectResponse {
                id: object.id,
                size: object.size.into(),
                owner: object.owner.into(),
                is_pinned: pins()
                    .idx
                    .object
                    .keys_raw(deps.storage, None, None, Order::Ascending)
                    .next()
                    .is_some(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::BucketError;
    use crate::msg::{BucketLimits, BucketResponse};
    use base64::{engine::general_purpose, Engine as _};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::StdError::NotFound;
    use cosmwasm_std::{from_binary, Attribute, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "foo".to_string(),
            limits: BucketLimits {
                max_total_size: None,
                max_objects: None,
                max_object_size: None,
                max_object_pins: None,
            },
        };
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_binary(&res).unwrap();
        assert_eq!("foo", value.name);
    }

    #[test]
    fn proper_limits_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "bar".to_string(),
            limits: BucketLimits {
                max_total_size: Some(Uint128::new(20000)),
                max_objects: Some(Uint128::new(10)),
                max_object_size: Some(Uint128::new(2000)),
                max_object_pins: Some(Uint128::new(1)),
            },
        };
        let info = mock_info("creator", &[]);

        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_binary(&res).unwrap();
        assert_eq!("bar", value.name);
        assert_eq!(Uint128::new(20000), value.limits.max_total_size.unwrap());
        assert_eq!(Uint128::new(10), value.limits.max_objects.unwrap());
        assert_eq!(Uint128::new(2000), value.limits.max_object_size.unwrap());
        assert_eq!(Uint128::new(1), value.limits.max_object_pins.unwrap());
    }

    #[test]
    fn empty_name_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "".to_string(),
            limits: BucketLimits {
                max_total_size: None,
                max_objects: None,
                max_object_size: None,
                max_object_pins: None,
            },
        };
        let info = mock_info("creator", &[]);

        let err = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert_eq!(err, ContractError::Bucket(BucketError::EmptyName));
    }

    #[test]
    fn whitespace_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "foo bar".to_string(),
            limits: BucketLimits {
                max_total_size: None,
                max_objects: None,
                max_object_size: None,
                max_object_pins: None,
            },
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_binary(&res).unwrap();
        assert_eq!("foobar", value.name);
    }

    #[test]
    fn store_object_without_limits() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            bucket: String::from("test"),
            limits: BucketLimits {
                max_objects: None,
                max_object_size: None,
                max_total_size: None,
                max_object_pins: None,
            },
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let obj1 = (
            general_purpose::STANDARD.encode("hello"),
            true,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            5,
        );
        let obj2 = (
            general_purpose::STANDARD.encode("okp4"),
            false,
            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
            4,
        );

        for obj in vec![obj1.clone(), obj2.clone()] {
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(obj.0.as_str()).unwrap(),
                pin: obj.1,
            };
            let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
            assert_eq!(
                res.attributes,
                vec![
                    Attribute::new("action", "store_object"),
                    Attribute::new("id", obj.2),
                ]
            );

            assert_eq!(
                Binary::from_base64(obj.0.as_str()).unwrap(),
                Binary::from(DATA.load(&deps.storage, String::from(obj.2)).unwrap()),
            );

            let created = objects().load(&deps.storage, String::from(obj.2)).unwrap();
            assert_eq!(created.id, obj.2);
            assert_eq!(created.owner, info.clone().sender);
            assert_eq!(created.size, obj.3);

            assert_eq!(
                pins().has(&deps.storage, (String::from(obj.2), info.clone().sender)),
                obj.1,
            );
        }

        assert_eq!(BUCKET.load(&deps.storage).unwrap().size, obj1.3 + obj2.3);

        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(obj1.0.as_str()).unwrap(),
            pin: obj1.1,
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                Attribute::new("action", "store_object"),
                Attribute::new("id", obj1.2),
            ]
        );
        assert_eq!(
            objects()
                .keys_raw(&deps.storage, None, None, Order::Ascending)
                .count(),
            2
        );
        assert_eq!(
            pins()
                .keys_raw(&deps.storage, None, None, Order::Ascending)
                .count(),
            1
        );
    }

    #[test]
    fn store_object_limits() {
        let cases = vec![
            (
                BucketLimits {
                    max_objects: Some(2u128.into()),
                    max_object_size: None,
                    max_total_size: None,
                    max_object_pins: None,
                },
                None,
            ),
            (
                BucketLimits {
                    max_objects: None,
                    max_object_size: Some(5u128.into()),
                    max_total_size: None,
                    max_object_pins: None,
                },
                None,
            ),
            (
                BucketLimits {
                    max_objects: None,
                    max_object_size: None,
                    max_total_size: Some(9u128.into()),
                    max_object_pins: None,
                },
                None,
            ),
            (
                BucketLimits {
                    max_objects: None,
                    max_object_size: None,
                    max_total_size: None,
                    max_object_pins: Some(1u128.into()),
                },
                None,
            ),
            (
                BucketLimits {
                    max_objects: Some(1u128.into()),
                    max_object_size: None,
                    max_total_size: None,
                    max_object_pins: None,
                },
                Some(MaxObjectsLimitExceeded {}),
            ),
            (
                BucketLimits {
                    max_objects: None,
                    max_object_size: Some(4u128.into()),
                    max_total_size: None,
                    max_object_pins: None,
                },
                Some(MaxObjectSizeLimitExceeded {}),
            ),
            (
                BucketLimits {
                    max_objects: None,
                    max_object_size: None,
                    max_total_size: Some(8u128.into()),
                    max_object_pins: None,
                },
                Some(MaxTotalSizeLimitExceeded {}),
            ),
            (
                BucketLimits {
                    max_objects: None,
                    max_object_size: None,
                    max_total_size: None,
                    max_object_pins: Some(0u128.into()),
                },
                Some(MaxObjectPinsLimitExceeded {}),
            ),
        ];

        let obj1 = general_purpose::STANDARD.encode("okp4");
        let obj2 = general_purpose::STANDARD.encode("hello");

        for case in cases {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &[]);

            let msg = InstantiateMsg {
                bucket: String::from("test"),
                limits: case.0,
            };
            instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(obj1.as_str()).unwrap(),
                pin: false,
            };
            execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(obj2.as_str()).unwrap(),
                pin: true,
            };
            let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

            assert_eq!(res.err(), case.1);
        }
    }

    #[test]
    fn object() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            bucket: String::from("test"),
            limits: BucketLimits {
                max_objects: None,
                max_object_size: None,
                max_total_size: None,
                max_object_pins: None,
            },
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        match query::object(
            deps.as_ref(),
            ObjectId::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
        )
        .err()
        .unwrap()
        {
            NotFound { .. } => (),
            _ => panic!("assertion failed"),
        }

        let data = general_purpose::STANDARD.encode("hello");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: true,
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let res = query::object(
            deps.as_ref(),
            ObjectId::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"),
        )
        .unwrap();
        assert_eq!(
            res.id,
            ObjectId::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
        );
        assert_eq!(res.owner, info.sender);
        assert_eq!(res.is_pinned, true);
        assert_eq!(res.size.u128(), 5u128);

        let data = general_purpose::STANDARD.encode("okp4");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let res = query::object(
            deps.as_ref(),
            ObjectId::from("315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6"),
        )
        .unwrap();
        assert_eq!(
            res.id,
            ObjectId::from("315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6")
        );
        assert_eq!(res.owner, info.sender);
        assert_eq!(res.is_pinned, false);
        assert_eq!(res.size.u128(), 4u128);
    }
}
