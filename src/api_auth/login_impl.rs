use crate::api_doc::errors::AppError;
use crate::controller::permission::Permission;
use crate::controller::user::{NewUser, User};
#[cfg(feature = "eth_mode")]
use crate::domain::eth_addr::EthAddr;
use crate::impl_from;
use crate::schema::groups::dsl::groups;
use crate::schema::groups_permissions::dsl::groups_permissions;
use crate::schema::groups_permissions::{group_id, permission_id};
use crate::schema::permissions::dsl::permissions;
use crate::schema::users::dsl::users;
use crate::schema::users::username;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use chrono::DateTime;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use password_auth::verify_password;
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::string::ToString;
use std::time::SystemTime;
const LOGIN_MESSAGE: &str = "welcome";
pub const DEFAULT_TENANTRY: &str = "default";

pub const COMMON_USER_ROLE: i64 = -1;
pub const COMMON_USER: i64 = -1;
pub const SUPER_USER_ROLE: i64 = -2;
pub const SUPER_USER: i64 = -2;

#[derive(Debug, Clone)]
pub struct AuthBackend {
    db: Pool<ConnectionManager<PgConnection>>,
}

#[cfg(not(feature = "eth_mode"))]
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[cfg(feature = "eth_mode")]
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub user_addr: EthAddr,
    pub signature: String,
    pub next: Option<String>,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

impl AuthBackend {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db: pool }
    }
}

#[derive(Debug)]
pub struct AuthError(AppError);
impl_from!(diesel::result::Error);
impl_from!(r2d2::Error);
// impl_from!(alloy::hex::FromHexError, "address format error");
#[cfg(feature = "eth_mode")]
impl_from!(alloy::primitives::SignatureError);
impl std::error::Error for AuthError {}
impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(+error:{})", self.0)
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    #[cfg(not(feature = "eth_mode"))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match users
            .filter(username.eq(creds.username))
            .select(User::as_select())
            .first(&mut self.db.get()?)
        {
            Ok(user) => verify_password(creds.password, &user.password)
                .map_err(|e| AuthError(AppError::new(e.to_string())))
                .map(|_| Some(user)),
            Err(e) => Err(e.into()),
        }
    }

    #[cfg(feature = "eth_mode")]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use alloy::signers::Signature;
        let signature = Signature::from_str(&creds.signature)?;
        let recovered_addr = signature.recover_address_from_msg(LOGIN_MESSAGE)?;
        let user_addr = creds.user_addr.0;

        assert_eq!(recovered_addr, user_addr, "not equal ");

        match users
            .filter(username.eq(user_addr.to_string()))
            .select(User::as_select())
            .first(&mut self.db.get()?)
            .optional()
        {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => {
                let user = diesel::insert_into(users)
                    .values(NewUser {
                        username: user_addr.to_string(),
                        password: password_auth::generate_hash(creds.signature),
                        group_id: COMMON_USER_ROLE,
                        tenantry: DEFAULT_TENANTRY.to_string(),
                        remark: None,
                        create_time: SystemTime::now().into(),
                        create_by: SUPER_USER,
                        is_delete: false,
                    })
                    .returning(User::as_select())
                    .get_result(&mut self.db.get()?)?;
                Ok(Some(user))
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        match users
            .find(user_id)
            .select(User::as_select())
            .first(&mut self.db.get()?)
        {
            Ok(user) => Ok(Some(user)),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AuthPermission {
    pub name: String,
}

impl From<&str> for AuthPermission {
    fn from(name: &str) -> Self {
        AuthPermission {
            name: name.to_string(),
        }
    }
}
impl From<String> for AuthPermission {
    fn from(name: String) -> Self {
        AuthPermission { name }
    }
}
#[async_trait]
impl AuthzBackend for AuthBackend {
    type Permission = AuthPermission;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let conn = &mut self.db.get()?;
        match users
            .inner_join(groups::table())
            .inner_join(groups_permissions.on(group_id.eq(crate::schema::groups::id)))
            .inner_join(permissions.on(permission_id.eq(crate::schema::permissions::id)))
            .filter(crate::schema::users::id.eq(user.id))
            .select(Permission::as_select())
            .load(conn)
        {
            Ok(res) => Ok(res.into_iter().map(|x| x.name.into()).collect()),
            Err(e) => Err(e.into()),
        }
    }
}
