#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, Order,
};
use cw2::set_contract_version;
use crate::ContractError::{NotImplemented, ObjectMaxSizeLimitExceeded, MaxObjectPinsLimitExceeded, MaxObjectsLimitExceeded, BucketSizeLimitExceeded};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ObjectId};
use crate::state::{Bucket, BUCKET, DATA, Object, objects, Pin, pins};
use crate::crypto::sha256_hash;

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
        ExecuteMsg::StoreObject {data, pin} => execute::store_object(deps, info, data, pin),
        _ => Err(NotImplemented {})
    }
}

pub mod execute {
    use super::*;

    pub fn store_object(
        deps: DepsMut,
        info: MessageInfo,
        data: Binary,
        pin: bool,
    ) -> Result<Response, ContractError> {
        let bucket_limits = BUCKET.load(deps.storage)?.limits;

        let size = data.len() as u128;
        match bucket_limits.max_object_size {
            Some(max) if size > max.u128() => Err(ObjectMaxSizeLimitExceeded {}),
            _ => Ok({})
        }?;

        let object_count = objects()
            .keys_raw(deps.storage, None, None, Order::Ascending)
            .count() as u128;
        match bucket_limits.max_objects {
            Some(max) if object_count >= max.u128() => Err(MaxObjectsLimitExceeded {}),
            _ => Ok({})
        }?;

        if pin && bucket_limits.max_object_pins.filter(|max: &Uint128| max.u128() < 1u128).is_some(){
            return Err(MaxObjectPinsLimitExceeded {})
        }

        BUCKET.update(deps.storage, |mut bucket| -> Result<_, ContractError> {
            bucket.size += size;
            match bucket.limits.max_total_size {
                Some(max) if bucket.size > max.u128() => Err(BucketSizeLimitExceeded {}),
                _ => Ok(bucket)
            }
        })?;

        let hash = sha256_hash(&data.0);
        let res = Response::new()
            .add_attribute("action", "store_object")
            .add_attribute("id", hash.clone());

        let data_path = DATA.key(hash.clone());
        if data_path.has(deps.storage) {
            // TODO: maybe throw an error if the owner is different?
            return Ok(res)
        }

        data_path.save(deps.storage, &data.0)?;
        objects().save(deps.storage, hash.clone(), &Object {
            id: hash.clone(),
            owner: info.sender.clone(),
            size,
        })?;

        if pin {
            pins().save(deps.storage, (hash.clone(), info.sender.clone()), &Pin {
                id: hash.into(),
                address: info.sender,
            })?;
        }

        Ok(res)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Bucket {} => to_binary(&query::bucket(deps)?),
        QueryMsg::Object {id} => to_binary(&query::object(deps, id)?),
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
        objects().load(deps.storage, id)
            .map(|object| ObjectResponse{
                id: object.id,
                size: object.size.into(),
                owner: object.owner.into(),
                is_pinned: pins().idx.object
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
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Uint128};

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
}
