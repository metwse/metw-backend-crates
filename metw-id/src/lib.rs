//! # metw-id
//!
//! Unique identifier types and ID generation algorithms.

mod checked_now;

pub mod snowflake;

pub use checked_now::checked_now;

#[doc(hidden)]
pub mod __private {
    pub use pastey;
    pub use serde;
    pub use serde_with;
    pub use sqlx;
    pub use utoipa;
}

/// Defines the ID wrapper.
#[macro_export]
macro_rules! define_id {
    ($name:ident) => {
        $crate::__private::pastey::paste! {
            #[allow(non_snake_case)]
            mod [< __define_id_ $name >] {
                use $crate::__private::{
                    serde::{Deserialize, Serialize},
                    serde_with::{DisplayFromStr, serde_as},
                    utoipa::{PartialSchema, ToSchema, openapi},
                    sqlx
                };
                use std::str::FromStr;
                use $crate::snowflake;

                #[doc = "Unique "]
                #[doc = stringify!($name)]
                #[doc = " identifier."]
                #[$crate::__private::serde_with::serde_as(
                    crate = "metw_id::__private::serde_with"
                )]
                #[derive(
                    Debug, Clone, Copy, Serialize, Deserialize,
                    Hash, PartialEq, Eq
                )]
                #[serde(crate = "metw_id::__private::serde")]
                pub struct [< $name Id >](
                    #[serde_as(as = "DisplayFromStr")]
                    i64
                );

                impl [< $name Id >] {
                    /// Generates a new unique ID using the snowflake algorithm.
                    pub fn unique() -> Self {
                        [< $name Id >](snowflake::next())
                    }
                }

                /// Creating ID from arbitrary values only allowed in tests.
                #[cfg(test)]
                impl From<i64> for [< $name Id >] {
                    fn from(value: i64) -> Self {
                        [< $name Id >](value)
                    }
                }

                impl PartialOrd for [< $name Id >] {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        self.0.partial_cmp(&other.0)
                    }
                }

                impl Ord for [< $name Id >] {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        self.0.cmp(&other.0)
                    }
                }

                impl ToSchema for [< $name Id >] {
                    fn name() -> std::borrow::Cow<'static, str> {
                        std::borrow::Cow::Borrowed(stringify!([< $name Id >]))
                    }
                }

                impl PartialSchema for [< $name Id >] {
                    fn schema() -> openapi::RefOr<openapi::schema::Schema> {
                        openapi::schema::Object::builder()
                            .schema_type(openapi::schema::Type::String)
                            .into()
                    }
                }

                impl FromStr for [< $name Id >] {
                    type Err = std::num::ParseIntError;

                    fn from_str(src: &str) -> Result<Self, Self::Err> {
                        i64::from_str(src).map([< $name Id >])
                    }
                }

                impl std::fmt::Display for [< $name Id >] {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.0.fmt(f)
                    }
                }

                impl sqlx::Type<sqlx::Postgres> for [< $name Id >] {
                    fn type_info() -> sqlx::postgres::PgTypeInfo {
                        <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
                    }

                    fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
                        <i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
                    }
                }

                impl<'q> sqlx::Encode<'q, sqlx::Postgres> for [< $name Id >] {
                    fn encode_by_ref(&self, buf: &mut sqlx::postgres::PgArgumentBuffer)
                        -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError>
                    {
                        <i64 as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(
                            &self.0,
                            buf
                        )
                    }

                    fn size_hint(&self) -> usize {
                        <i64 as sqlx::Encode<'q, sqlx::Postgres>>::size_hint(&self.0)
                    }
                }

                impl<'r> sqlx::Decode<'r, sqlx::Postgres> for [< $name Id >] {
                    fn decode(value: sqlx::postgres::PgValueRef<'r>)
                        -> Result<Self, sqlx::error::BoxDynError>
                    {
                        Ok(Self(
                            <i64 as sqlx::Decode<'r, sqlx::Postgres>>::decode(value)?
                        ))
                    }
                }
            }

            pub use [< __define_id_ $name >]::[< $name Id >];
        }
    };
}
