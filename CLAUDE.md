# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust 后端 API 模板项目，基于 axum + diesel-async + Tokio，使用 PostgreSQL 和 Redis。使用 cookie 认证

## Common Commands

```bash
# 开发（热重载）
ACTIVE_CONFIG="local" cargo run          # 本地开发
cargo check          # 仅类型检查（更快）

# 其他环境
ACTIVE_CONFIG="dev" cargo run            # Dev 环境（-c dev）
ACTIVE_CONFIG="prod" cargo run      # Prod 环境

# 构建与测试
cargo build          # Release 构建
cargo test           # cargo test
cargo test <test_name> -- --nocapture  # 运行单个测试并显示输出
cargo test --test <test_file_name>     # 运行集成测试

# 数据库迁移
无需迁徙 , 每次启动自动同步

# 登录 并 获取cookie 
curl 'http://127.0.0.1:5090/api/v1/auth/login' \
  -H 'Accept-Language: zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6,zh-TW;q=0.5' \
  -H 'Cache-Control: no-cache' \
  -H 'Connection: keep-alive' \
  -b 'id=08JrxNFujHxR1cnhMffejg' \
  -H 'DNT: 1' \
  -H 'Origin: http://127.0.0.1:5090' \
  -H 'Pragma: no-cache' \
  -H 'Referer: http://127.0.0.1:5090/docs/pretty_doc' \
  -H 'Sec-Fetch-Dest: empty' \
  -H 'Sec-Fetch-Mode: cors' \
  -H 'Sec-Fetch-Site: same-origin' \
  -H 'User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36 Edg/149.0.0.0' \
  -H 'accept: */*' \
  -H 'content-type: application/json' \
  -H 'sec-ch-ua: "Microsoft Edge";v="149", "Chromium";v="149", "Not)A;Brand";v="24"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Windows"' \
  -H 'sec-gpc: 1' \
  --data-raw $'{"next": null,\n  "password": "1234qwer",\n  "username": "super_admin"}'
# API 测试（开发完成后必须执行）
curl -X POST http://127.0.0.1:5090/<endpoint> \
  -H "Content-Type: application/json" \
  -b 'id=<cookie>' \
  -d '{"key": "value"}'
```

## Architecture

**启动流程** (`src/main.rs`):

1. 根据环境变量(ACTIVE_CONFIG) 选择.env配置文件
2. 加载配置 → 初始化数据库（自动建库+迁移） → 启动 HTTP 服务器

**全局单例** (定义在 `src/lib.rs`，通过 `LazyLock` 初始化):

- `CONFIG` — 应用配置 (`CONFIG`，从 env 加载)
- `HTTP_CLIENT` — 默认 HTTP 客户端

**配置系统** (`src/config/mod.rs`):

- 加载顺序：`.env` → `命令行env` → `{x}.env}`（后者覆盖前者）
- 支持环境：`local`、`dev`、`test`、`prod`

**代码分层**:

- `src/api_router/` — 路由定义层,所有api路由。所有路由挂载在 `/api/v1` 前缀下
- `src/api_service/` — 业务逻辑层，每个api的具体实现,仅包含api相关的逻辑
- `src/api_wrapper/` — 第三方api 包装 , 所有第三方api 都要有重试 与 兜底逻辑
- `src/bin/` — 其他可执行文件位置
- `src/config/` — 配置文件定义与加载
- `src/db_model/` — 数据库模型,使用diesel
- `src/domain/` — 领域模型
- `src/framework/` — 基础设施
- `src/middleware/` — api中间件
- `src/scheduled_task/` — 定时任务
- `src/util/` — 工具方法位置
- `migrations/` — 数据库版本管理

## Adding a New API Endpoint

参考 src/api_router/user.rs#L21-53
额外要求:

1. 返回值一律使用 crate::AppRes
2. 逻辑实现一律使用diesel_async 的异步实现 , 不要直接使用diesel 的同步实现

**开发完成后必须**：

1. 使用 curl 测试每个接口，验证请求/响应格式、参数校验、错误处理是否符合需求

## Error Handling

统一全部使用 crate::AppRes

## Common Access Patterns

