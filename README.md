# Cached GUI

一款轻量级的 Redis 和 Memcached 缓存管理桌面应用，支持连接管理、键值浏览、编辑、扫描等常用操作。

## 功能特性

- **多引擎支持**：支持 Redis 和 Memcached 两种缓存引擎
- **连接管理**：支持添加、编辑、删除和测试缓存连接
- **键值浏览**：支持 SCAN 模式扫描键，支持分页加载
- **键值编辑**：支持查看、编辑、删除、新建缓存键值
- **TTL 管理**：支持为键设置过期时间
- **多数据库**：Redis 支持切换数据库 (DB 0-15)
- **数据持久化**：连接配置本地保存，密码存储在系统钥匙串中，重启后不丢失

## 界面预览

应用采用左右分栏布局：
- **左侧边栏**：连接列表，管理所有已保存的缓存连接
- **右侧工作区**：
  - 上部：键名扫描工具栏和键列表
  - 下部：键值编辑器，显示选中键的详细信息

## 系统要求

- macOS 10.15+
- 已安装 Redis 或 Memcached 服务

## 安装使用

### 方式一：直接运行

构建完成后，在 `src-tauri/target/release/bundle/macos/` 目录下找到 `Cached GUI.app`，双击或在终端中打开：

```bash
open "src-tauri/target/release/bundle/macos/Cached GUI.app"
```

### 方式二：安装 DMG

构建完成后，在 `src-tauri/target/release/bundle/dmg/` 目录下找到 DMG 安装包并打开：

```bash
open "src-tauri/target/release/bundle/dmg/"*.dmg
```

## 操作指南

### 添加连接

1. 点击左侧边栏底部的 **"+"** 按钮
2. 填写连接信息：
   - **名称**：自定义连接名称（如 "本地 Redis"）
   - **引擎**：选择 `Redis` 或 `Memcached`
   - **主机**：服务器地址（默认 `localhost`）
   - **端口**：Redis 默认 `6379`，Memcached 默认 `11211`
   - **用户名/密码**：如需认证请填写
   - **数据库**：Redis 可选 0-15，Memcached 填 0
   - **超时时间**：连接超时（毫秒）
3. 点击 **"保存"** 或 **"测试连接"** 验证连通性

### 浏览键值

1. 在左侧点击选择一个已保存的连接
2. 在顶部输入框输入 SCAN 模式（如 `user:*`、`cache:*`）
3. 点击 **"扫描"** 按钮获取匹配的键列表
4. 点击列表中的键名，下方将显示该键的详细信息

### 读取指定键

在顶部输入框输入精确的键名，点击 **"读取"** 按钮直接查看。

### 新建键值

1. 点击 **"新建键"** 按钮
2. 填写键名、值和 TTL（可选，单位秒）
3. 点击 **"保存"**

### 编辑键值

1. 在键列表中点击选中一个键
2. 在下方编辑器中修改值或 TTL
3. 点击 **"保存"**

### 删除键值

1. 选中要删除的键
2. 点击 **"删除"** 按钮

### 切换数据库

对于 Redis 连接，可在工具栏的 DB 输入框中输入 0-15 之间的数字，切换后自动刷新键列表。

## 配置说明

| 参数 | 说明 | 默认值 |
|------|------|--------|
| 名称 | 连接标识名 | - |
| 引擎 | Redis / Memcached | Redis |
| 主机 | 服务器地址 | localhost |
| 端口 | 服务端口 | 6379 (Redis) / 11211 (Memcached) |
| 用户名 | 认证用户名 | - |
| 密码 | 认证密码（存储在系统钥匙串，不会写入配置文件） | - |
| 数据库 | DB 编号 | 0 |
| 超时时间 | 连接超时（毫秒） | 5000 |

## 快捷键

| 功能 | 快捷键 |
|------|--------|
| 新建连接 | `Cmd + N` |
| 新建键 | `Cmd + K` |
| 保存 | `Cmd + S` |
| 删除 | `Delete` |
| 刷新 | `Cmd + R` |

## 故障排除

### 连接失败

1. 确认 Redis/Memcached 服务已启动
2. 检查主机地址和端口是否正确
3. 检查防火墙设置
4. 如需认证，确认用户名密码正确

### 扫描不到键

1. 确认 SCAN 模式语法正确（支持通配符 `*`）
2. 确认键确实存在于当前数据库
3. 尝试使用精确键名直接读取

## 技术栈

- **前端**：Svelte + TypeScript + Vite
- **后端**：Rust + Tauri
- **图标**：App Icon

## CI / 发布

推送符合 `v*` 的 git tag（例如 `v0.1.0`）时，GitHub Actions 会编译并打包：

| 平台 | Runner | 产物 |
|------|--------|------|
| macOS Apple Silicon | `macos-latest` | DMG 等 |
| macOS Intel | `macos-latest` | DMG 等 |
| Linux x64 | `ubuntu-22.04` | Deb / AppImage / RPM |
| Windows x64 | `windows-latest` | MSI / NSIS |

也可在 Actions 页手动运行 **Release** workflow；填写已有 tag 即可写入对应 Release，留空则只上传 workflow artifacts。

### 触发示例

```bash
git tag v0.1.0
git push origin v0.1.0
```

### 仓库 Secrets（macOS 签名与公证）

在 GitHub 仓库 **Settings → Secrets and variables → Actions** 中配置：

| Secret | 说明 |
|--------|------|
| `APPLE_CERTIFICATE` | Developer ID Application 证书导出的 `.p12` 的 base64（`openssl base64 -A -in cert.p12 -out cert.txt`） |
| `APPLE_CERTIFICATE_PASSWORD` | 导出 `.p12` 时设置的密码 |
| `KEYCHAIN_PASSWORD` | CI 临时 keychain 密码（可自拟随机字符串） |
| `APPLE_ID` | Apple ID 邮箱（公证用） |
| `APPLE_PASSWORD` | [App 专用密码](https://support.apple.com/HT204397)（非登录密码） |
| `APPLE_TEAM_ID` | Apple Team ID（开发者账号 Membership 页） |

可选（与 Apple ID 公证二选一，使用 [App Store Connect API Key](https://developer.apple.com/documentation/appstoreconnectapi/creating_api_keys_for_app_store_connect_api)）：

| Secret | 说明 |
|--------|------|
| `APPLE_API_KEY` | Key ID |
| `APPLE_API_ISSUER` | Issuer ID |
| `APPLE_API_KEY_P8` | 下载的 `.p8` 私钥全文 |

未配置 `APPLE_CERTIFICATE` 时仍会构建 macOS 包，但不会正式签名/公证；Apple Silicon 用户从浏览器下载后可能被系统提示损坏。

工作流文件：`.github/workflows/release.yml`。

## 许可证

MIT License
