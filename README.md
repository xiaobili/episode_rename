# OpenList TUI

一个基于 Rust 的终端用户界面应用程序，用于通过 OpenList API 批量重命名视频文件。采用 MVVM/Elm 风格架构，实现清晰的职责分离和可维护的代码结构。

## 功能特性

### 核心功能
- **交互式终端界面** - 基于 ratatui 的美观 TUI 界面
- **OpenList API 集成** - 完整的 OpenList 服务端点支持
- **目录浏览** - 交互式导航 OpenList 文件系统
- **批量重命名** - 支持多种重命名模式的批量操作
- **剧集编号识别** - 自动从文件名中提取剧集信息

### 重命名模式
- **智能重命名** - 自动识别文件名中的剧集信息并标准化
- **手动重命名** - 逐个为文件指定新名称
- **统一样式** - 为所有文件使用相同模式，自动递增集数
- **正则替换** - 使用正则表达式进行高级重命名

### 剧集格式支持
支持多种常见的剧集编号格式：
- `S01E01` / `s01e01`
- `1x01` / `1X01`
- `EP01` / `ep01` / `Ep01`
- `第1集` / `第01话`
- `1 of 10` 格式

### 用户体验
- **异步操作** - 基于 Tokio 的异步处理，非阻塞 UI
- **进度指示** - 操作过程中的可视化加载状态
- **预览确认** - 重命名前预览变更
- **错误处理** - 完善的异常处理和错误提示

### 配置管理
- **Token 持久化** - 登录后自动保存 JWT 令牌
- **自动登录恢复** - 启动时自动检测并使用有效的本地令牌
- **配置持久化** - 服务地址等配置自动保存

## 技术栈

- **语言**: Rust 2021 Edition
- **终端 UI**: ratatui 0.29 + crossterm 0.28
- **异步运行时**: Tokio 1
- **HTTP 客户端**: reqwest 0.12 (rustls-tls)
- **序列化**: serde + serde_json
- **正则表达式**: regex 1

## 安装

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/your-username/openlist-tui.git
cd openlist-tui

# 构建 release 版本
cargo build --release

# 可执行文件位于
./target/release/openlist-tui
```

### 运行要求
- Rust 工具链 (rustc, cargo)
- 支持 Unicode 的终端 (推荐使用 Nerd Font 字体以获得最佳图标显示)
- 到 OpenList 服务器的网络连接

## 使用方法

### 启动程序

```bash
./target/release/openlist-tui
```

### 首次运行

1. 程序启动后会显示登录界面
2. 输入 OpenList 服务地址（如 `http://192.168.1.1:5244`）
3. 输入用户名和密码
4. 登录成功后，令牌会自动保存

### 目录导航

- 使用方向键或 `j`/`k` 在文件列表中移动
- 按 `Enter` 进入选中的目录
- 按 `Backspace` 返回上级目录
- 按数字键快速跳转到对应项

### 重命名操作

1. 在文件列表中选中要重命名的文件
2. 按 `r` 打开重命名模式选择
3. 选择重命名模式：
   - **智能**: 自动解析剧集信息
   - **手动**: 逐一指定新名称
   - **统一**: 统一格式，递增集数
   - **正则**: 使用正则表达式替换
4. 预览变更后确认执行

## 配置

### 配置文件位置

配置文件保存在平台特定的配置目录：

- **Linux/macOS**: `~/.config/openlist-tui/config.json`
- **Windows**: `%APPDATA%\openlist-tui\config.json`

### 配置文件结构

```json
{
  "server_url": "http://192.168.1.1:5244",
  "username": "your_username",
  "token": "your_jwt_token"
}
```

### 默认设置

- 默认服务器地址: `http://192.168.1.1:5244`
- 支持的视频格式: `.mp4`, `.mkv`, `.avi`, `.mov`, `.wmv`, `.flv`, `.webm`, `.m4v`, `.mpg`, `.mpeg`, `.ts`, `.m2ts`

## 项目结构

```
openlist-tui/
├── src/
│   ├── api/              # API 客户端
│   │   ├── client.rs     # OpenList HTTP 客户端
│   │   └── types.rs      # 请求/响应类型定义
│   ├── components/       # UI 组件
│   │   ├── directory_list.rs
│   │   ├── file_list.rs
│   │   ├── login_dialog.rs
│   │   ├── error_popup.rs
│   │   ├── rename/       # 重命名相关组件
│   │   └── style.rs      # 样式定义
│   ├── models/           # 数据模型
│   │   ├── episode.rs    # 剧集解析
│   │   └── file.rs       # 文件/目录模型
│   ├── state/            # 状态管理
│   │   ├── navigation.rs # 导航状态
│   │   ├── auth.rs       # 认证状态
│   │   ├── rename.rs     # 重命名状态
│   │   └── ui.rs         # UI 状态
│   ├── message/          # 消息定义
│   ├── update.rs         # 状态更新函数
│   ├── app.rs            # 应用入口
│   └── main.rs           # 主程序入口
├── Cargo.toml
└── README.md
```

## 架构设计

项目采用 **MVVM/Elm 混合架构**，核心特点：

- **单向数据流**: 所有状态变更通过消息机制触发
- **集中状态管理**: 应用状态集中在 `App` 结构体中
- **组件化渲染**: UI 按组件拆分，支持状态切片
- **可测试性**: 业务逻辑与 UI 解耦，便于单元测试

### 数据流

```
用户输入 -> Message -> update() -> 新状态 -> 渲染
                ^
                |
            异步任务
```

## 开发

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_episode_parsing

# 运行测试并显示输出
cargo test -- --nocapture
```

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 2021 Edition 标准

### 添加新的重命名模式

项目架构设计支持轻松扩展新的重命名模式：

1. 在 `src/state/rename.rs` 添加新模式状态
2. 在 `src/message/rename.rs` 定义新消息
3. 在 `src/update.rs` 添加处理逻辑
4. 在 `src/components/rename/` 添加 UI 组件

## 故障排除

如遇到问题，请检查：

1. OpenList 服务是否正常运行
2. 网络连接是否稳定
3. 用户名和密码是否正确
4. 配置文件格式是否正确
5. 终端是否支持 Unicode 字符

## 许可证

本项目采用 MIT 许可证。详情请参阅 LICENSE 文件。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目！

---

*享受整洁有序的视频收藏！*
