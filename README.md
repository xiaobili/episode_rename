# OpenList 批量剧集重命名工具

这个工具可以帮助您使用 OpenList API 批量重命名剧集文件。

## 功能

- 批量重命名视频文件
- 智能识别剧集信息（如季数、集数、标题）
- 支持自定义命名模式
- 试运行模式（预览重命名结果，不实际执行）
- 支持多种视频格式
- 交互式目录导航
- 多种重命名模式（智能、手动、正则替换）

## 依赖

- Python 3.6+
- requests 库

安装依赖：
```bash
pip install requests
```

## 使用方法

### 1. 基础批量重命名

使用 `bulk_rename_episodes.py` 脚本：

### 2. 交互式批量重命名

使用 `interactive_episode_renamer.py` 脚本：

1. 运行脚本：
   ```bash
   python interactive_episode_renamer.py
   ```

2. 按提示输入服务器地址、用户名和密码

3. 登录后，您可以：
   - 浏览和导航目录
   - 查看目录中的文件
   - 选择不同的重命名模式：
     * 智能重命名：自动识别剧集信息并应用命名模式
     * 手动重命名：为每个文件单独指定新名称
     * 统一命名：为所有文件使用相同模式，递增集数

### 3. 高级批量重命名

使用 `advanced_episode_renamer.py` 脚本：

1. 修改脚本中的配置信息：
   ```python
   BASE_URL = "https://fox.oplist.org.cn"  # OpenList 服务地址
   USERNAME = "your_username"               # 您的用户名
   PASSWORD = "your_password"               # 您的密码
   DIRECTORY_PATH = "/path/to/episodes"     # 要重命名的目录路径
   ```

2. 定义重命名映射：
   ```python
   episode_mapping = {
       "old_name1.mkv": "new_name1.mkv",
       "old_name2.mkv": "new_name2.mkv",
       # 添加更多映射...
   }
   ```

3. 运行脚本：
   ```bash
   python bulk_rename_episodes.py
   ```

### 2. 高级智能重命名

使用 `advanced_episode_renamer.py` 脚本：

1. 修改配置信息（同上）

2. 选择重命名方式：
   - 智能重命名：自动识别剧集信息并应用命名模式
   - 自定义重命名：使用您提供的映射

3. 运行脚本：
   ```bash
   python advanced_episode_renamer.py
   ```

## API 说明

### 认证
- 端点：`/api/auth/login`
- 方法：POST
- 请求体：`{"username": "your_username", "password": "your_password"}`
- 响应：JWT 令牌

### 批量重命名
- 端点：`/api/fs/batch_rename`
- 方法：POST
- 请求头：`Authorization: Bearer {jwt_token}`
- 请求体：
  ```json
  {
    "src_dir": "/path/to/directory",
    "rename_objects": [
      {
        "src_name": "old_filename.ext",
        "new_name": "new_filename.ext"
      }
    ]
  }
  ```

## 命名模式

支持以下命名模式变量：
- `{season}`: 季数（2位数字）
- `{episode}`: 集数（2位数字）
- `{title}`: 剧集标题

示例模式：
- `"S{season}E{episode:02d}_{title}"` → `S01E01_剧集标题.mkv`
- `"{title}_Season_{season}_Episode_{episode}"` → `剧集标题_Season_01_Episode_01.mkv`

## 注意事项

1. **试运行模式**：在实际执行重命名前，建议先使用试运行模式预览结果
2. **备份**：在执行批量重命名前，请备份您的文件
3. **权限**：确保您有目录的写入权限
4. **文件名冲突**：确保目标文件名不会与现有文件冲突
5. **网络连接**：确保可以访问 OpenList 服务

## 常见问题

### 登录失败
- 检查用户名和密码是否正确
- 确认 OpenList 服务是否可用

### 重命名失败
- 检查目标目录是否存在
- 确保文件名不包含非法字符
- 确认目标文件名没有冲突

### 智能识别不准确
- 可以使用自定义映射方式精确控制重命名
- 文件名应包含季数、集数等信息以便识别

## 支持的视频格式

- MP4, MKV, AVI, MOV, WMV, FLV, WEBM, M4V, MPG, MPEG, TS, M2TS, VOB, ISO

## 交互式脚本特性

交互式脚本 (`interactive_episode_renamer.py`) 提供了完整的目录浏览和文件管理功能：

- 交互式目录导航
- 可视化显示目录结构
- 多种重命名模式（智能、手动、统一命名）
- 实时预览重命名结果
- 安全确认机制避免误操作

### 重命名模式说明

1. 智能重命名：自动分析现有文件名，识别剧集信息并应用标准命名模式
2. 手动重命名：逐个为文件指定新名称
3. 统一命名：为所有文件使用相同的命名规则，自动递增集数

## 贡献

欢迎提交问题和功能请求！