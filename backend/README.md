# PasswordDog 后端设计文档

## 技术栈

- 框架：Actix-web 4.x
- 数据库：SQLite 3 (使用 SQLx ORM)
- 加密：
  - AES-GCM 用于密码加密
  - Argon2 用于密码哈希
- 日志：env_logger

## 目录结构

```text
backend/
├── src/
│   ├── main.rs          # 主程序入口
│   ├── models/          # 数据模型
│   │   ├── mod.rs       # 模块导出
│   │   ├── user.rs      # 用户模型
│   │   └── password.rs  # 密码模型
│   ├── handlers/        # 请求处理器
│   │   ├── mod.rs       # 模块导出
│   │   ├── auth.rs      # 认证相关
│   │   └── password.rs  # 密码管理
│   ├── db/             # 数据库相关
│   │   └── mod.rs      # 数据库连接和初始化
│   └── utils/          # 工具函数
│       ├── mod.rs      # 模块导出
│       └── crypto.rs   # 加密相关
├── migrations/         # 数据库迁移
│   └── 20250201_init.sql  # 初始化表结构
├── data/              # 数据文件目录
│   └── passworddog.db    # SQLite数据库文件
├── Cargo.toml         # 项目依赖配置
└── .env              # 环境变量配置
```

## 数据库设计

### 用户表 (users)
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

### 密码表 (passwords)
```sql
CREATE TABLE passwords (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    username TEXT NOT NULL,
    encrypted_password TEXT NOT NULL,
    website TEXT,
    notes TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

## API 接口

### 认证相关

#### POST /api/login
登录接口
```json
请求体：
{
    "username": "string",
    "password": "string"
}

响应：
{
    "token": "string"
}
```

### 密码管理

#### POST /api/passwords
创建新密码
```json
请求体：
{
    "title": "string",
    "username": "string",
    "password": "string",
    "website": "string?",
    "notes": "string?"
}

响应：204 No Content
```

#### GET /api/passwords
获取密码列表
```json
响应：
[
    {
        "id": "number",
        "title": "string",
        "username": "string",
        "website": "string?",
        "notes": "string?"
    }
]
```

## 安全特性

1. 密码存储
   - 用户密码使用 Argon2 进行哈希
   - 存储的密码使用 AES-GCM 进行加密

2. 数据库安全
   - 使用 SQLite WAL 模式提高并发性能和数据完整性
   - 启用外键约束
   - 自动创建数据库目录和文件

3. API 安全
   - 所有密码相关接口需要认证
   - 使用 HTTPS 传输（生产环境）

## 开发说明

1. 环境变量配置（.env）：
```bash
DATABASE_URL="sqlite:data/passworddog.db"  # 可选，默认使用 data/passworddog.db
JWT_SECRET="your-secret-key-here"
RUST_LOG="info"                           # 日志级别
```

2. 开发命令：
```bash
# 运行开发服务器
cargo run

# 运行测试
cargo test

# 生产构建
cargo build --release
```
