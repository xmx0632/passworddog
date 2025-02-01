## 系统说明

这是一个使用 tauri rust 开发的密码管理软件。


## 功能点

1. 登录
2. 注册
3. 添加密码
4. 删除密码
5. 查询密码
6. 修改密码
7. 打开程序
8. 退出程序
9. 忘记密码
10. 导入密码
11. 导出密码



## 系统设计

### 架构

1. 前端:使用 nextjs 框架实现前端功能，使用 tailwindcss 框架实现前端样式，使用 shadcn 框架实现前端组件。
2. 后端:使用 rust 框架实现后端功能，使用 sqlite3 数据库实现本地数据库。
3. 数据交互:使用 sqlx 实现数据库交互。
4. 接口:使用 actix web 实现接口。
5. json 序列化:使用 serde 实现 json 序列化。
6. 测试:使用 cargo test 实现测试。
7. 部署:使用 tauri 打包。
8. 生成文档:使用 cargo doc 实现生成文档。
9. 生产环境打包:使用 cargo build --release 实现生产环境打包。
10. 开发环境打包:使用 cargo build 实现开发环境打包。
11. 运行:使用 cargo run 实现运行。

### 目录结构

#### 后端

```text
 backend
```


#### 前端
```text
frontend
```


### 前端

前端使用 tauri 打包。
使用nextjs框架实现前端功能，使用 tailwindcss 框架实现前端样式。
使用 shadcn 框架实现前端组件。


### 后端

后端使用rust语言实现系统接口和数据库交互。
使用sqlite3数据库实现本地数据库。
使用sqlx实现数据库交互。
接口使用actix web实现。
json序列化使用serde实现。
测试使用cargo test实现。


## 部署方法
1. 后端部署
   1. 后端项目目录下运行 `cargo run`
   2. 后端项目目录下运行 `cargo build --release`
   3. 后端项目目录下运行 `cargo build --release --features=sqlite`

2. 前端部署
   1. 前端项目目录下运行 `npm run dev`
   2. 前端项目目录下运行 `npm run build`
   3. 前端项目目录下运行 `npm run preview`

3. 运行
   1. 后端项目目录下运行 `cargo run`
   2. 前端项目目录下运行 `npm run dev`
   
4. 生产环境打包
   1. 前端项目目录下运行 `npm run build`
   2. 后端项目目录下运行 `cargo build --release`