use crate::error::BucketError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::crypto;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ObjectId, QueryMsg};
use crate::state;
use crate::state::{objects, pins, Bucket, Object, Pin, BUCKET, DATA};

// version info for migration info
const CONTRACT_NAME: &str = concat!("crates.io:", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let bucket = Bucket::try_new(
        info.sender,
        msg.bucket,
        msg.config.into(),
        msg.limits.into(),
        msg.pagination.try_into()?,
    )?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    BUCKET.save(deps.storage, &bucket)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StoreObject {
            data,
            pin,
            compression_algorithm,
        } => execute::store_object(deps, info, data, pin, compression_algorithm),
        ExecuteMsg::PinObject { id } => execute::pin_object(deps, info, id),
        ExecuteMsg::UnpinObject { id } => execute::unpin_object(deps, info, id),
        ExecuteMsg::ForgetObject { id } => execute::forget_object(deps, info, id),
    }
}

pub mod execute {
    use super::*;
    use crate::compress::CompressionAlgorithm;
    use crate::crypto::Hash;
    use crate::msg;
    use crate::state::BucketLimits;
    use crate::ContractError::ObjectPinned;
    use cosmwasm_std::{Order, Uint128};

    pub fn store_object(
        deps: DepsMut<'_>,
        info: MessageInfo,
        data: Binary,
        pin: bool,
        compression_algorithm: Option<msg::CompressionAlgorithm>,
    ) -> Result<Response, ContractError> {
        let size = (data.len() as u128).into();
        let bucket = BUCKET.load(deps.storage)?;
        let compressions = &bucket.config.accepted_compression_algorithms;
        let compression: CompressionAlgorithm = compression_algorithm
            .map(Into::into)
            .or_else(|| compressions.first().cloned())
            .unwrap_or(CompressionAlgorithm::Passthrough);

        // pre-conditions
        if let Some(limit) = bucket.limits.max_object_size {
            if size > limit {
                return Err(BucketError::MaxObjectSizeLimitExceeded(size, limit).into());
            }
        }
        if let Some(limit) = bucket.limits.max_objects {
            let value = bucket.stat.object_count + Uint128::one();
            if value > limit {
                return Err(BucketError::MaxObjectsLimitExceeded(value, limit).into());
            }
        }
        if let Some(limit) = bucket.limits.max_object_pins {
            if pin && limit.is_zero() {
                return Err(BucketError::MaxObjectPinsLimitExceeded(Uint128::one(), limit).into());
            }
        }
        if let Some(limit) = bucket.limits.max_total_size {
            let value = bucket.stat.size + size;
            if value > limit {
                return Err(BucketError::MaxTotalSizeLimitExceeded(value, limit).into());
            }
        }
        if !compressions.contains(&compression) {
            return Err(BucketError::CompressionAlgorithmNotAccepted(
                compression.into(),
                bucket
                    .config
                    .accepted_compression_algorithms
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            )
            .into());
        }

        // store object data
        let id = crypto::hash(&bucket.config.hash_algorithm.into(), &data.0);
        let data_path = DATA.key(id.clone());

        if data_path.has(deps.storage) {
            return Err(ContractError::Bucket(BucketError::ObjectAlreadyStored));
        }
        let compressed_data = compression.compress(&data.0)?;

        data_path.save(deps.storage, &compressed_data)?;

        // store object
        let compressed_size = (compressed_data.len() as u128).into();
        let object = &Object {
            id,
            owner: info.sender.clone(),
            size,
            pin_count: if pin { Uint128::one() } else { Uint128::zero() },
            compression,
            compressed_size,
        };

        objects().save(deps.storage, object.id.clone(), object)?;

        // save bucket stats
        BUCKET.update(deps.storage, |mut bucket| -> Result<_, ContractError> {
            let stat = &mut bucket.stat;
            stat.size += size;
            stat.object_count += Uint128::one();
            stat.compressed_size += compressed_size;
            Ok(bucket)
        })?;

        // save pin
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

        Ok(Response::new()
            .add_attribute("action", "store_object")
            .add_attribute("id", object.id.clone()))
    }

    pub fn pin_object(
        deps: DepsMut<'_>,
        info: MessageInfo,
        object_id: ObjectId,
    ) -> Result<Response, ContractError> {
        let res = Response::new()
            .add_attribute("action", "pin_object")
            .add_attribute("id", object_id.clone());

        let id: Hash = object_id.try_into()?;
        if pins().has(deps.storage, (id.clone(), info.sender.clone())) {
            return Ok(res);
        }

        let object = objects().load(deps.storage, id.clone())?;
        let mut updated_object = object.clone();
        updated_object.pin_count += Uint128::one();

        objects().replace(
            deps.storage,
            id.clone(),
            Some(&updated_object),
            Some(&object),
        )?;

        let bucket = BUCKET.load(deps.storage)?;

        match bucket.limits {
            BucketLimits {
                max_object_pins: Some(max),
                ..
            } if max < updated_object.pin_count => {
                Err(BucketError::MaxObjectPinsLimitExceeded(updated_object.pin_count, max).into())
            }
            _ => {
                pins().save(
                    deps.storage,
                    (id.clone(), info.sender.clone()),
                    &Pin {
                        id,
                        address: info.sender,
                    },
                )?;
                Ok(res)
            }
        }
    }

