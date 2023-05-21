use entity::sea_orm::prelude::Uuid;

pub enum CacheKey {
    Users,
    Categories {
        page: u64,
        max_per_page: u64,
        parent_id: Option<i32>,
    },
    Category {
        id: i32,
    },
    User(CacheUserFilter),
    Session {
        token: String,
    },
}

pub enum CacheUserFilter {
    Session {
        token: String,
    },
    Account {
        provider: String,
        provider_account_id: String,
    },
    Id(CacheUserIdType),
}

pub enum CacheUserIdType {
    ID(Uuid),
    Email(String),
}

impl ToString for CacheKey {
    fn to_string(&self) -> String {
        match self {
            CacheKey::Users => "users:all".to_owned(),
            CacheKey::Categories {
                parent_id,
                page,
                max_per_page,
            } => {
                format!(
                    "categories:page={page}:count={max_per_page}:parent-id={}",
                    parent_id.unwrap_or_default()
                )
            }
            CacheKey::Category { id } => {
                format!("categories:id={id}")
            }
            CacheKey::Session { token } => {
                format!("sessions:token={token}")
            }
            CacheKey::User(filter) => match filter {
                CacheUserFilter::Session { token } => {
                    format!("users:session={token}")
                }
                CacheUserFilter::Account {
                    provider,
                    provider_account_id,
                } => {
                    format!("users:provider={provider}:provider_account_id={provider_account_id}")
                }
                CacheUserFilter::Id(id_type) => {
                    format!(
                        "users:{}",
                        match id_type {
                            CacheUserIdType::ID(id) => format!("id={id}"),
                            CacheUserIdType::Email(email) => format!("email={email}"),
                        }
                    )
                }
            },
        }
    }
}
