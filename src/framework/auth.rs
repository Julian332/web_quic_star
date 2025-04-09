use crate::db_models::group::Group;
use crate::db_models::user::User;
use crate::db_models::{ConnPool, DbType};
use crate::framework::errors::AppError;
use crate::schema::groups::table as groups;
use crate::schema::users::{table as users, username};
use crate::{impl_from, DB};
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use axum_login::{
    AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, AuthzBackend, UserId,
};
use diesel::associations::HasTable;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::{Text, VarChar};
use diesel::{
    deserialize, serialize, ExpressionMethods, FromSqlRow, JoinOnDsl, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[allow(dead_code)]
const LOGIN_MESSAGE: &str = "welcome";
pub const DEFAULT_TENANTRY: &str = "default";
pub const COMMON_USER_ROLE: i64 = -1;
pub const COMMON_USER: i64 = -1;
pub const SUPER_USER_ROLE: i64 = -2;
pub const SUPER_USER: i64 = -2;

#[derive(Debug, FromSqlRow, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq, Hash)]
#[diesel(sql_type = Text)]
pub enum AuthPermission2 {
    Admin,
    TablePermissions(TablePermission),
}

impl Display for AuthPermission2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AuthPermission2::Admin => "Admin".to_string(),
            AuthPermission2::TablePermissions(x) => x.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl From<&str> for AuthPermission2 {
    fn from(s: &str) -> Self {
        match s {
            string if string.eq_ignore_ascii_case("Admin") => AuthPermission2::Admin,
            _ => AuthPermission2::TablePermissions(TablePermission::from(s)),
        }
    }
}
impl FromStr for AuthPermission2 {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            string if string.eq_ignore_ascii_case("Admin") => AuthPermission2::Admin,
            _ => AuthPermission2::TablePermissions(TablePermission::from_str(s)?),
        };
        Ok(result)
    }
}

impl ToSql<Text, DbType> for AuthPermission2 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DbType>) -> serialize::Result {
        <String as ToSql<VarChar, DbType>>::to_sql(&self.to_string(), &mut out.reborrow())
    }
}

impl FromSql<Text, DbType> for AuthPermission2 {
    fn from_sql(
        bytes: <DbType as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let string = <String as FromSql<VarChar, DbType>>::from_sql(bytes)?;
        let perm = AuthPermission2::from_str(&string).map_err(Box::new)?;

        Ok(perm)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Eq, PartialEq, Hash)]
pub enum TablePermission<Table = String> {
    Read(Table),
    Add(Table),
    Delete(Table),
    Update(Table),
}
impl FromStr for TablePermission {
    type Err = AuthError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut split = value.split(":");
        let table = split.next().expect("table must exist");
        let permission = split.next().expect("permission must exist");
        let permission = match permission {
            permission if permission.eq_ignore_ascii_case("read") => Self::Read(table.to_string()),
            permission if permission.eq_ignore_ascii_case("add") => Self::Add(table.to_string()),
            permission if permission.eq_ignore_ascii_case("delete") => {
                Self::Delete(table.to_string())
            }
            permission if permission.eq_ignore_ascii_case("update") => {
                Self::Update(table.to_string())
            }
            _ => {
                return Err(AppError::new(&format!("unknown permission: {}", value)).into());
            }
        };
        Ok(permission)
    }
}

impl From<&str> for TablePermission {
    fn from(value: &str) -> Self {
        let mut split = value.split(":");
        let table = split.next().expect("table must exist");
        let permission = split.next().expect("permission must exist");
        match permission {
            permission if permission.eq_ignore_ascii_case("read") => Self::Read(table.to_string()),
            permission if permission.eq_ignore_ascii_case("add") => Self::Add(table.to_string()),
            permission if permission.eq_ignore_ascii_case("delete") => {
                Self::Delete(table.to_string())
            }
            permission if permission.eq_ignore_ascii_case("update") => {
                Self::Update(table.to_string())
            }
            _ => {
                panic!("invalid table permission");
            }
        }
    }
}

impl Display for TablePermission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TablePermission::Read(t) => {
                format!("{t}:Read")
            }
            TablePermission::Add(t) => {
                format!("{t}:Add")
            }
            TablePermission::Delete(t) => {
                format!("{t}:Delete")
            }
            TablePermission::Update(t) => {
                format!("{t}:Update")
            }
        };
        write!(f, "{}", str)
    }
}

