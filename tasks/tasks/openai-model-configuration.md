# 扩展 OpenAI 聊天与决策模型配置

| Field | Value |
| --- | --- |
| Status | Completed (2026-11-10 18:39) |
| Artifacts | [Artifacts](../artifacts/openai-model-configuration/) |
| Kind | Task |

## Target
- [x] T1: 模型设置页可配置并使用 OpenAI API 的聊天模型和决策模型。
- [x] T2: 对应中文与英文使用文档与已验证行为一致。

## Plan

1. 独立审查当前 OpenAI 协议适配、设置入口与文档；按实际发现修正。
2. 核验最终内容及任务证据，交付改动与验证边界。

## Execution Log

### 001 — 确认接口差异

用户确认决策模型指 OpenAI Decisions API。现有问答已提供 OpenAI 供应商并使用 Chat Completions；Jev 仅实现 Typesafe 的 state／noul 对象协议。已读取 OpenAI 官方 Decisions 文档，确认 input、predicate 问题数组、按顺序返回 probability，当前支持 gpt-6-luna。将为决策配置保存显式协议，避免将 OpenAI 与 OpenRouter／Typesafe 的同名 decisions 路径混用。当前工作区没有已跟踪文件改动；任务运行时使用 tasks/ 旧布局，保持原布局。

### 002 — 实现及验证

已增加显式 decision_protocol（旧配置默认 Typesafe），将 OpenAI predicate 问题及按顺序返回的 probability 接入现有 SAG2 候选评分，验证数量、标识、类型和范围，保留 Noul 路径。设置页新增 OpenAI Jev 预设、协议选择和说明；官方聊天连接验证使用 max_completion_tokens，检查正常结束和非空回复。同步既有供应商测试的选项数量断言，但未运行任何测试套件。

已通过 npm run check、npm run build、cargo check --manifest-path core/Cargo.toml --all-targets 和文档构建。使用现有 OPENAI_API_KEY，明确直连官方地址，只发送合成文本：模型列表 HTTP 200 包含 gpt-4.1-mini 与 gpt-6-luna；通过实际 Rust models::test_connection 验证聊天和 Decisions 均成功；models::score_with_runtime 返回合成相关／无关片段 0.12／0.00。示例源及输出保存于 evidence；模型方法未暴露 HTTP 状态码或用量，不能推断账单或具体 token。该验证只说明协议和排序可用，不验证真实文件准确率或 Noul 阈值等效性。

使用 ego-browser 加载真实 Settings 组件和模拟 Tauri 桥，验证选择 OpenAI 自动填入协议／鉴权／模型、连接请求携带正确配置、保存回调保留配置。截图不含真实密钥或个人资料；已同步中英文回答、Jev 与 OpenAI Decisions 截图。中英文模型页面 HTTP 200、图片 HTTP 200，搜索 OpenAI Decisions 在 zh／en 均命中 models 页面。未打包或启动原生桌面程序，也未用真实用户文件验证完整 SAG2 检索。

## Task Summary

**模型服务与配置协议**：[core](../../core/) 与 [protocol](../../protocol/) 增加 OpenAI Decisions 请求及评分解析、持久化协议选择，并适配官方聊天连接验证参数。

**模型设置界面**：[src](../../src/) 增加决策供应商和协议选项、中英文提示；供应商测试断言随新增选项更新。

**使用文档**：[docs-site](../../docs-site/) 更新中英文模型配置说明与对应截图。

## Self Assessment

