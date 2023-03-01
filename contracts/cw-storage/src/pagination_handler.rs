use crate::msg::{Cursor, PageInfo};
use crate::state::Pagination;
use cosmwasm_std::{StdError, StdResult};
use cw_storage_plus::{Bound, PrimaryKey};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice::Iter;

    struct TestIter<'a> {
        sub_iter: Iter<'a, i32>,
        shall_err: bool,
    }

    impl<'a> TestIter<'a> {
        fn map_to_result(&mut self, val: i32) -> StdResult<(i32, i32)> {
            if self.shall_err {
                return Err(StdError::generic_err("iter error".to_string()));
            }
            Ok((val, val))
        }
    }

    impl<'a> Iterator for TestIter<'a> {
        type Item = StdResult<(i32, i32)>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.sub_iter.next() {
                Some(&x) => Some(self.map_to_result(x)),
                _ => None,
            }
        }
    }

    #[test]
    fn query_page() {
        let data = &[1, 2, 3, 4, 5];
        let handler: PaginationHandler<i32, i32> = Pagination {
            max_page_size: 3,
            default_page_size: 2,
        }
        .into();

        let iter_fn = |min_bound: Option<Bound<i32>>| match min_bound {
            Some(Bound::Exclusive((b, ..))) => Box::new(TestIter {
                sub_iter: data[b as usize..].iter(),
                shall_err: false,
            })
                as Box<dyn Iterator<Item = StdResult<(i32, i32)>>>,
            _ => Box::new(TestIter {
                sub_iter: data.iter(),
                shall_err: false,
            }),
        };
        let cursor_dec_fn =
            |cursor: Cursor| cursor.parse::<i32>().map_err(|_| StdError::generic_err(""));
        let cursor_enc_fn = |pk: &i32| pk.to_string();

        let res = handler
            .clone()
            .query_page(
                |_: Option<Bound<i32>>| {
                    Box::new(TestIter {
                        sub_iter: (&[] as &[i32]).iter(),
                        shall_err: true,
                    })
                },
                cursor_dec_fn,
                cursor_enc_fn,
                None,
                None,
            )
            .unwrap();
        assert_eq!(res.0, Vec::<i32>::new());
        assert_eq!(
            res.1,
            PageInfo {
                has_next_page: false,
                cursor: "".to_string(),
            }
        );

        let res = handler
            .clone()
            .query_page(iter_fn, cursor_dec_fn, cursor_enc_fn, None, None)
            .unwrap();
        assert_eq!(res.0, vec![1, 2]);
        assert_eq!(
            res.1,
            PageInfo {
                has_next_page: true,
                cursor: "2".to_string(),
            }
        );

        let res = handler
            .clone()
            .query_page(iter_fn, cursor_dec_fn, cursor_enc_fn, None, Some(1))
            .unwrap();
        assert_eq!(res.0, vec![1]);
        assert_eq!(
            res.1,
            PageInfo {
                has_next_page: true,
                cursor: "1".to_string(),
            }
        );

        let res = handler
            .clone()
            .query_page(iter_fn, cursor_dec_fn, cursor_enc_fn, None, Some(3))
            .unwrap();
        assert_eq!(res.0, vec![1, 2, 3]);
        assert_eq!(
            res.1,
            PageInfo {
                has_next_page: true,
                cursor: "3".to_string(),
            }
        );

        let res = handler
            .clone()
            .query_page(
                iter_fn,
                cursor_dec_fn,
                cursor_enc_fn,
                Some("1".to_string()),
                None,
            )
            .unwrap();
        assert_eq!(res.0, vec![2, 3]);
        assert_eq!(
            res.1,
            PageInfo {
                has_next_page: true,
                cursor: "3".to_string(),
            }
        );

        let res = handler
            .clone()
            .query_page(
                iter_fn,
                cursor_dec_fn,
                cursor_enc_fn,
                Some("2".to_ascii_lowercase()),
                Some(3),
            )
            .unwrap();
        assert_eq!(res.0, vec![3, 4, 5]);
        assert_eq!(
            res.1,
            PageInfo {
                has_next_page: false,
                cursor: "5".to_string(),
            }
        );

        let res = handler
            .clone()
            .query_page(
                iter_fn,
                cursor_dec_fn,
                cursor_enc_fn,
                Some("3".to_ascii_lowercase()),
                Some(3),
            )
            .unwrap();
        assert_eq!(res.0, vec![4, 5]);
        assert_eq!(
            res.1,
            PageInfo {
                has_next_page: false,
                cursor: "5".to_string(),
            }
        );
    }

    #[test]
    fn query_page_err() {
        let data = &[1, 2, 3, 4, 5];
        let handler: PaginationHandler<i32, i32> = Pagination {
            max_page_size: 3,
            default_page_size: 2,
        }
        .into();

        let iter_fn = |_: Option<Bound<i32>>| {
            Box::new(TestIter {
                sub_iter: data.iter(),
                shall_err: false,
            }) as Box<dyn Iterator<Item = StdResult<(i32, i32)>>>
        };
        let cursor_dec_fn =
            |cursor: Cursor| cursor.parse::<i32>().map_err(|_| StdError::generic_err(""));
        let cursor_enc_fn = |pk: &i32| pk.to_string();

        let res = handler.clone().query_page(
            |_: Option<Bound<i32>>| {
                Box::new(TestIter {
                    sub_iter: data.iter(),
                    shall_err: true,
                })
            },
            cursor_dec_fn,
            cursor_enc_fn,
            None,
            None,
        );
        assert_eq!(res, Err(StdError::generic_err("iter error".to_string())));

        let res = handler
            .clone()
            .query_page(iter_fn, cursor_dec_fn, cursor_enc_fn, None, Some(4));
        assert_eq!(
            res,
            Err(StdError::generic_err(
                "Requested page size exceed maximum allowed".to_string()
            ))
        );

        let res = handler.clone().query_page(
            iter_fn,
            |_| Err(StdError::generic_err("cursor decode error")),
            cursor_enc_fn,
            Some("1".to_string()),
            None,
        );
        assert_eq!(
            res,
            Err(StdError::generic_err("cursor decode error".to_string()))
        );
    }
}
