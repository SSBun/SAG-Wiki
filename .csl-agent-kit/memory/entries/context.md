---
title: 项目基础信息
summary: SAG-Wiki 的本地运行入口与服务隔离条件、组件职责、文档折叠链路及模型协议边界。
---

- [2026-11-10] SAG-Wiki 的文档折叠阅读界面位于 `src/components/DocumentFold.svelte`；该 Svelte 组件调用 `fold_document` 生成阅读层级，以 `/textfold/reader.html` iframe 承载阅读器，通过 `sag-fold-*` 消息交互；原文滚动容器为 `.document-sheet`。
- [2026-11-10] 文档折叠处理链路：`src/lib/document-fold.ts` 从当前编辑器 DOM 按 H1～H6 分章，为段落、列表项和图片说明分句编号；表格作为章节上下文，代码块及 Mermaid 不作为普通段落概括，原 HTML 保留。`core/src/document_fold.rs` 使用 `settings.answer` 回答模型，按运行时 `fold_part_units` 分批、`fold_concurrency` 并发生成段落与章节结构化概括；遇上下文或输出长度限制会拆半重试，多批或拆分章节另作汇总；`core/src/document_fold/ladder.rs` 负责整理阅读层级结果，交给 TextFold 阅读器使用。
- [2026-11-10] SAG-Wiki 的“设置 → 模型设置”支持配置 OpenAI 问答模型：填写 API Key 并逐行填写支持 Chat Completions 和工具调用的模型 ID；当前聊天接入使用 Chat Completions，不支持仅提供 Responses 接口的模型。Jev 模型新增 OpenAI 供应商，默认端点为 `https://api.openai.com/v1`、协议为 OpenAI Decisions、模型为 `gpt-6-luna`；决策接口协议可选 OpenAI Decisions 或 Typesafe Noul，旧配置继续使用原有协议。
- [2026-11-10] 会话对 `core/src/models.rs` 的实现说明：Jev 的 OpenAI Decisions 请求使用 `POST /v1/decisions`，发送 `model`、字符串 `input`（原始问题与已有证据的 JSON 文本）及 `questions` 数组；每个候选片段对应一个 `type=predicate` 问题，`name` 为候选 ID，字符串 `instructions` 包含候选标题、正文及相关性标准。`input` 替代 Typesafe 的 `state`，评分读取 `answers[].probability` 而非 `noul`；实现检查每批不超过 400 个问题，响应校验数量、顺序、名称、类型及概率在 `[0,1]` 内，拒绝或异常响应报错。该请求不附带聊天参数 `messages`、`temperature`、`max_tokens`、`reasoning_effort`；当前未实现 `choice`、`score` 题型，也未开放 `prompt_cache_key`、`prompt_cache_options` 等可选参数。这是现有实现的范围说明，不是新增功能请求或所有 Decisions 能力均受支持的结论。
- [2026-11-10] 本地完整桌面开发入口为仓库根目录的 `npm run tauri dev`，自动构建开发 CLI、启动 Vite 并打开 Tauri 应用；新克隆先执行 `npm ci`。`npm run dev` 仅提供浏览器前端，缺少完整本地文件、检索及模型后端。Mac 上可用 `SAG_DATA_DIR="$HOME/Library/Application Support/SAG-Wiki-Dev" npm run tauri dev` 指定独立开发目录，保存开发配置、索引和模型密钥，首次需重新添加文件夹和配置模型；正式版服务已占用默认数据目录时，开发版 MUST 使用独立目录，后续启动 MUST 持续指定同一开发目录以避免连接到正式版服务。这一条件下隔离目录不是可选项。会话根据用户截图报告编译成功但出现“SAG 服务构建类型为 release，当前客户端为 development”；建议 Ctrl+C 后带上述环境变量重启，尚无重启成功或冲突消除的反馈。
