use std::assert_eq;

use crate::cache::{CacheKey, CacheUserFilter, CacheUserIdType};
use entity::sea_orm::prelude::Uuid;
use strum::EnumCount;

#[test]
fn unique_cache_keys() {
    let variant_count = CacheKey::COUNT;
    let user_filter_count = CacheUserFilter::COUNT;
    let user_id_count = CacheUserIdType::COUNT;

    let mut sample = Vec::with_capacity(variant_count);
    let page = 1;
    let max_per_page = 10;
    let parent_id = 0;
    let token = "token";
    let provider = "provider";
    let provider_account_id = "provider_account_id";
    let id = 12;
    let users = CacheKey::Users;
    sample.push(&users);

    let categories = CacheKey::Categories {
        page,
        max_per_page,
        parent_id: Some(parent_id),
    };
    sample.push(&categories);

    let category = CacheKey::Category { id };
    sample.push(&category);

    let mut user_queries = Vec::with_capacity(user_filter_count);

    let user_1 = CacheKey::User(CacheUserFilter::Session {
        token: token.to_owned(),
    });
    user_queries.push(&user_1);

    let user_2 = CacheKey::User(CacheUserFilter::Account {
        provider: provider.to_owned(),
        provider_account_id: provider_account_id.to_owned(),
    });
    user_queries.push(&user_2);

    let user_3 = CacheKey::User(CacheUserFilter::Id(CacheUserIdType::ID(Uuid::default())));
    user_queries.push(&user_3);

    let mut user_id_type = Vec::with_capacity(user_id_count);
    let user_4 = CacheKey::User(CacheUserFilter::Id(CacheUserIdType::Email(String::from(
        "userid",
    ))));
    user_id_type.push(&user_3);
    user_id_type.push(&user_4);

    sample.push(&user_3);

    let session = CacheKey::Session {
        token: token.to_owned(),
    };
    sample.push(&session);

    assert_eq!(user_queries.len(), user_filter_count);

    assert_eq!(sample.len(), variant_count);

    let mut keys: Vec<String> = sample.iter().map(|f| f.to_string()).collect();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), sample.len());
}