    pub fn unpin_object(
        deps: DepsMut<'_>,
        info: MessageInfo,
        object_id: ObjectId,
    ) -> Result<Response, ContractError> {
        let id: Hash = object_id.clone().try_into()?;
        let object_path = objects().key(id.clone());
        let mut object = object_path.load(deps.storage)?;

        let res = Response::new()
            .add_attribute("action", "unpin_object")
            .add_attribute("id", object_id);

        if !pins().has(deps.storage, (id.clone(), info.sender.clone())) {
            return Ok(res);
        }

        object.pin_count -= Uint128::one();
        object_path.save(deps.storage, &object)?;

        pins().remove(deps.storage, (id, info.sender))?;

        Ok(res)
    }

    pub fn forget_object(
        deps: DepsMut<'_>,
        info: MessageInfo,
        object_id: ObjectId,
    ) -> Result<Response, ContractError> {
        let id: Hash = object_id.clone().try_into()?;
        if pins().has(deps.storage, (id.clone(), info.sender.clone())) {
            pins().remove(deps.storage, (id.clone(), info.sender))?;
        }

        if pins()
            .idx
            .object
            .prefix(id.clone())
            .keys_raw(deps.storage, None, None, Order::Ascending)
            .next()
            .is_some()
        {
            return Err(ObjectPinned {});
        }
        let object = query::object(deps.as_ref(), object_id.clone())?;
        BUCKET.update(deps.storage, |mut b| -> Result<_, ContractError> {
            b.stat.object_count -= Uint128::one();
            b.stat.size -= object.size;
            Ok(b)
        })?;

        objects().remove(deps.storage, id.clone())?;
        DATA.remove(deps.storage, id);

        Ok(Response::new()
            .add_attribute("action", "forget_object")
            .add_attribute("id", object_id))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<'_>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    Ok(match msg {
        QueryMsg::Bucket {} => to_json_binary(&query::bucket(deps)?),
        QueryMsg::Object { id } => to_json_binary(&query::object(deps, id)?),
        QueryMsg::ObjectData { id } => to_json_binary(&query::data(deps, id)?),
        QueryMsg::Objects {
            address,
            after,
            first,
        } => to_json_binary(&query::fetch_objects(deps, address, after, first)?),
        QueryMsg::ObjectPins { id, after, first } => {
            to_json_binary(&query::object_pins(deps, id, after, first)?)
        }
    }?)
}

pub mod query {
    use super::*;
    use crate::crypto::Hash;
    use crate::cursor;
    use crate::msg::{
        BucketResponse, Cursor, ObjectPinsResponse, ObjectResponse, ObjectsResponse, PageInfo,
    };
    use crate::pagination::{PaginationHandler, QueryPage};
    use cosmwasm_std::{Addr, Order};

    pub fn bucket(deps: Deps<'_>) -> Result<BucketResponse, ContractError> {
        let bucket = BUCKET.load(deps.storage)?;

        Ok(BucketResponse {
            name: bucket.name,
            config: bucket.config.into(),
            limits: bucket.limits.into(),
            pagination: bucket.pagination.into(),
        })
    }

    pub fn object(deps: Deps<'_>, object_id: ObjectId) -> Result<ObjectResponse, ContractError> {
        let id: Hash = object_id.try_into()?;
        let object = objects().load(deps.storage, id)?;
        Ok((&object).into())
    }

    pub fn data(deps: Deps<'_>, object_id: ObjectId) -> Result<Binary, ContractError> {
        let id: Hash = object_id.try_into()?;
        let compression = objects().load(deps.storage, id.clone())?.compression;
        let data = DATA.load(deps.storage, id)?;
        let decompressed_data = compression.decompress(&data)?;
        Ok(Binary::from(decompressed_data))
    }

    pub fn fetch_objects(
        deps: Deps<'_>,
        address: Option<String>,
        after: Option<Cursor>,
        first: Option<u32>,
    ) -> StdResult<ObjectsResponse> {
        let address = match address {
            Some(raw) => Some(deps.api.addr_validate(&raw)?),
            _ => None,
        };

        let handler: PaginationHandler<'_, Object, Hash> =
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
            after,
            first,
        )?;

