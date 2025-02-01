# PasswordDog 前端

基于 Next.js 的密码管理系统前端实现。

## 技术栈

- 框架：Next.js 13+ (App Router)
- UI组件：shadcn/ui
- 样式：TailwindCSS
- 状态管理：React Hooks

## 目录结构

```text
frontend/
├── src/
│   ├── app/           # Next.js 13+ App Router
│   │   ├── layout.tsx    # 全局布局
│   │   ├── page.tsx      # 首页
│   │   ├── auth/         # 认证相关页面
│   │   │   ├── login/      # 登录
│   │   │   ├── register/   # 注册
│   │   │   └── forgot-password/  # 忘记密码
│   │   └── passwords/    # 密码管理页面
│   ├── components/    # 可复用组件
│   │   ├── ui/         # UI组件 (shadcn)
│   │   └── password/   # 密码相关组件
│   └── lib/          # 工具函数和类型定义
├── public/           # 静态资源
├── tailwind.config.js
├── package.json
└── next.config.js
```

## 页面设计

### 1. 首页 (/)
- 未登录时显示登录/注册选项
- 已登录时重定向到密码管理页面

### 2. 认证页面
- 登录页面 (/auth/login)
- 注册页面 (/auth/register)
- 忘记密码页面 (/auth/forgot-password)

### 3. 密码管理页面 (/passwords)
- 密码列表展示
- 添加新密码
- 搜索和过滤功能
- 密码详情查看

## 开发指南

This is a [Next.js](https://nextjs.org) project bootstrapped with [`create-next-app`](https://nextjs.org/docs/app/api-reference/cli/create-next-app).

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.
