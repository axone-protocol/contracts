use crate::error::BucketError;
use crate::ContractError::NotImplemented;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
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
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    msg.pagination.validate()?;
    let bucket = Bucket::new(
        info.sender,
        msg.bucket,
        msg.limits.into(),
        msg.pagination.into(),
    )?;

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
        ExecuteMsg::PinObject { id } => execute::pin_object(deps, info, id),
        ExecuteMsg::UnpinObject { id } => execute::unpin_object(deps, info, id),
        _ => Err(NotImplemented {}),
    }
}

pub mod execute {
    use super::*;
    use crate::state::Limits;
    use cosmwasm_std::{StdError, Uint128};
    use std::any::type_name;

    pub fn store_object(
        deps: DepsMut,
        info: MessageInfo,
        data: Binary,
        pin: bool,
    ) -> Result<Response, ContractError> {
        let size = (data.len() as u128).into();
        BUCKET.update(deps.storage, |mut bucket| -> Result<_, ContractError> {
            bucket.stat.size += size;
            bucket.stat.object_count += Uint128::one();
            match bucket.limits {
                Limits {
                    max_object_size: Some(max),
                    ..
                } if size > max => Err(BucketError::MaxObjectSizeLimitExceeded(size, max).into()),
                Limits {
                    max_objects: Some(max),
                    ..
                } if bucket.stat.object_count > max => {
                    Err(BucketError::MaxObjectsLimitExceeded(bucket.stat.object_count, max).into())
                }
                Limits {
                    max_object_pins: Some(max),
                    ..
                } if pin && max < Uint128::one() => {
                    Err(BucketError::MaxObjectPinsLimitExceeded(Uint128::one(), max).into())
                }
                Limits {
                    max_total_size: Some(max),
                    ..
                } if bucket.stat.size > max => {
                    Err(BucketError::MaxTotalSizeLimitExceeded(bucket.stat.size, max).into())
                }
                _ => Ok(bucket),
            }
        })?;

        let object = &Object {
            id: sha256_hash(&data.0),
            owner: info.sender.clone(),
            size,
            pin_count: if pin { Uint128::one() } else { Uint128::zero() },
        };
        let res = Response::new()
            .add_attribute("action", "store_object")
            .add_attribute("id", object.id.clone());

        let data_path = DATA.key(object.id.clone());
        if data_path.has(deps.storage) {
            return Err(ContractError::Bucket(BucketError::ObjectAlreadyStored));
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

    pub fn pin_object(
        deps: DepsMut,
        info: MessageInfo,
        object_id: ObjectId,
    ) -> Result<Response, ContractError> {
        let res = Response::new()
            .add_attribute("action", "pin_object")
            .add_attribute("id", object_id.clone());

        if pins().has(deps.storage, (object_id.clone(), info.sender.clone())) {
            return Ok(res);
        }

        let o = objects().update(
            deps.storage,
            object_id.clone(),
            |o| -> Result<Object, StdError> {
                o.map(|mut e: Object| -> Object {
                    e.pin_count += Uint128::one();
                    e
                })
                .ok_or_else(|| StdError::not_found(type_name::<Object>()))
            },
        )?;

        let bucket = BUCKET.load(deps.storage)?;

        match bucket.limits {
            Limits {
                max_object_pins: Some(max),
                ..
            } if max < o.pin_count => {
                Err(BucketError::MaxObjectPinsLimitExceeded(o.pin_count, max).into())
            }
            _ => {
                pins().save(
                    deps.storage,
                    (object_id.clone(), info.sender.clone()),
                    &Pin {
                        id: object_id,
                        address: info.sender,
                    },
                )?;
                Ok(res)
            }
        }
    }

    pub fn unpin_object(
        deps: DepsMut,
        info: MessageInfo,
        object_id: ObjectId,
    ) -> Result<Response, ContractError> {
        let object_path = objects().key(object_id.clone());
        let mut object = object_path.load(deps.storage)?;

        let res = Response::new()
            .add_attribute("action", "unpin_object")
            .add_attribute("id", object_id.clone());

        if !pins().has(deps.storage, (object_id.clone(), info.sender.clone())) {
            return Ok(res);
        }

        object.pin_count -= Uint128::one();
        object_path.save(deps.storage, &object)?;

        pins().remove(deps.storage, (object_id, info.sender))?;

        Ok(res)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Bucket {} => to_binary(&query::bucket(deps)?),
        QueryMsg::Object { id } => to_binary(&query::object(deps, id)?),
        QueryMsg::ObjectData { id } => to_binary(&query::data(deps, id)?),
        QueryMsg::Objects {
            address,
            after,
            first,
        } => to_binary(&query::fetch_objects(deps, address, after, first)?),
        QueryMsg::ObjectPins { id, after, first } => {
            to_binary(&query::object_pins(deps, id, after, first)?)
        }
    }
}

pub mod query {
    use super::*;
    use crate::cursor;
    use crate::msg::{
        BucketResponse, Cursor, ObjectPinsResponse, ObjectResponse, ObjectsResponse, PageInfo,
    };
    use crate::pagination_handler::PaginationHandler;
    use cosmwasm_std::Addr;

    pub fn bucket(deps: Deps) -> StdResult<BucketResponse> {
        let bucket = BUCKET.load(deps.storage)?;

        Ok(BucketResponse {
            name: bucket.name,
            limits: bucket.limits.into(),
            pagination: bucket.pagination.into(),
        })
    }

    pub fn object(deps: Deps, id: ObjectId) -> StdResult<ObjectResponse> {
        objects()
            .load(deps.storage, id)
            .map(|object| (&object).into())
    }

    pub fn data(deps: Deps, id: ObjectId) -> StdResult<Binary> {
        DATA.load(deps.storage, id).map(Binary::from)
    }

    pub fn fetch_objects(
        deps: Deps,
        address: Option<String>,
        after: Option<Cursor>,
        first: Option<u32>,
    ) -> StdResult<ObjectsResponse> {
        let address = match address {
            Some(raw) => Some(deps.api.addr_validate(&raw)?),
            _ => None,
        };

        let handler: PaginationHandler<Object, String> =
            PaginationHandler::from(BUCKET.load(deps.storage)?.pagination);

        let page: (Vec<Object>, PageInfo) = handler.query_page(
            |min_bound| match address {
                Some(addr) => objects().idx.owner.prefix(addr).range(
                    deps.storage,
                    min_bound,
                    None,
                    Order::Ascending,
                ),
                _ => objects().range(deps.storage, min_bound, None, Order::Ascending),
            },
            cursor::decode,
            |o: &Object| cursor::encode(o.id.clone()),
            after,
            first,
        )?;

        Ok(ObjectsResponse {
            data: page.0.iter().map(|object| object.into()).collect(),
            page_info: page.1,
        })
    }

    pub fn object_pins(
        deps: Deps,
        id: ObjectId,
        after: Option<Cursor>,
        first: Option<u32>,
    ) -> StdResult<ObjectPinsResponse> {
        let handler: PaginationHandler<Pin, (String, Addr)> =
            PaginationHandler::from(BUCKET.load(deps.storage)?.pagination);

        let page: (Vec<Pin>, PageInfo) = handler.query_page(
            |min_bound| {
                pins().idx.object.prefix(id.clone()).range(
                    deps.storage,
                    min_bound,
                    None,
                    Order::Ascending,
                )
            },
            |c| {
                cursor::decode(c)
                    .and_then(|raw| deps.api.addr_validate(raw.as_str()))
                    .map(|addr| (id.clone(), addr))
            },
            |pin: &Pin| cursor::encode(pin.clone().address.into_string()),
            after,
            first,
        )?;

        Ok(ObjectPinsResponse {
            data: page
                .0
                .iter()
                .map(|pin: &Pin| pin.address.as_str().to_string())
                .collect(),
            page_info: page.1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::BucketError;
    use crate::error::BucketError::MaxObjectPinsLimitExceeded;
    use crate::msg::{
        BucketLimits, BucketResponse, ObjectPinsResponse, ObjectResponse, ObjectsResponse,
        PageInfo, PaginationConfig,
    };
    use base64::{engine::general_purpose, Engine as _};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::StdError::NotFound;
    use cosmwasm_std::{from_binary, Attribute, Order, StdError, Uint128};
    use std::any::type_name;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "foo".to_string(),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
        };
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_binary(&res).unwrap();
        assert_eq!("foo", value.name);
        assert_eq!(value.limits, BucketLimits::new());
        assert_eq!(value.pagination.max_page_size, Some(30));
        assert_eq!(value.pagination.default_page_size, Some(10));

        // check internal state too
        let bucket = BUCKET.load(&deps.storage).unwrap();
        assert_eq!("creator", bucket.owner.into_string());
        assert_eq!(Uint128::zero(), bucket.stat.size);
        assert_eq!(Uint128::zero(), bucket.stat.object_count);
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
            pagination: PaginationConfig::new()
                .set_max_page_size(50)
                .set_default_page_size(30),
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
        assert_eq!(value.pagination.max_page_size, Some(50));
        assert_eq!(value.pagination.default_page_size, Some(30));
    }

    #[test]
    fn proper_pagination_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            bucket: "bar".to_string(),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new()
                .set_max_page_size(50)
                .set_default_page_size(30),
        };
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_binary(&res).unwrap();
        assert_eq!(value.pagination.max_page_size, Some(50));
        assert_eq!(value.pagination.default_page_size, Some(30));
    }

    #[test]
    fn invalid_pagination_initialization() {
        let cases = vec![
            (
                PaginationConfig::new().set_max_page_size(u32::MAX),
                StdError::generic_err("'max_page_size' cannot exceed 'u32::MAX - 1'"),
            ),
            (
                PaginationConfig::new().set_default_page_size(31),
                StdError::generic_err("'default_page_size' cannot exceed 'max_page_size'"),
            ),
        ];
        for case in cases {
            let mut deps = mock_dependencies();
            let msg = InstantiateMsg {
                bucket: "bar".to_string(),
                limits: BucketLimits::new(),
                pagination: case.0,
            };
            match instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg) {
                Err(err) => assert_eq!(err, ContractError::Std(case.1)),
                _ => panic!("assertion failure!"),
            }
        }
    }