| Dimension | Grade | Basis and evidence | Limitations |
| --- | --- | --- | --- |
| 简洁性 | S | 沿用模型配置、现有候选分批、评分和鉴权流程，仅为两种不兼容协议增加显式适配。 | 没有新增依赖或独立推理架构；未发现不必要抽象。 |
| 稳定性 | A | 类型检查、前端构建及 Rust 全目标编译通过；官方聊天和 Decisions 实际请求成功，评分结果可解析；旧配置使用默认 Noul。 | 未运行测试套件；未构建原生桌面包。真实检索分数校准不在本次验证范围，界面与文档已提示。 |
| 完整性 | A | 设置预设、协议保存、连接验证、实际评分路径及中英文文档和截图已覆盖，详见 T1／T2 结果。 | 未验证所有 OpenAI 模型与账号权限；仅支持已有 Chat Completions 端点，Responses-only 模型不支持，文档已说明。 |

## Result

- T1: npm run check 与 npm run build 通过；Rust core 全目标 cargo check 通过。官方模型列表 HTTP 200；实际 Rust 调用链完成 gpt-4.1-mini 聊天连接与 gpt-6-luna Decisions 连接、两候选评分（0.12／0.00），源与输出见 ../artifacts/openai-model-configuration/evidence/。浏览器真实 Settings 组件＋模拟 Tauri 验证 OpenAI 预设、测试请求和保存回调配置。未运行测试套件，未打包原生桌面，未验证真实文件质量或阈值校准。
- T2: 中文与英文 models 页面和六张设置演示图已同步；npm --prefix docs-site run build 成功，存在现有 highlight CSS 警告。本地生产服务核验 zh／en 模型页面及新增图片 HTTP 200；zh／en 搜索 OpenAI Decisions 均返回 7 项且命中 models 页面。文档明确 Chat Completions 范围、当前决策模型、旧配置默认协议和评分校准限制。
- Review gate: Passed — 新鲜只读 reviewer 88a21643-0f01-4307-8e46-77d8324994bb，round-01 无确认问题；报告已保存。主会话在审查后执行 check-evidence，18 文件快照及报告摘要均 verified，补齐 Reviewer 无哈希工具的内容身份核验限制。其余运行覆盖限制已在 T1／T2 明确，不阻塞本次端点与配置适配。
## Review Assessment

- Schema: task-review-assessment/v1
- Level: R2
- State: Current
- Reason: 跨持久化配置、设置界面与外部决策协议的适配；候选评分和错误处理具有兼容性影响，需要独立审查。
- Snapshot: snapshot:tasks/artifacts/openai-model-configuration/evidence/933c72d11e8ace40e068b2dc11a7e71bea710feb374ff5f324032f1bc9340ea7.json#933c72d11e8ace40e068b2dc11a7e71bea710feb374ff5f324032f1bc9340ea7
- Evidence: T1／T2 Result、Execution Log 002 与 evidence/live-probe.log；当前 18 文件快照包含代码、文档、图片及实际验证源和结果。
- Task fingerprint: 7d4260f13bc3f3ce5251d4f2a415c67bc0ed9e3cdf32bcd004043d62434867a6

## Review Reports

- Schema: task-review-reports/v1

| Round | Report | Snapshot | Outcome | SHA256 |
| --- | --- | --- | --- | --- |
| 01 | [Round 01](../artifacts/openai-model-configuration/reviews/round-01.md) | snapshot:tasks/artifacts/openai-model-configuration/evidence/933c72d11e8ace40e068b2dc11a7e71bea710feb374ff5f324032f1bc9340ea7.json#933c72d11e8ace40e068b2dc11a7e71bea710feb374ff5f324032f1bc9340ea7 | no blocking findings | 4fc433e89fe7eca00c44b22178a2d3b52b599d6cc95944b562f460feb0cfadf3 |

## Verification

- Passed: 最终复核 T1／T2 均满足限定接入目标；check-evidence 确认当前 18 文件字节与审查快照一致且 round-01 摘要有效；git diff --check 无错误；最终 docs-site 构建成功。前端类型／构建、Rust 全目标编译及合成文本直连证据有效，无审查后交付物改动。自评依据和限制未变化。已移除临时 QA 页面并关闭本次预览服务与浏览器。未运行测试套件、未打包安装桌面版本，未声称已验证真实文件准确率或所有模型工具调用。