#[test]
fn permissions_test() {
    let perm = TablePermission::from("t:add");
    let perm2 = TablePermission::from("t:Add");
    println!("{:?}", perm);
    println!("{:?}", perm2);
    println!("{}", perm.to_string());
    println!("{}", perm2.to_string());
}

#[test]
pub fn test() {
    println!("{}", password_auth::generate_hash("1234qwer"));
}

pub fn get_auth_layer() -> AuthManagerLayer<AuthBackend, MemoryStore> {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = AuthBackend::new(DB.clone());
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

#[derive(Debug, Clone)]
pub struct AuthBackend {
    db: ConnPool,
}

#[cfg(any(
    all(not(feature = "eth_mode"), not(feature = "solana_mode")),
    all(feature = "eth_mode", feature = "solana_mode")
))]
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[cfg(all(feature = "eth_mode", not(feature = "solana_mode")))]
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub user_addr: crate::domain::eth_addr::EthAddr,
    pub signature: String,
    pub next: Option<String>,
}
#[cfg(all(feature = "solana_mode", not(feature = "eth_mode")))]
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Credentials {
    pub user_addr: crate::domain::solana_addr::SolAddr,
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
    pub fn new(pool: ConnPool) -> Self {
        Self { db: pool }
    }
}

#[derive(Debug)]
pub struct AuthError(AppError);

impl From<AppError> for AuthError {
    fn from(value: AppError) -> Self {
        AuthError(value)
    }
}

impl std::error::Error for AuthError {}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(+error:{})", self.0)
    }
}

impl AuthnBackend for AuthBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    #[cfg(any(
        all(not(feature = "eth_mode"), not(feature = "solana_mode")),
        all(feature = "eth_mode", feature = "solana_mode")
    ))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use password_auth::verify_password;

        match users
            .filter(username.eq(creds.username))
            .select(User::as_select())
            .first(&mut self.db.get()?)
        {
            Ok(user) => verify_password(creds.password, &user.password)
                .map_err(|e| AuthError(AppError::from(e)))
                .map(|_| Some(user)),
            Err(e) => Err(e.into()),
        }
    }

    #[cfg(all(feature = "eth_mode", not(feature = "solana_mode")))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use crate::db_models::user::NewUser;
        use alloy::signers::Signature;
        use diesel::OptionalExtension;
        use std::str::FromStr;
        use std::time::SystemTime;
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
    #[cfg(all(feature = "solana_mode", not(feature = "eth_mode")))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use crate::db_models::user::NewUser;
        use diesel::OptionalExtension;
        use std::str::FromStr;
        use std::time::SystemTime;
        let signature =
            anchor_client::solana_sdk::signature::Signature::from_str(&creds.signature)?;
        let user_addr = creds.user_addr.0;
        let is_validate = signature.verify(LOGIN_MESSAGE.as_ref(), user_addr.as_ref());
        if !is_validate {
            return Err(AuthError(AppError::new("wrong signature")));
        }

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
#[test]
#[cfg(feature = "solana_mode")]
fn test1() {
    use anchor_client::solana_sdk::signature::Keypair;
    use anchor_client::solana_sdk::signer::Signer;

    let keypair = Keypair::new();
    let x = b"messagee";
    // let result = Signature::from_str("asd").unwrap();
    let signature = keypair.sign_message(x);
    let x1 = signature.verify(keypair.pubkey().as_ref(), x);
    println!("{}", x1);
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

// #[async_trait]
impl AuthzBackend for AuthBackend {
    type Permission = AuthPermission2;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        match users
            .inner_join(groups::table())
            // .inner_join(groups_permissions.on(group_id.eq(crate::schema::groups::id)))
            // .inner_join(permissions.on(permission_id.eq(crate::schema::permissions::id)))
            .filter(crate::schema::users::id.eq(user.id))
            .select(Group::as_select())
            .load(&mut self.db.get()?)
        {
            Ok(res) => Ok(res.into_iter().map(|x| x.permissions).flatten().collect()),
            Err(e) => Err(e.into()),
        }
    }
}

impl_from!(diesel::result::Error);
impl_from!(r2d2::Error);
#[cfg(feature = "eth_mode")]
impl_from!(alloy::primitives::SignatureError);
#[cfg(feature = "solana_mode")]
impl_from!(anchor_client::solana_sdk::signature::ParseSignatureError);