    #[test]
    fn empty_name_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "".to_string(),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
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
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
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
        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg {
                bucket: "test".to_string(),
                limits: BucketLimits::new(),
                pagination: PaginationConfig::new(),
            },
        )
        .unwrap();

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
            assert_eq!(created.size.u128(), obj.3);
            assert_eq!(
                created.pin_count,
                if obj.1 {
                    Uint128::one()
                } else {
                    Uint128::zero()
                }
            );

            assert_eq!(
                pins().has(&deps.storage, (String::from(obj.2), info.clone().sender)),
                obj.1,
            );
        }

        let bucket = BUCKET.load(&deps.storage).unwrap();
        assert_eq!(bucket.stat.size.u128(), obj1.3 + obj2.3);
        assert_eq!(bucket.stat.object_count.u128(), 2);
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
    fn store_object_already_stored() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            bucket: String::from("test"),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let object = general_purpose::STANDARD.encode("already existing object");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(object.as_str()).unwrap(),
            pin: true,
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(
            execute(deps.as_mut(), mock_env(), info, msg).err(),
            Some(ContractError::Bucket(BucketError::ObjectAlreadyStored)),
        );
    }

    #[test]
    fn store_object_limits() {
        let cases = vec![
            (BucketLimits::new().set_max_objects(2u128.into()), None),
            (BucketLimits::new().set_max_object_size(5u128.into()), None),
            (BucketLimits::new().set_max_total_size(9u128.into()), None),
            (BucketLimits::new().set_max_object_pins(1u128.into()), None),
            (
                BucketLimits::new().set_max_objects(1u128.into()),
                Some(ContractError::Bucket(BucketError::MaxObjectsLimitExceeded(
                    2u128.into(),
                    1u128.into(),
                ))),
            ),
            (
                BucketLimits::new().set_max_object_size(4u128.into()),
                Some(ContractError::Bucket(
                    BucketError::MaxObjectSizeLimitExceeded(5u128.into(), 4u128.into()),
                )),
            ),
            (
                BucketLimits::new().set_max_total_size(8u128.into()),
                Some(ContractError::Bucket(
                    BucketError::MaxTotalSizeLimitExceeded(9u128.into(), 8u128.into()),
                )),
            ),
            (
                BucketLimits::new().set_max_object_pins(0u128.into()),
                Some(ContractError::Bucket(
                    BucketError::MaxObjectPinsLimitExceeded(1u128.into(), 0u128.into()),
                )),
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
                pagination: PaginationConfig::new(),
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
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        match query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Object {
                id: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".to_string(),
            },
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

        let msg = QueryMsg::Object {
            id: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".to_string(),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectResponse = from_binary(&result).unwrap();
        assert_eq!(
            response.id,
            ObjectId::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824")
        );
        assert_eq!(response.owner, info.sender);
        assert!(response.is_pinned);
        assert_eq!(response.size.u128(), 5u128);

        let data = general_purpose::STANDARD.encode("okp4");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::Object {
            id: "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6".to_string(),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectResponse = from_binary(&result).unwrap();
        assert_eq!(
            response.id,
            ObjectId::from("315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6")
        );
        assert_eq!(response.owner, info.sender);
        assert!(!response.is_pinned);
        assert_eq!(response.size.u128(), 4u128);
    }

    #[test]
    fn object_data() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            bucket: String::from("test"),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        match query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ObjectData {
                id: "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6".to_string(),
            },
        )
        .err()
        .unwrap()
        {
            NotFound { .. } => (),
            _ => panic!("assertion failed"),
        }

        let data = Binary::from_base64(general_purpose::STANDARD.encode("okp4").as_str()).unwrap();
        let msg = ExecuteMsg::StoreObject {
            data: data.clone(),
            pin: false,
        };
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = QueryMsg::ObjectData {
            id: "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6".to_string(),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        assert_eq!(result, to_binary(&data).unwrap());
    }

    struct TestPinCase {
        objects: Vec<ObjectId>,
        senders: Vec<MessageInfo>,
        expected_count: usize,
        expected_error: Option<ContractError>,
        expected_object_pin_count: Vec<(ObjectId, Uint128)>,
    }

    #[test]
    fn pin_object() {
        let cases = vec![
            TestPinCase {
                // One object, 1 one pinner => 1 pin
                objects: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                senders: vec![mock_info("bob", &[])],
                expected_count: 1,
                expected_error: None,
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::one(),
                )],
            },
            TestPinCase {
                // Same object, two pinners => 2 pin
                objects: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                ],
                senders: vec![mock_info("bob", &[]), mock_info("alice", &[])],
                expected_count: 2,
                expected_error: None,
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::new(2),
                )],
            },
            TestPinCase {
                // Same object, one pinner twice => 1 pin
                objects: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                ],
                senders: vec![mock_info("bob", &[]), mock_info("bob", &[])],
                expected_count: 1,
                expected_error: None,
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::one(),
                )],
            },
            TestPinCase {
                // two objects, same pinner => 2 pin
                objects: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                ],
                senders: vec![mock_info("bob", &[]), mock_info("bob", &[])],
                expected_count: 2,
                expected_error: None,
                expected_object_pin_count: vec![
                    (
                        ObjectId::from(
                            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                        ),
                        Uint128::one(),
                    ),
                    (
                        ObjectId::from(
                            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                        ),
                        Uint128::one(),
                    ),
                ],
            },
            TestPinCase {
                // two objects, two pinner => 2 pin
                objects: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                ],
                senders: vec![mock_info("bob", &[]), mock_info("alice", &[])],
                expected_count: 2,
                expected_error: None,
                expected_object_pin_count: vec![
                    (
                        ObjectId::from(
                            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                        ),
                        Uint128::one(),
                    ),
                    (
                        ObjectId::from(
                            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                        ),
                        Uint128::one(),
                    ),
                ],
            },
            TestPinCase {
                // two objects, two pinner, twice 1 pinner => 2 pin
                objects: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                ],
                senders: vec![
                    mock_info("bob", &[]),
                    mock_info("alice", &[]),
                    mock_info("alice", &[]),
                ],
                expected_count: 2,
                expected_error: None,
                expected_object_pin_count: vec![
                    (
                        ObjectId::from(
                            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                        ),
                        Uint128::one(),
                    ),
                    (
                        ObjectId::from(
                            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                        ),
                        Uint128::one(),
                    ),
                ],
            },
            TestPinCase {
                // exceed limits
                objects: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                ],
                senders: vec![
                    mock_info("bob", &[]),
                    mock_info("alice", &[]),
                    mock_info("martin", &[]),
                    mock_info("pierre", &[]),
                ],
                expected_count: 3,
                expected_error: Some(ContractError::Bucket(MaxObjectPinsLimitExceeded(
                    Uint128::new(3),
                    Uint128::new(2),
                ))),
                expected_object_pin_count: vec![
                    (
                        ObjectId::from(
                            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                        ),
                        Uint128::one(),
                    ),
                    (
                        ObjectId::from(
                            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                        ),
                        Uint128::new(2),
                    ),
                ],
            },
            TestPinCase {
                // Object not exists
                objects: vec![ObjectId::from("NOTFOUND")],
                senders: vec![mock_info("bob", &[])],
                expected_count: 0,
                expected_error: Some(ContractError::Std(StdError::not_found(
                    type_name::<Object>(),
                ))),
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::zero(),
                )],
            },
        ];

        for case in cases {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &[]);

            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg {
                    bucket: "test".to_string(),
                    limits: BucketLimits::new().set_max_object_pins(Uint128::new(2)),
                    pagination: PaginationConfig::new(),
                },
            )
            .unwrap();

            let data = general_purpose::STANDARD.encode("okp4");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("data");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("hello");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let mut last_result: Option<Result<Response, ContractError>> = None;
            case.objects
                .iter()
                .zip(case.senders)
                .for_each(|(object_id, info)| {
                    last_result = Some(execute(
                        deps.as_mut(),
                        mock_env(),
                        info,
                        ExecuteMsg::PinObject {
                            id: object_id.clone(),
                        },
                    ));
                });

            match case.expected_error {
                Some(err) => assert_eq!(last_result.unwrap().unwrap_err(), err),
                _ => {
                    assert_eq!(
                        pins()
                            .keys_raw(&deps.storage, None, None, Order::Ascending)
                            .count(),
                        case.expected_count
                    );
                    for (object_id, count) in case.expected_object_pin_count {
                        assert_eq!(
                            objects().load(&deps.storage, object_id).unwrap().pin_count,
                            count
                        );
                    }
                }
            }
        }
    }

    struct TestUnpinCase {
        pin: Vec<ObjectId>,
        pin_senders: Vec<MessageInfo>,
        unpin: Vec<ObjectId>,
        unpin_senders: Vec<MessageInfo>,
        expected_count: usize,
        expected_error: Option<ContractError>,
        expected_object_pin_count: Vec<(ObjectId, Uint128)>,
    }

    #[test]
    fn unpin_object() {
        let cases = vec![
            TestUnpinCase {
                pin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                pin_senders: vec![mock_info("bob", &[])],
                unpin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                unpin_senders: vec![mock_info("bob", &[])],
                expected_count: 0,
                expected_error: None,
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::zero(),
                )],
            },
            TestUnpinCase {
                pin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                pin_senders: vec![mock_info("bob", &[])],
                unpin: vec![ObjectId::from(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                )],
                unpin_senders: vec![mock_info("bob", &[])],
                expected_count: 1,
                expected_error: None,
                expected_object_pin_count: vec![
                    (
                        ObjectId::from(
                            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                        ),
                        Uint128::one(),
                    ),
                    (
                        ObjectId::from(
                            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                        ),
                        Uint128::zero(),
                    ),
                ],
            },
            TestUnpinCase {
                pin: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                ],
                pin_senders: vec![mock_info("bob", &[]), mock_info("bob", &[])],
                unpin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                unpin_senders: vec![mock_info("bob", &[])],
                expected_count: 1,
                expected_error: None,
                expected_object_pin_count: vec![
                    (
                        ObjectId::from(
                            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                        ),
                        Uint128::zero(),
                    ),
                    (
                        ObjectId::from(
                            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                        ),
                        Uint128::one(),
                    ),
                ],
            },
            TestUnpinCase {
                pin: vec![],
                pin_senders: vec![],
                unpin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                unpin_senders: vec![mock_info("bob", &[])],
                expected_count: 0,
                expected_error: None,
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::zero(),
                )],
            },
            TestUnpinCase {
                pin: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                ],
                pin_senders: vec![
                    mock_info("bob", &[]),
                    mock_info("alice", &[]),
                    mock_info("martin", &[]),
                    mock_info("pierre", &[]),
                ],
                unpin: vec![ObjectId::from(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                )],
                unpin_senders: vec![mock_info("martin", &[])],
                expected_count: 3,
                expected_error: None,
                expected_object_pin_count: vec![
                    (
                        ObjectId::from(
                            "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                        ),
                        Uint128::one(),
                    ),
                    (
                        ObjectId::from(
                            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                        ),
                        Uint128::new(2),
                    ),
                ],
            },
            TestUnpinCase {
                // Object not exists
                pin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                pin_senders: vec![mock_info("bob", &[])],
                unpin: vec![ObjectId::from("NOTFOUND")],
                unpin_senders: vec![mock_info("martin", &[])],
                expected_count: 1,
                expected_error: Some(ContractError::Std(StdError::not_found(
                    type_name::<Object>(),
                ))),
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::one(),
                )],
            },
        ];

        for case in cases {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &[]);

            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg {
                    bucket: "test".to_string(),
                    limits: BucketLimits::new(),
                    pagination: PaginationConfig::new(),
                },
            )
            .unwrap();

            let data = general_purpose::STANDARD.encode("okp4");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("data");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("hello");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let mut last_result: Option<Result<Response, ContractError>> = None;
            case.pin
                .iter()
                .zip(case.pin_senders)
                .for_each(|(object_id, info)| {
                    last_result = Some(execute(
                        deps.as_mut(),
                        mock_env(),
                        info,
                        ExecuteMsg::PinObject {
                            id: object_id.clone(),
                        },
                    ));
                });
            case.unpin
                .iter()
                .zip(case.unpin_senders)
                .for_each(|(object_id, info)| {
                    last_result = Some(execute(
                        deps.as_mut(),
                        mock_env(),
                        info,
                        ExecuteMsg::UnpinObject {
                            id: object_id.clone(),
                        },
                    ));
                });

            match case.expected_error {
                Some(err) => assert_eq!(last_result.unwrap().unwrap_err(), err),
                _ => {
                    assert_eq!(
                        pins()
                            .keys_raw(&deps.storage, None, None, Order::Ascending)
                            .count(),
                        case.expected_count
                    );
                    for (object_id, count) in case.expected_object_pin_count {
                        assert_eq!(
                            objects().load(&deps.storage, object_id).unwrap().pin_count,
                            count
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn fetch_objects() {
        let mut deps = mock_dependencies();
        let info1 = mock_info("creator1", &[]);
        let info2 = mock_info("creator2", &[]);

        let msg = InstantiateMsg {
            bucket: String::from("test"),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
        };
        instantiate(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();

        let msg = QueryMsg::Objects {
            address: None,
            first: None,
            after: None,
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectsResponse = from_binary(&result).unwrap();
        assert_eq!(response.data.len(), 0);
        assert_eq!(
            response.page_info,
            PageInfo {
                has_next_page: false,
                cursor: "".to_string()
            }
        );

        let data = general_purpose::STANDARD.encode("object1");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
        };
        execute(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();
        let data = general_purpose::STANDARD.encode("object2");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
        };
        execute(deps.as_mut(), mock_env(), info1, msg).unwrap();
        let data = general_purpose::STANDARD.encode("object3");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
        };
        execute(deps.as_mut(), mock_env(), info2, msg).unwrap();

        let cases = vec![
            (QueryMsg::Objects {address: None,first: None,after: None}, 3, PageInfo {has_next_page: false,cursor: "2wvnkrvqBwQPX2Zougwd2BQufN4tbUGQfzajMyhNXnnPheaiP6HmCQw9JH4MvtxLzJuqpm6h2rJYPXHE1kCnDXS5".to_string()}),
            (QueryMsg::Objects {address: Some("unknown".to_string()), first: None, after: None}, 0, PageInfo {has_next_page: false, cursor: "".to_string()}),
            (QueryMsg::Objects {address: Some("creator1".to_string()),first: None,after: None}, 2, PageInfo {has_next_page: false,cursor: "2wvnkrvqBwQPX2Zougwd2BQufN4tbUGQfzajMyhNXnnPheaiP6HmCQw9JH4MvtxLzJuqpm6h2rJYPXHE1kCnDXS5".to_string()}),
            (QueryMsg::Objects {address: Some("creator1".to_string()),first: Some(1),after: None}, 1, PageInfo {has_next_page: true,cursor: "23Y64LH99dTheD49F6F7PvqH4J8wBm1dtd5mXsrYJfSvR8x4L214YUQ2xv1PY7uxqGKVSs4QxDsWF3qCo6QGzWWS".to_string()}),
            (QueryMsg::Objects {address: Some("creator1".to_string()),first: Some(1),after: Some("23Y64LH99dTheD49F6F7PvqH4J8wBm1dtd5mXsrYJfSvR8x4L214YUQ2xv1PY7uxqGKVSs4QxDsWF3qCo6QGzWWS".to_string())}, 1, PageInfo {has_next_page: false,cursor: "2wvnkrvqBwQPX2Zougwd2BQufN4tbUGQfzajMyhNXnnPheaiP6HmCQw9JH4MvtxLzJuqpm6h2rJYPXHE1kCnDXS5".to_string()}),
        ];

        for case in cases {
            let msg = case.0;
            let result = query(deps.as_ref(), mock_env(), msg).unwrap();
            let response: ObjectsResponse = from_binary(&result).unwrap();
            assert_eq!(response.data.len(), case.1);
            assert_eq!(response.page_info, case.2);
        }

        let msg = QueryMsg::Objects {
            address: Some("creator2".to_string()),
            first: None,
            after: None,
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectsResponse = from_binary(&result).unwrap();
        assert_eq!(
            response.data.first().unwrap(),
            &ObjectResponse {
                id: "0a6d95579ba3dd2f79c870906fd894007ce449020d111d358894cfbbcd9a03a4".to_string(),
                owner: "creator2".to_string(),
                is_pinned: false,
                size: 7u128.into()
            }
        );
    }

    #[test]
    fn object_pins() {
        let mut deps = mock_dependencies();
        let info1 = mock_info("creator1", &[]);
        let info2 = mock_info("creator2", &[]);

        let msg = InstantiateMsg {
            bucket: String::from("test"),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
        };
        instantiate(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();

        let data = general_purpose::STANDARD.encode("object1");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
        };
        execute(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();
        // 1: 445008b7f2932922bdb184771d9978516a4f89d77000c2d6eab18b0894aac3a7
        let data = general_purpose::STANDARD.encode("object2");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: true,
        };
        execute(deps.as_mut(), mock_env(), info2, msg).unwrap();
        // 2: abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56
        let msg = ExecuteMsg::PinObject {
            id: "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56".to_string(),
        };
        execute(deps.as_mut(), mock_env(), info1, msg).unwrap();

        let cases = vec![
            (
                QueryMsg::ObjectPins {
                    id: "445008b7f2932922bdb184771d9978516a4f89d77000c2d6eab18b0894aac3a7"
                        .to_string(),
                    first: None,
                    after: None,
                },
                Vec::<String>::new(),
                PageInfo {
                    has_next_page: false,
                    cursor: "".to_string(),
                },
            ),
            (
                QueryMsg::ObjectPins {
                    id: "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56"
                        .to_string(),
                    first: None,
                    after: None,
                },
                vec!["creator1".to_string(), "creator2".to_string()],
                PageInfo {
                    has_next_page: false,
                    cursor: "Hdm2eF21ryF".to_string(),
                },
            ),
            (
                QueryMsg::ObjectPins {
                    id: "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56"
                        .to_string(),
                    first: Some(1),
                    after: None,
                },
                vec!["creator1".to_string()],
                PageInfo {
                    has_next_page: true,
                    cursor: "Hdm2eF21ryE".to_string(),
                },
            ),
            (
                QueryMsg::ObjectPins {
                    id: "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56"
                        .to_string(),
                    first: Some(1),
                    after: Some("Hdm2eF21ryE".to_string()),
                },
                vec!["creator2".to_string()],
                PageInfo {
                    has_next_page: false,
                    cursor: "Hdm2eF21ryF".to_string(),
                },
            ),
        ];

        for case in cases {
            let result = query(deps.as_ref(), mock_env(), case.0).unwrap();
            let response: ObjectPinsResponse = from_binary(&result).unwrap();
            assert_eq!(response.data, case.1);
            assert_eq!(response.page_info, case.2);
        }
    }

    #[test]
    fn object_pins_non_existing() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: String::from("test"),
            limits: BucketLimits::new(),
            pagination: PaginationConfig::new(),
        };
        instantiate(deps.as_mut(), mock_env(), mock_info("creator1", &[]), msg).unwrap();

        match query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::ObjectPins {
                id: "unknown".to_string(),
                after: None,
                first: None,
            },
        )
        .err()
        .unwrap()
        {
            NotFound { .. } => (),
            _ => panic!("assertion failed"),
        }
    }
}
