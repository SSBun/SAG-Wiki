# Review round 01

- Schema: task-review-report/v1
- Task: provider-thinking-level
- Round: 01
- Snapshot: snapshot:tasks/artifacts/provider-thinking-level/evidence/06b2f3c492a8d0786952cd5f09a8be70fda16aa147d018197842d2435f793708.json#06b2f3c492a8d0786952cd5f09a8be70fda16aa147d018197842d2435f793708
- Task fingerprint: 5078c5884b18e6d7185dec6633a669fae30754c48f4167bb1b47b241bc07011f
- Outcome: findings
- Mode: independent subagent
- Run: 5363a435-0dba-4e44-8632-559c567ef787

## Reviewer feedback

Review mode: independent subagent（新鲜上下文、只读审查）。

Reviewed scope / snapshot:
本轮核对了 provider-thinking-level 的 T1／T2、供应商页新增入口、共享 profile、前后端枚举、能力表、连接测试与实际问答调用链、四份中英文文档、六张截图及四份探针源和输出。已区分上一任务的 OpenAI Decisions 改动，未将其整体归入本任务。

快照标识：snapshot:tasks/artifacts/provider-thinking-level/evidence/06b2f3c492a8d0786952cd5f09a8be70fda16aa147d018197842d2435f793708.json#06b2f3c492a8d0786952cd5f09a8be70fda16aa147d018197842d2435f793708

Findings:

- **TR-1 — P1 / Concern：非 OpenAI 官方地址的连接测试忽略所选思考档位。**
  - **位置**：[Settings.svelte](/Users/caishilin/Desktop/personal/SAG-Wiki/src/components/Settings.svelte:106)、[models.rs](/Users/caishilin/Desktop/personal/SAG-Wiki/core/src/models.rs:798)。
  - **问题**：新增入口说明“测试连接使用此模型及档位”，前端也确实传入 profile 的 reasoning_effort；但后端仅在主机为 api.openai.com 时调用共享 payload。其他聊天地址仍构造固定请求，仅包含 model、messages 和 max_tokens，丢弃所选强度。
  - **证据**：models.rs 第 798—804 行中，官方 OpenAI 分支使用 payload(config, …)，其他聊天分支直接使用 json!。后者不会应用既有 Qwen 的 enable_thinking／thinking_budget 或 DeepSeek 的 reasoning_effort，也不会拒绝已保存但当前服务不支持的档位。
  - **影响**：用户在本次新增入口选择已有适配模型的档位后，“测试连接”成功并不能验证该档位；在不支持该档位的新地址上也可能测试成功，而实际问答随后失败。该问题由新增入口和连接测试承诺变得可达，影响 T1 的连接验证行为及 T2 的兼容性。
  - **最小修复建议**：所有聊天连接测试均复用共享 payload 构造模型参数，再按现有端点约束设置输出上限；保持 Decisions 分支独立。通过获授权的非测试请求检查，核对 Qwen、DeepSeek、默认值及不支持档位的构造结果，无需新增外部 API 调用。

Unverified risks:

- **快照字节身份尚未独立核验**：当前工具不能计算 SHA-256 或运行 check-evidence。审查前后重读的清单一致，重复查看的 Settings 与能力表 diff 也一致，但这不能证明全部 23 个文件与清单逐字节一致。主会话仍需补核验；在补齐前，不能把本轮结论认定为已绑定到该快照。该项阻塞最终 T1／T2 证据绑定，不代表已发现内容漂移。
- **原生完整链路未验证**：作者的浏览器验证使用真实组件和模拟 Tauri 桥；实际 Rust 探针单独验证请求。该限制已披露，对本次限定目标本身不新增阻塞，但不能据此声称原生端到端流程已验证。
- 官方文档及外部 API 未由本审查再次访问。四档接受与 max 拒绝属于作者实测证据；本轮未独立重跑。该限制不新增阻塞。

Checks performed:

- 核对新增供应商入口和问答设置确实读写同一 model_profiles，保存流程保留 profile 其他字段。
- 核对实际问答在 service.rs 第 749—752 行按请求模型加载强度，核心 payload 按共享 catalog 映射。
- 核对 Sol 仅匹配官方域名，提供 low／medium／high／xhigh；默认分支不添加 reasoning_effort，原有其他模型 max 保留。
- 核对 Rust Xhigh 的 snake_case 序列化、前端类型及双语标签。
- 查看六张截图，确认新增入口与双语显示，未见个人信息或凭据。
- 核对四份文档对默认值、max 实测拒绝、Decisions 独立及 Responses 工具限制的说明。
- 未编辑文件、变更任务、运行测试套件或调用外部 API。

Author evidence examined:

- 任务记录中的 Target、Result、Execution Log 与 Review Assessment。
- thinking_level_probe.rs 和 live-probe.log：记录四档返回 OK、max 返回明确 HTTP 400。
- thinking_request_inspect.rs 和 request-mapping.log：记录四档参数映射及默认省略参数。
- 浏览器交互、类型检查、前端／Rust／文档构建及页面搜索结果仅核读作者记录，未独立重跑。
- live probe 包含 max，最终能力表已移除 max，因此该探针是历史验证源，不能直接视为当前能力表下可完整重跑的成功程序；独立 request-mapping 证据覆盖了最终四档和默认值。

Coverage limits:
本轮聚焦本任务相关改动及必要调用方，未扩展审计 Decisions、代理协议或全部工具能力。初始 ledger 为空；新增 TR-1 需要主会话核实并修复。本反馈不授予完成批准。