        Ok(ObjectsResponse {
            data: page.0.iter().map(Into::into).collect(),
            page_info: page.1,
        })
    }

    pub fn object_pins(
        deps: Deps<'_>,
        object_id: ObjectId,
        after: Option<Cursor>,
        first: Option<u32>,
    ) -> StdResult<ObjectPinsResponse> {
        let id: Hash = object_id.try_into()?;
        objects().load(deps.storage, id.clone())?;

        let handler: PaginationHandler<'_, Pin, (Hash, Addr)> =
            PaginationHandler::from(BUCKET.load(deps.storage)?.pagination);

        let page: (Vec<Pin>, PageInfo) = handler.query_page_cursor_fn(
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

impl From<state::HashAlgorithm> for crypto::HashAlgorithm {
    fn from(algorithm: state::HashAlgorithm) -> Self {
        match algorithm {
            state::HashAlgorithm::MD5 => crypto::HashAlgorithm::MD5,
            state::HashAlgorithm::Sha224 => crypto::HashAlgorithm::Sha224,
            state::HashAlgorithm::Sha256 => crypto::HashAlgorithm::Sha256,
            state::HashAlgorithm::Sha384 => crypto::HashAlgorithm::Sha384,
            state::HashAlgorithm::Sha512 => crypto::HashAlgorithm::Sha512,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::BucketError;
    use crate::msg::{
        BucketConfig, BucketLimitsBuilder, BucketResponse, CompressionAlgorithm, HashAlgorithm,
        ObjectPinsResponse, ObjectResponse, ObjectsResponse, PageInfo, PaginationConfigBuilder,
    };
    use base64::{engine::general_purpose, Engine as _};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::StdError::NotFound;
    use cosmwasm_std::{from_json, Attribute, Order, StdError, Uint128};
    use std::any::type_name;

    fn decode_hex(hex: &str) -> Vec<u8> {
        base16ct::lower::decode_vec(hex).unwrap()
    }

    fn with_namespace(key: &[u8]) -> Vec<u8> {
        let namespace = decode_hex("00064f424a454354");
        let mut v = Vec::with_capacity(namespace.len() + key.len());
        v.extend(namespace);
        v.extend_from_slice(key);
        v
    }

    fn not_found_object_info<T>(hex: &str) -> String {
        let type_name = type_name::<T>();
        let key = with_namespace(&decode_hex(hex));
        format!("type: {type_name}; key: {:02X?}", key)
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "foo".to_string(),
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_json(&res).unwrap();
        assert_eq!("foo", value.name);
        assert_eq!(value.config, Default::default());
        assert_eq!(value.limits, Default::default());
        assert_eq!(value.pagination.max_page_size, 30);
        assert_eq!(value.pagination.default_page_size, 10);

        // check internal state too
        let bucket = BUCKET.load(&deps.storage).unwrap();
        assert_eq!("creator", bucket.owner.into_string());
        assert_eq!(Uint128::zero(), bucket.stat.size);
        assert_eq!(Uint128::zero(), bucket.stat.object_count);
    }

    #[test]
    fn proper_config_initialization() {
        let mut deps = mock_dependencies();

        // Define the test cases
        let test_cases = vec![
            (HashAlgorithm::MD5, HashAlgorithm::MD5),
            (HashAlgorithm::Sha224, HashAlgorithm::Sha224),
            (HashAlgorithm::Sha256, HashAlgorithm::Sha256),
            (HashAlgorithm::Sha384, HashAlgorithm::Sha384),
            (HashAlgorithm::Sha512, HashAlgorithm::Sha512),
        ];

        for (hash_algorithm, expected_hash_algorithm) in test_cases {
            let msg = InstantiateMsg {
                bucket: "bar".to_string(),
                config: BucketConfig {
                    hash_algorithm,
                    ..Default::default()
                },
                limits: Default::default(),
                pagination: Default::default(),
            };
            let info = mock_info("creator", &[]);

            let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

            let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
            let value: BucketResponse = from_json(&res).unwrap();

            assert_eq!("bar", value.name);
            assert_eq!(value.config.hash_algorithm, expected_hash_algorithm);
        }
    }

    #[test]
    fn proper_limits_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: "bar".to_string(),
            config: Default::default(),
            limits: BucketLimitsBuilder::default()
                .max_total_size(Uint128::new(20000))
                .max_objects(Uint128::new(10))
                .max_object_size(Uint128::new(2000))
                .max_object_pins(Uint128::new(1))
                .build()
                .unwrap(),
            pagination: PaginationConfigBuilder::default()
                .max_page_size(50)
                .default_page_size(30)
                .build()
                .unwrap(),
        };
        let info = mock_info("creator", &[]);

        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_json(&res).unwrap();
        assert_eq!("bar", value.name);
        assert_eq!(Uint128::new(20000), value.limits.max_total_size.unwrap());
        assert_eq!(Uint128::new(10), value.limits.max_objects.unwrap());
        assert_eq!(Uint128::new(2000), value.limits.max_object_size.unwrap());
        assert_eq!(Uint128::new(1), value.limits.max_object_pins.unwrap());
        assert_eq!(value.pagination.max_page_size, 50);
        assert_eq!(value.pagination.default_page_size, 30);
    }

    #[test]
    fn proper_pagination_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            bucket: "bar".to_string(),
            config: Default::default(),
            limits: Default::default(),
            pagination: PaginationConfigBuilder::default()
                .max_page_size(50)
                .default_page_size(30)
                .build()
                .unwrap(),
        };
        instantiate(deps.as_mut(), mock_env(), mock_info("creator", &[]), msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_json(&res).unwrap();
        assert_eq!(value.pagination.max_page_size, 50);
        assert_eq!(value.pagination.default_page_size, 30);
    }

    #[test]
    fn invalid_pagination_initialization() {
        let cases = vec![
            (
                PaginationConfigBuilder::default()
                    .max_page_size(u32::MAX)
                    .build()
                    .unwrap(),
                StdError::generic_err("'max_page_size' cannot exceed 'u32::MAX - 1'"),
            ),
            (
                PaginationConfigBuilder::default()
                    .default_page_size(31)
                    .build()
                    .unwrap(),
                StdError::generic_err("'default_page_size' cannot exceed 'max_page_size'"),
            ),
        ];
        for case in cases {
            let mut deps = mock_dependencies();
            let msg = InstantiateMsg {
                bucket: "bar".to_string(),
                config: Default::default(),
                limits: Default::default(),
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
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
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
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Bucket {}).unwrap();
        let value: BucketResponse = from_json(&res).unwrap();
        assert_eq!("foobar", value.name);
    }

    #[test]
    fn store_object_without_limits() {
        let obj1_content = &general_purpose::STANDARD.encode("hello");
        let obj2_content = &general_purpose::STANDARD.encode("okp4");

        let test_cases = vec![
            (
                HashAlgorithm::MD5,
                vec![
                    (
                        obj1_content,
                        true,
                        "5d41402abc4b2a76b9719d911017c592"
                            .to_string(),
                        5,
                    ),
                    (
                        obj2_content,
                        false,
                        "33f41d49353ad1a876e36918f64eac4d"
                            .to_string(),
                        4,
                    ),
                ],
            ),
            (
                HashAlgorithm::Sha224,
                vec![
                    (
                        obj1_content,
                        true,
                        "ea09ae9cc6768c50fcee903ed054556e5bfc8347907f12598aa24193"
                            .to_string(),
                        5,
                    ),
                    (
                        obj2_content,
                        false,
                        "fe798aa30e560c57d69c46982b2bb1320dc86813730bb7c6406ce84b"
                            .to_string(),
                        4,
                    ),
                ],
            ),
            (
                HashAlgorithm::Sha256,
                vec![
                    (
                        obj1_content,
                        true,
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                            .to_string(),
                        5,
                    ),
                    (
                        obj2_content,
                        false,
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6"
                            .to_string(),
                        4,
                    ),
                ],
            ),
            (
                HashAlgorithm::Sha384,
                vec![
                    (
                        obj1_content,
                        true,
                        "59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f"
                            .to_string(),
                        5,
                    ),
                    (
                        obj2_content,
                        false,
                        "e700b122a81f64ce34ab67c6a815987536a05b0590bbeb32cf5e88963edd8c6e69c9e43b0f957f242d984f09f91bcaf2"
                            .to_string(),
                        4,
                    ),
                ],
            ),
            (
                HashAlgorithm::Sha512,
                vec![
                    (
                        obj1_content,
                        true,
                        "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043"
                            .to_string(),
                        5,
                    ),
                    (
                        obj2_content,
                        false,
                        "e4f4025e1e28abb473c89bcae03ded972e91b4427e8970be87f645cc34b9b203d633c12760e32c97011439640cba159f60992e10aac8023fa2577cadc1be3b55"
                            .to_string(),
                        4,
                    ),
                ],
            ),
        ];

        for (hash_algorithm, objs) in test_cases {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &[]);

            instantiate(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                InstantiateMsg {
                    bucket: "test".to_string(),
                    config: BucketConfig {
                        hash_algorithm,
                        ..Default::default()
                    },
                    limits: Default::default(),
                    pagination: Default::default(),
                },
            )
            .unwrap();

            for (content, pin, expected_hash, expected_size) in &objs {
                let msg = ExecuteMsg::StoreObject {
                    data: Binary::from_base64(content).unwrap(),
                    pin: *pin,
                    compression_algorithm: Some(CompressionAlgorithm::Passthrough),
                };
                let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
                assert_eq!(
                    res.attributes,
                    vec![
                        Attribute::new("action", "store_object"),
                        Attribute::new("id", expected_hash.clone()),
                    ]
                );

                assert_eq!(
                    Binary::from_base64(content).unwrap(),
                    Binary::from(
                        DATA.load(&deps.storage, decode_hex(&expected_hash).into())
                            .unwrap()
                    ),
                );

                let created = objects()
                    .load(&deps.storage, decode_hex(&expected_hash).into())
                    .unwrap();
                assert_eq!(created.id, decode_hex(&expected_hash).into());
                assert_eq!(created.owner, info.sender.clone());
                assert_eq!(created.size.u128(), *expected_size);
                assert_eq!(
                    created.pin_count,
                    if *pin {
                        Uint128::one()
                    } else {
                        Uint128::zero()
                    }
                );

                assert_eq!(
                    pins().has(
                        &deps.storage,
                        (decode_hex(&expected_hash).into(), info.clone().sender),
                    ),
                    *pin,
                );
            }

            let bucket = BUCKET.load(&deps.storage).unwrap();
            assert_eq!(
                bucket.stat.size.u128(),
                objs.iter().map(|x| x.3).sum::<u128>()
            );
            assert_eq!(
                bucket.stat.object_count.u128(),
                u128::try_from(objs.len()).unwrap()
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
    }

    #[test]
    fn store_object_already_stored() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {
            bucket: String::from("test"),
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let object = general_purpose::STANDARD.encode("already existing object");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(object.as_str()).unwrap(),
            pin: true,
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
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
            (
                BucketLimitsBuilder::default()
                    .max_objects(2u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                BucketLimitsBuilder::default()
                    .max_object_size(5u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                BucketLimitsBuilder::default()
                    .max_total_size(9u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                BucketLimitsBuilder::default()
                    .max_object_pins(1u128)
                    .build()
                    .unwrap(),
                None,
            ),
            (
                BucketLimitsBuilder::default()
                    .max_objects(1u128)
                    .build()
                    .unwrap(),
                Some(ContractError::Bucket(BucketError::MaxObjectsLimitExceeded(
                    2u128.into(),
                    1u128.into(),
                ))),
            ),
            (
                BucketLimitsBuilder::default()
                    .max_object_size(4u128)
                    .build()
                    .unwrap(),
                Some(ContractError::Bucket(
                    BucketError::MaxObjectSizeLimitExceeded(5u128.into(), 4u128.into()),
                )),
            ),
            (
                BucketLimitsBuilder::default()
                    .max_total_size(8u128)
                    .build()
                    .unwrap(),
                Some(ContractError::Bucket(
                    BucketError::MaxTotalSizeLimitExceeded(9u128.into(), 8u128.into()),
                )),
            ),
            (
                BucketLimitsBuilder::default()
                    .max_object_pins(0u128)
                    .build()
                    .unwrap(),
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
                config: Default::default(),
                limits: case.0,
                pagination: Default::default(),
            };
            instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(obj1.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(obj2.as_str()).unwrap(),
                pin: true,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

            assert_eq!(res.err(), case.1);
        }
    }

    #[test]
    fn store_object_compressed() {
        use either::Either;

        struct ExpectedCompressionResult {
            compression_algorithm: CompressionAlgorithm,
            compressed_size: u128,
        }
        struct TC {
            accepted_compression_algorithms: Vec<CompressionAlgorithm>,
            compression_algorithm: Option<CompressionAlgorithm>,
            expected_result: Either<ContractError, ExpectedCompressionResult>,
        }

        let cases: Vec<TC> = vec![
            TC {
                accepted_compression_algorithms: vec![
                    CompressionAlgorithm::Passthrough,
                    CompressionAlgorithm::Snappy,
                    CompressionAlgorithm::Lzma,
                ],
                compression_algorithm: None,
                expected_result: Either::Right(ExpectedCompressionResult {
                    compression_algorithm: CompressionAlgorithm::Passthrough,
                    compressed_size: 466,
                }),
            },
            TC {
                accepted_compression_algorithms: vec![
                    CompressionAlgorithm::Passthrough,
                    CompressionAlgorithm::Snappy,
                    CompressionAlgorithm::Lzma,
                ],
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
                expected_result: Either::Right(ExpectedCompressionResult {
                    compression_algorithm: CompressionAlgorithm::Passthrough,
                    compressed_size: 466,
                }),
            },
            TC {
                accepted_compression_algorithms: vec![
                    CompressionAlgorithm::Passthrough,
                    CompressionAlgorithm::Snappy,
                    CompressionAlgorithm::Lzma,
                ],
                compression_algorithm: Some(CompressionAlgorithm::Snappy),
                expected_result: Either::Right(ExpectedCompressionResult {
                    compression_algorithm: CompressionAlgorithm::Snappy,
                    compressed_size: 414,
                }),
            },
            TC {
                accepted_compression_algorithms: vec![
                    CompressionAlgorithm::Passthrough,
                    CompressionAlgorithm::Snappy,
                    CompressionAlgorithm::Lzma,
                ],
                compression_algorithm: Some(CompressionAlgorithm::Lzma),
                expected_result: Either::Right(ExpectedCompressionResult {
                    compression_algorithm: CompressionAlgorithm::Lzma,
                    compressed_size: 344,
                }),
            },
            TC {
                accepted_compression_algorithms: vec![CompressionAlgorithm::Passthrough],
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
                expected_result: Either::Right(ExpectedCompressionResult {
                    compression_algorithm: CompressionAlgorithm::Passthrough,
                    compressed_size: 466,
                }),
            },
            TC {
                accepted_compression_algorithms: vec![CompressionAlgorithm::Snappy],
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
                expected_result: Either::Left(ContractError::Bucket(
                    BucketError::CompressionAlgorithmNotAccepted(
                        CompressionAlgorithm::Passthrough,
                        vec![CompressionAlgorithm::Snappy],
                    ),
                )),
            },
        ];
        let data ="In a magical land,  there were many realms, one of which was known as OKP4. Within \
            this realm, druid programmers possessed the power to create smart contracts. As the kingdom \
            grew, the druids used their skills to power decentralized systems, bringing prosperity and \
            wonder to all who sought their expertise. And so, the legend of the druid programmers and \
            their magical smart contracts lived on, inspiring future generations to unlock the power of \
            the digital realm.";
        let obj = general_purpose::STANDARD.encode(data);
        let obj_id = "25056da0c504e6beb9d8666f9e5919a4a02689f4bceeb4698a21c651f07d8e04";

        for case in cases {
            // Arrange
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &[]);
            let msg = InstantiateMsg {
                bucket: String::from("test"),
                config: BucketConfig {
                    accepted_compression_algorithms: case.accepted_compression_algorithms,
                    ..Default::default()
                },
                limits: Default::default(),
                pagination: Default::default(),
            };
            instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            // Act
            let res = execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                ExecuteMsg::StoreObject {
                    data: Binary::from_base64(obj.as_str()).unwrap(),
                    pin: false,
                    compression_algorithm: case.compression_algorithm,
                },
            );

            // Assert
            match case.expected_result {
                Either::Left(err) => assert_eq!(res.err(), Some(err)),
                Either::Right(expected) => {
                    let _to_assert_if_we_want = res.unwrap();
                    let res_object_info = query::object(deps.as_ref(), obj_id.to_string()).unwrap();
                    let res_object_data = query::data(deps.as_ref(), obj_id.to_string()).unwrap();

                    assert_eq!(
                        res_object_info,
                        ObjectResponse {
                            id: obj_id.to_string(),
                            owner: "creator".to_string(),
                            is_pinned: false,
                            size: Uint128::from(data.len() as u128),
                            compressed_size: expected.compressed_size.into(),
                            compression_algorithm: expected.compression_algorithm,
                        }
                    );
                    assert_eq!(res_object_data, data.as_bytes().to_vec());
                }
            }
        }
    }

    #[test]
    fn object() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);

        let msg = InstantiateMsg {
            bucket: String::from("test"),
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
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
            ContractError::Std(NotFound { .. }) => (),
            _ => panic!("assertion failed"),
        }

        let data = general_purpose::STANDARD.encode("hello");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: true,
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::Object {
            id: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".to_string(),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectResponse = from_json(&result).unwrap();
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
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
        };
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::Object {
            id: "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6".to_string(),
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectResponse = from_json(&result).unwrap();
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
        struct TC {
            compression_algorithm: Option<CompressionAlgorithm>,
        }

        let cases = vec![
            TC {
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            },
            TC {
                compression_algorithm: Some(CompressionAlgorithm::Snappy),
            },
            TC {
                compression_algorithm: Some(CompressionAlgorithm::Lzma),
            },
        ];

        for case in cases {
            let mut deps = mock_dependencies();
            let info = mock_info("creator", &[]);
            let data =
                Binary::from_base64(general_purpose::STANDARD.encode("okp4").as_str()).unwrap();

            let msg = InstantiateMsg {
                bucket: String::from("test"),
                config: Default::default(),
                limits: Default::default(),
                pagination: Default::default(),
            };
            instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            match query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::ObjectData {
                    id: "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6"
                        .to_string(),
                },
            )
            .err()
            .unwrap()
            {
                ContractError::Std(NotFound { .. }) => (),
                _ => panic!("assertion failed"),
            }

            let msg = ExecuteMsg::StoreObject {
                data: data.clone(),
                pin: false,
                compression_algorithm: case.compression_algorithm,
            };
            execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let msg = QueryMsg::ObjectData {
                id: "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6".to_string(),
            };
            let result = query(deps.as_ref(), mock_env(), msg).unwrap();
            assert_eq!(result, to_json_binary(&data).unwrap());
        }
    }

    #[test]
    fn pin_object() {
        struct TC {
            objects: Vec<ObjectId>,
            senders: Vec<MessageInfo>,
            expected_count: usize,
            expected_error: Option<ContractError>,
            expected_object_pin_count: Vec<(ObjectId, Uint128)>,
        }

        let cases = vec![
            TC {
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
            TC {
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
            TC {
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
            TC {
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
            TC {
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
            TC {
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
            TC {
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
                expected_error: Some(ContractError::Bucket(
                    BucketError::MaxObjectPinsLimitExceeded(Uint128::new(3), Uint128::new(2)),
                )),
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
            TC {
                // Object not exists
                objects: vec![ObjectId::from(
                    "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56",
                )],
                senders: vec![mock_info("bob", &[])],
                expected_count: 0,
                expected_error: Some(ContractError::Std(StdError::not_found(
                    not_found_object_info::<Object>(
                        "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56",
                    ),
                ))),
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::zero(),
                )],
            },
            TC {
                // Invalid object id
                objects: vec![ObjectId::from("invalid id")],
                senders: vec![mock_info("bob", &[])],
                expected_count: 0,
                expected_error: Some(ContractError::Std(StdError::parse_err(
                    type_name::<Vec<u8>>(),
                    "invalid Base16 encoding".to_string(),
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
                    config: Default::default(),
                    limits: BucketLimitsBuilder::default()
                        .max_object_pins(Uint128::new(2))
                        .build()
                        .unwrap(),
                    pagination: Default::default(),
                },
            )
            .unwrap();

            let data = general_purpose::STANDARD.encode("okp4");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("data");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("hello");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
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
                            objects()
                                .load(&deps.storage, decode_hex(&object_id).into())
                                .unwrap()
                                .pin_count,
                            count
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn unpin_object() {
        struct TC {
            pin: Vec<ObjectId>,
            pin_senders: Vec<MessageInfo>,
            unpin: Vec<ObjectId>,
            unpin_senders: Vec<MessageInfo>,
            expected_count: usize,
            expected_error: Option<ContractError>,
            expected_object_pin_count: Vec<(ObjectId, Uint128)>,
        }

        let cases = vec![
            TC {
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
            TC {
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
            TC {
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
            TC {
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
            TC {
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
            TC {
                // Object not exists
                pin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                pin_senders: vec![mock_info("bob", &[])],
                unpin: vec![ObjectId::from(
                    "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56",
                )],
                unpin_senders: vec![mock_info("martin", &[])],
                expected_count: 1,
                expected_error: Some(ContractError::Std(StdError::not_found(
                    not_found_object_info::<Object>(
                        "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56",
                    ),
                ))),
                expected_object_pin_count: vec![(
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    Uint128::one(),
                )],
            },
            TC {
                // Invalid object id
                pin: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                pin_senders: vec![mock_info("bob", &[])],
                unpin: vec![ObjectId::from("invalid id")],
                unpin_senders: vec![mock_info("martin", &[])],
                expected_count: 1,
                expected_error: Some(ContractError::Std(StdError::parse_err(
                    type_name::<Vec<u8>>(),
                    "invalid Base16 encoding".to_string(),
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
                    config: Default::default(),
                    limits: Default::default(),
                    pagination: Default::default(),
                },
            )
            .unwrap();

            let data = general_purpose::STANDARD.encode("okp4");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("data");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("hello");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
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
                            objects()
                                .load(&deps.storage, decode_hex(&object_id).into())
                                .unwrap()
                                .pin_count,
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
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
        };
        instantiate(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();

        let msg = QueryMsg::Objects {
            address: None,
            first: None,
            after: None,
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectsResponse = from_json(&result).unwrap();
        assert_eq!(response.data.len(), 0);
        assert_eq!(
            response.page_info,
            PageInfo {
                has_next_page: false,
                cursor: "".to_string(),
            }
        );

        let data = general_purpose::STANDARD.encode("object1");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
        };
        execute(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();
        let data = general_purpose::STANDARD.encode("object2");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
        };
        execute(deps.as_mut(), mock_env(), info1, msg).unwrap();
        let data = general_purpose::STANDARD.encode("object3");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
        };
        execute(deps.as_mut(), mock_env(), info2, msg).unwrap();

        let cases = vec![
            (
                QueryMsg::Objects {
                    address: None,
                    first: None,
                    after: None,
                },
                3,
                PageInfo {
                    has_next_page: false,
                    cursor: "CZC4Avd5xNeJaBkK6MYrA1ZSQPNr76GU1k2JJSjmaDyF".to_string(),
                },
            ),
            (
                QueryMsg::Objects {
                    address: Some("unknown".to_string()),
                    first: None,
                    after: None,
                },
                0,
                PageInfo {
                    has_next_page: false,
                    cursor: "".to_string(),
                },
            ),
            (
                QueryMsg::Objects {
                    address: Some("creator1".to_string()),
                    first: None,
                    after: None,
                },
                2,
                PageInfo {
                    has_next_page: false,
                    cursor: "CZC4Avd5xNeJaBkK6MYrA1ZSQPNr76GU1k2JJSjmaDyF".to_string(),
                },
            ),
            (
                QueryMsg::Objects {
                    address: Some("creator1".to_string()),
                    first: Some(1),
                    after: None,
                },
                1,
                PageInfo {
                    has_next_page: true,
                    cursor: "5bfWM6UF5MowkQVp16q5pnXvwc9SVkS4xZkFeVLdswjU".to_string(),
                },
            ),
            (
                QueryMsg::Objects {
                    address: Some("creator1".to_string()),
                    first: Some(1),
                    after: Some("5bfWM6UF5MowkQVp16q5pnXvwc9SVkS4xZkFeVLdswjU".to_string()),
                },
                1,
                PageInfo {
                    has_next_page: false,
                    cursor: "CZC4Avd5xNeJaBkK6MYrA1ZSQPNr76GU1k2JJSjmaDyF".to_string(),
                },
            ),
        ];

        for case in cases {
            let msg = case.0;
            let result = query(deps.as_ref(), mock_env(), msg).unwrap();
            let response: ObjectsResponse = from_json(&result).unwrap();
            assert_eq!(response.data.len(), case.1);
            assert_eq!(response.page_info, case.2);
        }

        let msg = QueryMsg::Objects {
            address: Some("creator2".to_string()),
            first: None,
            after: None,
        };
        let result = query(deps.as_ref(), mock_env(), msg).unwrap();
        let response: ObjectsResponse = from_json(&result).unwrap();
        assert_eq!(
            response.data.first().unwrap(),
            &ObjectResponse {
                id: "0a6d95579ba3dd2f79c870906fd894007ce449020d111d358894cfbbcd9a03a4".to_string(),
                owner: "creator2".to_string(),
                is_pinned: false,
                size: 7u128.into(),
                compressed_size: 7u128.into(),
                compression_algorithm: CompressionAlgorithm::Passthrough,
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
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
        };
        instantiate(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();

        let data = general_purpose::STANDARD.encode("object1");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: false,
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
        };
        execute(deps.as_mut(), mock_env(), info1.clone(), msg).unwrap();
        // 1: 445008b7f2932922bdb184771d9978516a4f89d77000c2d6eab18b0894aac3a7
        let data = general_purpose::STANDARD.encode("object2");
        let msg = ExecuteMsg::StoreObject {
            data: Binary::from_base64(data.as_str()).unwrap(),
            pin: true,
            compression_algorithm: Some(CompressionAlgorithm::Passthrough),
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
            let response: ObjectPinsResponse = from_json(&result).unwrap();
            assert_eq!(response.data, case.1);
            assert_eq!(response.page_info, case.2);
        }
    }

    #[test]
    fn object_pins_errors() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            bucket: String::from("test"),
            config: Default::default(),
            limits: Default::default(),
            pagination: Default::default(),
        };
        instantiate(deps.as_mut(), mock_env(), mock_info("creator1", &[]), msg).unwrap();

        let cases = vec![
            (
                QueryMsg::ObjectPins {
                    id: "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56"
                        .to_string(),
                    after: None,
                    first: None,
                },
                ContractError::Std(StdError::not_found(not_found_object_info::<Object>(
                    "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56",
                ))),
            ),
            (
                QueryMsg::ObjectPins {
                    id: "invalid id".to_string(),
                    after: None,
                    first: None,
                },
                ContractError::Std(StdError::parse_err(
                    type_name::<Vec<u8>>(),
                    "invalid Base16 encoding".to_string(),
                )),
            ),
        ];

        for case in cases {
            let res = query(deps.as_ref(), mock_env(), case.0).err().unwrap();
            assert_eq!(res, case.1)
        }
    }

    #[test]
    fn forget_object() {
        struct TC {
            pins: Vec<ObjectId>,
            pins_senders: Vec<MessageInfo>,
            forget_objects: Vec<ObjectId>,
            forget_senders: Vec<MessageInfo>,
            expected_count: usize,
            expected_total_size: Uint128,
            expected_error: Option<ContractError>,
        }

        let cases = vec![
            TC {
                pins: vec![],
                pins_senders: vec![],
                forget_objects: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                forget_senders: vec![mock_info("bob", &[])],
                expected_count: 2,
                expected_total_size: Uint128::new(9),
                expected_error: None,
            },
            TC {
                pins: vec![],
                pins_senders: vec![],
                forget_objects: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
                    ),
                ],
                forget_senders: vec![mock_info("bob", &[]), mock_info("bob", &[])],
                expected_count: 1,
                expected_total_size: Uint128::new(4),
                expected_error: None,
            },
            TC {
                pins: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                pins_senders: vec![mock_info("bob", &[])],
                forget_objects: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                forget_senders: vec![mock_info("alice", &[])], // the sender is different from the pinner, so error
                expected_count: 3,
                expected_total_size: Uint128::new(13),
                expected_error: Some(ContractError::ObjectPinned {}),
            },
            TC {
                pins: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                pins_senders: vec![mock_info("bob", &[])],
                forget_objects: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                forget_senders: vec![mock_info("bob", &[])], // the sender is the same as the pinner, so forget should work
                expected_count: 2,
                expected_total_size: Uint128::new(9),
                expected_error: None,
            },
            TC {
                pins: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                ],
                pins_senders: vec![mock_info("bob", &[]), mock_info("alice", &[])],
                forget_objects: vec![ObjectId::from(
                    "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                )],
                forget_senders: vec![mock_info("bob", &[])], // the sender is the same as the pinner, but another pinner is on it so error
                expected_count: 3,
                expected_total_size: Uint128::new(13),
                expected_error: Some(ContractError::ObjectPinned {}),
            },
            TC {
                pins: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                ],
                pins_senders: vec![mock_info("bob", &[]), mock_info("alice", &[])],
                forget_objects: vec![ObjectId::from(
                    "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56",
                )],
                forget_senders: vec![mock_info("bob", &[])], // the sender is the same as the pinner, but another pinner is on it so error
                expected_count: 3,
                expected_total_size: Uint128::new(13),
                expected_error: Some(ContractError::Std(StdError::not_found(
                    not_found_object_info::<Object>(
                        "abafa4428bdc8c34dae28bbc17303a62175f274edf59757b3e9898215a428a56",
                    ),
                ))),
            },
            TC {
                pins: vec![
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                    ObjectId::from(
                        "315d0d9ab12c5f8884100055f79de50b72db4bd2c9bfd3df049d89640fed1fa6",
                    ),
                ],
                pins_senders: vec![mock_info("bob", &[]), mock_info("alice", &[])],
                forget_objects: vec![ObjectId::from("invalid id")],
                forget_senders: vec![mock_info("bob", &[])], // the sender is the same as the pinner, but another pinner is on it so error
                expected_count: 3,
                expected_total_size: Uint128::new(13),
                expected_error: Some(ContractError::Std(StdError::parse_err(
                    type_name::<Vec<u8>>(),
                    "invalid Base16 encoding".to_string(),
                ))),
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
                    config: Default::default(),
                    limits: Default::default(),
                    pagination: Default::default(),
                },
            )
            .unwrap();

            let data = general_purpose::STANDARD.encode("okp4");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("data");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            let data = general_purpose::STANDARD.encode("hello");
            let msg = ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            };
            let _ = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

            case.pins
                .iter()
                .zip(case.pins_senders)
                .for_each(|(object_id, info)| {
                    _ = execute(
                        deps.as_mut(),
                        mock_env(),
                        info,
                        ExecuteMsg::PinObject {
                            id: object_id.clone(),
                        },
                    );
                });

            let mut last_result: Option<Result<Response, ContractError>> = None;

            case.forget_objects
                .iter()
                .zip(case.forget_senders)
                .for_each(|(object_id, info)| {
                    last_result = Some(execute(
                        deps.as_mut(),
                        mock_env(),
                        info,
                        ExecuteMsg::ForgetObject {
                            id: object_id.clone(),
                        },
                    ));
                });

            match case.expected_error {
                Some(err) => assert_eq!(last_result.unwrap().unwrap_err(), err),
                _ => {
                    for object_id in case.forget_objects {
                        assert_eq!(
                            objects()
                                .load(&deps.storage, decode_hex(object_id.as_str()).into())
                                .unwrap_err(),
                            StdError::not_found(not_found_object_info::<Object>(&object_id))
                        );
                    }
                }
            }
            assert_eq!(
                objects()
                    .keys_raw(&deps.storage, None, None, Order::Ascending)
                    .count(),
                case.expected_count
            );
            let bucket = BUCKET.load(&deps.storage).unwrap();
            assert_eq!(
                bucket.stat.object_count,
                Uint128::from(case.expected_count as u128)
            );
            assert_eq!(bucket.stat.size, case.expected_total_size);
        }
    }

    #[test]
    fn store_forgotten_object() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &[]);

        instantiate(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            InstantiateMsg {
                bucket: "test".to_string(),
                config: Default::default(),
                limits: Default::default(),
                pagination: Default::default(),
            },
        )
        .unwrap();

        let data = general_purpose::STANDARD.encode("data");
        let _ = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            },
        )
        .unwrap();

        let _ = execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            ExecuteMsg::ForgetObject {
                id: "3a6eb0790f39ac87c94f3856b2dd2c5d110e6811602261a9a923d3bb23adc8b7".to_string(),
            },
        )
        .unwrap();

        let result = execute(
            deps.as_mut(),
            mock_env(),
            info,
            ExecuteMsg::StoreObject {
                data: Binary::from_base64(data.as_str()).unwrap(),
                pin: false,
                compression_algorithm: Some(CompressionAlgorithm::Passthrough),
            },
        );

        assert_eq!(
            result.err(),
            None,
            "Object should successfully restored after a forgot"
        );
    }
}
