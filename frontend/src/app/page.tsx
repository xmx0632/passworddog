import Image from "next/image";
import Link from "next/link";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center p-8">
      <main className="w-full max-w-md space-y-8">
        <div className="text-center">
          <h1 className="text-4xl font-bold">PasswordDog</h1>
          <p className="mt-2 text-gray-600 dark:text-gray-400">
            安全的密码管理工具
          </p>
        </div>

        <div className="mt-8 space-y-4">
          <Link
            href="/auth/login"
            className="flex w-full justify-center rounded-lg bg-blue-600 px-4 py-2.5 text-white hover:bg-blue-700"
          >
            登录
          </Link>
          <Link
            href="/auth/register"
            className="flex w-full justify-center rounded-lg border border-gray-300 px-4 py-2.5 text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-800"
          >
            注册
          </Link>
        </div>

        <div className="mt-6 text-center text-sm text-gray-500">
          <Link href="/auth/forgot-password" className="hover:text-blue-600">
            忘记密码？
          </Link>
        </div>
      </main>
    </div>
  );
}