```rust
// 数据库使用 eg:
diesel::insert_into(req_records)
.values(new_req_record.clone())
.execute( & mut DB.get().await?)
.await?;
// 注意: 数据库连接使用完要尽量早的drop , 或者 `xxx.execute(&mut DB.get().await?).await?` 的方式使用;

// Redis 使用eg
crate::REDIS
.clone()
.hset_multiple("devWalletMap:8453", resp.as_slice())
.await
.unwrap();

// 配置 获取eg:
let database_url = CONFIG.database_url;
//响应定义  eg:
#[derive(Serialize, Deserialize, JsonSchema, OperationIo)]
struct FileSave {
    /// 文件名
    filename: String,
    ///文件hash
    hash: String,
}

// 分页eg: 
pub async fn get_entity_page(
    Json(page): Json<PageParam<Filter>>,
) -> Result<Json<PageRes<Group, Filter>>, AppError> {
    let statement = crate::schema::groups::dsl::groups.into_boxed();
    let x_table = diesel_dynamic_schema::table(stringify!(groups));
    let filters = page.filters.clone();
    let statement = filters.append_to_sql(statement, &x_table);
    let order_column = x_table.column::<diesel::sql_types::Text, _>(page.order_column.clone());
    let res = if page.is_desc {
        statement
            .order(order_column.desc().nulls_last())
            .select(Group::as_select())
            .logic_delete_query()
            .paginate(page.page_no, page.page_size)
            .load_and_count_pages(&mut DB.get().await?)
            .await?
    } else {
        statement
            .order(order_column.asc().nulls_last())
            .select(Group::as_select())
            .logic_delete_query()
            .paginate(page.page_no, page.page_size)
            .load_and_count_pages(&mut DB.get().await?)
            .await?
    };
    let page_res = PageRes::from_param_records_count(page, res.0, res.1);
    Ok(Json(page_res))
}
```

## Code Quality Requirements

### 函数文档

每个函数必须有完整的 doc comment（`///`），包含：

- 函数用途说明
- 每个参数的作用与约束
- 返回值含义
- 可能的错误情况（如返回 `Result`）

```rust
/// 根据平台和小说 ID 查询小说详情
///
/// # Arguments
/// * `platform` - 小说来源平台标识（如 "qidian"、"fanqie"）
/// * `novel_id` - 该平台上的小说唯一 ID
///
/// # Returns
/// 小说实体，未找到时返回 `None`
///
/// # Errors
/// 数据库连接失败时返回 `AppError`
pub async fn find_by_platform(platform: &str, novel_id: &str) -> AppRes<Option<novel::Model>> {
```

### 集成测试

- 新增或修改功能必须编写集成测试，放在 `tests/` 目录下
- 集成测试覆盖完整的请求链路（HTTP 请求 → handler → service → 数据库）
- 运行集成测试：`cargo test --test <test_file_name>`

## Development Workflow

开发新功能时遵循以下流程：

1. **需求分析** — 分析需求，明确功能边界、输入输出、错误处理
2. **数据库设计** — 如需新表，参考migrations/2025-11-26-071432_create_req_record,创建数据库版本管理
   sql ,时间使用当前时间，参考src/db_model/req_record.rs生成 Entity ,entity 要求 , 尽量严格的限制类型 比如日期使用DateTime<Utc> ,有符号数字使用i64 ,uuid 使用 Uuid等等;  
3. **编写代码** — 按分层架构编写 router → service → model，添加完整 doc comment , 函数参数尽量严格的限制类型 比如日期使用DateTime<Utc> ,有符号数字使用i64 ,uuid 使用 Uuid等等;
4. **编写测试** — 集成测试覆盖完整链路
5. **本地验证** — `ACTIVE_CONFIG="local" cargo run` 启动服务，用 curl 测试所有接口 ,测试完成后关闭 
6. **需求对照** — 对照需求，确认无遗漏
7. **代码审查** — 检查代码质量、安全性、性能
8. **提交代码** — 确保 `cargo test` 全部通过后再提交

## Key Conventions
- 数据库表统一使用 `deleted_at` 软删除
- 数据库启动时自动建库（如不存在）并执行迁移，无需手动 `migrate up`
