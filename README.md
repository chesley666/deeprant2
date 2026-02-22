# DeepRant 游戏快捷翻译工具 🎮

<div align="center">
  <img src="src/assets/app-icon.png" alt="DeepRant Logo" width="200"/>
  
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

</div>

## 关于此 Fork

这是 [liseami/DeepRant](https://github.com/liseami/DeepRant) 的一个 Fork，进行了一些改进。

## 🖼️ 功能预览

<div align="center">
  <img src="src/assets/preview.png" alt="DeepRant Preview" width="800"/>
</div>

## 📝 项目简介

DeepRant 是一款专为游戏玩家设计的多语言快捷翻译工具。它能帮助您在国际服务器中快速进行文字交流，让语言不再成为跨服竞技的障碍。

### ✨ 主要特性

- 🚀 **快捷键翻译**：一键快速翻译，无需切出游戏
- 🌍 **多语言支持**：支持全球主流语言之间的互相翻译
- 🎭 **多种翻译模式**：
  - 😈 嘴臭模式：地道的游戏黑话翻译
  - 🎯 专业玩家模式：专业电竞用语
  - 🤖 自动模式：智能识别场景
- 📚 **常用语管理**：可自定义保存常用短语
- 🎛️ **翻译引擎选择**：支持多种主流翻译模型

## 🎯 使用场景

- 跨服竞技对战
- 国际服务器社交
- 多人在线游戏交流
- 电竞比赛实时沟通

## 🚀 快速开始

1. 从 [Releases](https://github.com/liseami/DeepRant/releases) 下载最新版本
2. 安装程序
3. 运行 DeepRant
4. 设置您喜好的快捷键
5. 开始游戏，享受无障碍交流！

## ⌨️ 默认快捷键

- `CMD + T`: 快速翻译


## 💡 使用技巧

1. 在游戏中选中要翻译的文字
2. 按下设定的快捷键
3. 翻译结果会自动复制到剪贴板
4. 在游戏聊天框中粘贴即可

## 🆓 完全免费

DeepRant 目前完全免费使用，我们相信游戏交流应该没有门槛。

## 🛠️ 技术栈

- 🖥️ **跨平台框架**：[Tauri](https://tauri.app/) - 使用 Rust 构建的轻量级跨平台框架
- ⚛️ **前端框架**：
  - React 18
  - Vite
  - TailwindCSS
  - Framer Motion
- 🦀 **后端技术**：
  - Rust
  - Tauri API
  - Global Shortcut
  - Store Plugin
  - Clipboard Manager

## 👨‍💻 开发指南

### 环境要求

- Node.js 16+
- Rust 1.70+
- macOS: Xcode Command Line Tools
- Windows: Visual Studio C++ 构建工具

### 模型配置

DeepRant 支持多个 AI 模型进行翻译。在开发或运行应用前，需要配置模型的 API 密钥。

#### 配置步骤

1. **复制配置文件示例**
   ```bash
   cp .env.example .env
   ```

2. **填入你的 API 密钥**
   编辑 `.env` 文件，替换为你的实际 API 密钥,环境实例中分别适配了3个平台的api：
   ```env
  # DeepSeek 模型（推荐）
  DEEPSEEK_API_KEY=your_api_key_here
  MODELSCOPE_API_URL=https://api-inference.modelscope.cn/v1/chat/completions
  MODELSCOPE_MODEL_NAME=deepseek-ai/DeepSeek-V3.2
   
  # DeepSeek-OCR 模型
  DEEPSEEK_OCR_API_KEY=your_api_key_here
  SILICONFLOW_API_URL=https://api.siliconflow.cn/v1/chat/completions
  SILICONFLOW_MODEL_NAME=deepseek-ai/DeepSeek-OCR
   
  # GLM 模型
  GLM_API_KEY=your_api_key_here
  BIGMODEL_API_URL=https://open.bigmodel.cn/api/paas/v4/chat/completions
  BIGMODEL_MODEL_NAME=glm-4.7-flash
   ```

3. **获取 API 密钥**
   - **DeepSeek**: 访问 [ModelScope](https://api-inference.modelscope.cn/) 获取
   - **DeepSeek-OCR**: 访问 [Silicon Flow](https://www.siliconflow.cn/) 获取
   - **GLM**: 访问 [BigModel](https://open.bigmodel.cn/) 获取

### 打包后的用户配置

发布给用户的已打包应用，用户无需重新编译，直接将 `.env` 文件放在应用目录同级位置即可使用。

#### 配置步骤

1. **准备 .env 文件**

   从源代码中的 `.env.example` 复制，或直接创建 `.env` 文件并填入以下内容：

```env
# DeepSeek 模型（推荐）
DEEPSEEK_API_KEY=your_api_key_here
MODELSCOPE_API_URL=https://api-inference.modelscope.cn/v1/chat/completions
MODELSCOPE_MODEL_NAME=deepseek-ai/DeepSeek-V3.2

# DeepSeek-OCR 模型
DEEPSEEK_OCR_API_KEY=your_api_key_here
SILICONFLOW_API_URL=https://api.siliconflow.cn/v1/chat/completions
SILICONFLOW_MODEL_NAME=deepseek-ai/DeepSeek-OCR

# GLM 模型
GLM_API_KEY=your_api_key_here
BIGMODEL_API_URL=https://open.bigmodel.cn/api/paas/v4/chat/completions
BIGMODEL_MODEL_NAME=glm-4-flash-250414
```

2. **放置 .env 文件**

   - **Windows**: 将 `.env` 放在 `DeepRant.exe` 同级目录
     - 例如：`C:\Users\YourName\Desktop\DeepRant\.env` 和 `DeepRant.exe` 在同一文件夹
   - **macOS**: 将 `.env` 放在应用根目录或 `DeepRant.app` 同级
   - **Linux**: 将 `.env` 放在应用可执行文件同级目录

3. **重启应用**

   应用启动时会自动从应用目录读取 `.env` 文件。应用启动时会在日志中显示是否成功加载。

#### 目录结构示例（Windows）

```
C:\Users\YourName\DeepRant\
├── DeepRant.exe          ← 应用可执行文件
├── .env                  ← 配置文件（与 exe 同级）
└── ...其他文件
```

#### 故障排查

- **未找到 .env 文件**
  - 确保 `.env` 文件与应用可执行文件在同一目录
  - 检查文件名拼写为 `.env`（区分大小写）
  - Windows 用户：在资源管理器中可能看不到 `.env`，可以通过文本编辑器创建或类型检查显示扩展名

- **API 密钥无效**
  - 获取最新的 API 密钥并更新 `.env` 文件
  - 确保没有复制多余的空格或换行符

- **查看加载日志**
  - 应用启动时会输出 `.env` 加载结果
  - 如有错误，日志会显示具体的加载路径和错误信息

### 安装依赖

```bash
# 安装依赖
npm install

# 或使用 yarn
yarn
```

### 开发命令

```bash
# 开发模式
npm run tauri dev

# 清理构建缓存
npm run clean
```

### 打包命令

```bash
# macOS (Intel & Apple Silicon)
npm run tauri build

# macOS (仅 Apple Silicon)
npm run build:mac-arm

# Windows
npm run tauri build
```

## 🤝 贡献指南

欢迎提交 Issue 和 Pull Request！

## 📜 开源协议

本项目采用 MIT 协议开源

---

<div align="center">
  Made with ❤️ for Gamers
</div>
