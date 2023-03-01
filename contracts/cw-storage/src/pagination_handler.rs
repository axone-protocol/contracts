use crate::msg::{Cursor, PageInfo};
use crate::state::Pagination;
use cosmwasm_std::{StdError, StdResult, Storage};
use cw_storage_plus::{Bound, PrimaryKey};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

pub struct PaginationHandler<'a, T, PK>
where
    T: Serialize + DeserializeOwned,
    PK: PrimaryKey<'a>,
{
    max_page_size: u32,
    default_page_size: u32,

    _data_type: PhantomData<T>,
    _pk_type: PhantomData<PK>,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a, T, PK> From<Pagination> for PaginationHandler<'a, T, PK>
where
    T: Serialize + DeserializeOwned,
    PK: PrimaryKey<'a>,
{
    fn from(value: Pagination) -> Self {
        PaginationHandler::new(value.max_page_size, value.default_page_size)
    }
}

impl<'a, T, PK> PaginationHandler<'a, T, PK>
where
    T: Serialize + DeserializeOwned,
    PK: PrimaryKey<'a>,
{
    pub const fn new(max_page_size: u32, default_page_size: u32) -> Self {
        PaginationHandler {
            max_page_size,
            default_page_size,
            _data_type: PhantomData,
            _pk_type: PhantomData,
            _lifetime: PhantomData,
        }
    }

    pub fn query_page<I, CE, CD>(
        self,
        iter_fn: I,
        cursor_dec_fn: CD,
        cursor_enc_fn: CE,
        after: Option<Cursor>,
        first: Option<u32>,
    ) -> StdResult<(Vec<T>, PageInfo)>
    where
        I: FnOnce(Option<Bound<PK>>) -> Box<dyn Iterator<Item = StdResult<(PK, T)>> + 'a>,
        CD: FnOnce(Cursor) -> StdResult<PK>,
        CE: FnOnce(&T) -> Cursor,
    {
        let min_bound = match after {
            Some(cursor) => Some(Bound::exclusive(cursor_dec_fn(cursor)?)),
            _ => None,
        };
        let page_size = self.compute_page_size(first)?;
        let mut raw_items: Vec<T> = iter_fn(min_bound)
            .take(page_size + 1)
            .map(|res: StdResult<(PK, T)>| res.map(|(_, item)| item))
            .collect::<StdResult<Vec<T>>>()?;

        let has_next_page = raw_items.len() > page_size;
        if has_next_page {
            raw_items.pop();
        }

        let cursor = raw_items
            .last()
            .map(|item| cursor_enc_fn(item))
            .unwrap_or("".to_string());

        Ok((
            raw_items,
            PageInfo {
                has_next_page,
                cursor,
            },
        ))
    }

    fn compute_page_size(self, first: Option<u32>) -> StdResult<usize> {
        match first {
            Some(req) => {
                if req > self.max_page_size {
                    return Err(StdError::generic_err(
                        "Requested page size exceed maximum allowed",
                    ));
                }
                Ok(req)
            }
            _ => Ok(self.default_page_size),
        }
        .map(|size| size as usize)
    }
}
