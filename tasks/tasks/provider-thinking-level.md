# 模型供应商配置中增加思考强度选项

| Field | Value |
| --- | --- |
| Status | Completed (2026-11-11 15:17) |
| Artifacts | [Artifacts](../artifacts/provider-thinking-level/) |
| Kind | Task |

## Target
- [x] T1: 模型设置中的问答模型配置可选择并保存思考强度，实际聊天请求使用所选参数。
- [x] T2: 支持已核实的 OpenAI 聊天模型与档位，现有模型配置行为保持兼容，中英文文档同步。

## Plan

1. 独立审查当前思考强度入口、模型参数与配置共享行为。
2. 按实际发现修正并完成最终快照和交付核验。

## Execution Log

### 001 — 需求与参数核对

用户确认模型供应商页增加思考强度、按聊天模型保存、与问答设置共享并补充 gpt-6.1-sol 适配。现有改动属于上一项 OpenAI 接入任务，保留其内容，仅修改本次所需区域。已读取官方 gpt-6.1-sol 模型页及 Chat Completions 参数说明：该模型支持 low、medium（默认）、high、xhigh、max；聊天端点参数为 reasoning_effort。模型页另注明工具调用需要 Responses API，本次不扩展聊天协议，文档和交付必须说明此现有限制。新增 xhigh 类型和标签以准确表达档位，不把 xhigh 与 max 合并。

### 002 — 实际接口修正及入口实现

通过临时 Rust 示例调用现有 prepare_answer／answer_with_runtime，明确直连官方 API，仅发送“Reply exactly OK.”：模型元数据 HTTP 200；low／medium／high／xhigh 均返回 OK，输入均 89 token，输出分别 4／19／31／42 token。max 返回 HTTP 400，错误明确列出仅支持 low、medium、high、xhigh。因此按实际结果只在 Sol 能力表提供四档，不自动重试失败请求；原有其他模型 max 档位保留。失败探针源与日志将保存为证据。

模型供应商页已增加配置模型选择及 ReasoningControl，直接读写既有 model_profiles；问答设置与输入框沿用同一份数据。连接验证改用当前配置模型和档位。前端类型检查、Rust core 全目标编译通过；中英文模型与问答说明已补充档位、默认含义及 Responses 工具限制，截图与浏览器行为验证待完成。

### 003 — 验证与文档同步

使用真实 Settings 组件及模拟 Tauri 桥在 ego-browser 中完成手动交互：供应商页选择 xhigh 后问答设置显示“很高”，测试请求带当前模型和 xhigh；保存回调保留其他 profile 字段，重挂载组件后仍显示 xhigh。切换 gpt-4.1-mini 时只显示默认且无 slider，再切回 Sol 保留 xhigh；恢复默认后测试请求和保存值均为 null。Decisions 配置保持独立。浏览器验证不是原生完整链路，未修改用户设置或使用真实密钥。

另用本地 Rust 请求检查示例读取 ModelProfile 并构造实际 prepare_answer 请求：四个显式档位逐一映射为同名 reasoning_effort，默认配置不序列化 profile 的强度字段且请求不含 reasoning_effort。证据位于 ../artifacts/provider-thinking-level/evidence/，包含实际 API 探针（max 失败）及本地映射检查的源和输出。未再次发送已验证的模型请求。

npm run check、npm run build、cargo check --manifest-path core/Cargo.toml --all-targets、npm --prefix docs-site run build 均通过。Rust 有既存 dead_code 警告，文档有既存 highlight CSS 警告。未运行任何单元测试或项目测试套件。临时预览用 1431 端口避开用户已占用的 1420，未停止用户服务。

中英文 models／chat 四个页面和六张截图已同步。生产服务核验四页面 HTTP 200 且含 xhigh，中文“思考强度”和英文 reasoning 搜索均命中 models／chat，新增中英文图片均 HTTP 200。未打包安装或操作原生桌面，未增加 Responses 工具调用支持。

### 004 — 修复审查发现 TR-1

首轮审查确认非 OpenAI 的聊天连接验证未应用前端所选强度。已将所有聊天验证统一使用实际聊天的 payload 构造，Decisions 保持独立；OpenAI 保留 max_completion_tokens 上限，其他服务不再施加与思考预算冲突的固定 8-token 上限。

使用临时本地连接请求检查示例，Client 的域名解析强制指向 loopback 并禁用代理，未访问外部 API：捕获 Qwen High 的 enable_thinking=true／thinking_budget=8192、Qwen Default 的 enable_thinking=false、DeepSeek High 的 reasoning_effort=high、DeepSeek Default 省略 reasoning_effort；不支持的模型档位在网络发送前报现有未适配错误。源及输出见 evidence/connection_payload_inspect.rs 与 connection-payload.log。Rust 全目标编译和最终文档构建通过，git diff --check 无误。TR-1 待新鲜 Reviewer 复核；无无进展轮次。

## Task Summary

**模型设置界面**：[src/components](../../src/components/) 的供应商页增加按模型配置思考强度入口，共用问答设置的 profile；连接验证使用当前配置对象及档位。

**模型能力与配置类型**：[src/lib](../../src/lib/) 增加官方 OpenAI gpt-6.1-sol 四档能力映射和中英文提示；[protocol](../../protocol/) 增加 xhigh 枚举值，沿用既有序列化与请求构造。

**聊天连接验证**：[core/src](../../core/src/) 的模型服务统一用实际聊天参数构造验证请求，使既有 Qwen、DeepSeek 等供应商也验证所选档位。

**使用文档**：[docs-site](../../docs-site/) 同步中英文模型及问答说明、供应商及问答设置截图，说明 max 实测拒绝和 Responses 工具边界。

## Self Assessment

| Dimension | Grade | Basis and evidence | Limitations |
| --- | --- | --- | --- |
| 简洁性 | S | 复用 ReasoningControl 与 model_profiles，不新增配置来源或请求适配分支；仅补充共享能力表与枚举。 | 未发现不必要抽象或依赖。 |
| 稳定性 | A | 四档经实际 Rust 模型调用成功；浏览器验证跨页共享、保存重开、模型切换与恢复默认；本地捕获 Qwen／DeepSeek 显式与默认连接参数并确认不支持档位报错；Rust 映射检查和类型／构建通过。 | 未运行测试套件，未验证原生完整 UI；官方模型页与端点 max 行为不一致，已按实际拒绝结果排除并说明。 |
| 完整性 | A | 新入口、实际参数映射、共享配置、中英文说明及截图均完成，见 T1／T2。 | 仅适配官方 OpenAI 域名的 gpt-6.1-sol；不推断代理能力，不扩展 Responses 工具支持，均已说明。 |

## Result

- T2: 官方直连模型元数据 HTTP 200，gpt-6.1-sol 的 low／medium／high／xhigh 通过实际 Rust 文本生成均返回 OK；max HTTP 400 被拒，能力表因此不提供 max，未自动重试。源和日志见 ../artifacts/provider-thinking-level/evidence/。中英文 models／chat 文档和六张截图同步，文档构建通过，四页面及新图片 HTTP 200，双语搜索均命中受影响页面。原有 Qwen／DeepSeek 能力未改动；Responses 工具支持和代理兼容性不在本次范围。
- T1: 真实组件＋模拟 Tauri 浏览器验证跨页共享、测试选中模型和档位、保存重开、模型切换及默认清除；Rust 请求检查确认四档映射及默认省略。修复 TR-1 后，本地强制 loopback 的实际 test_connection 请求捕获证明 Qwen High／Default 与 DeepSeek High／Default 应用正确参数，不支持档位发送前报错，见 evidence/connection-payload.log。前端类型／构建与最终 Rust 全目标编译通过；未跑测试套件或原生完整 UI。
- Review gate: Passed — round-02 新鲜只读 Reviewer 40186323-a155-4221-b4eb-aec66d38cb55 确认 TR-1 resolved，未发现新问题。主会话 check-evidence 验证25文件快照和两份报告摘要均 verified，已补齐审查工具无法计算哈希的内容身份限制；原生UI及所有模型工具能力未验证的边界保留。
## Review Assessment

- Schema: task-review-assessment/v1
- Level: R2
- State: Current
- Reason: 修复 TR-1 后重新核对跨入口配置、持久化及所有聊天供应商的连接参数路径。
- Snapshot: snapshot:tasks/artifacts/provider-thinking-level/evidence/1008dc59adaf6bb7472c7ffe3a441eff99a9102e732d3a22e269c2ba95dbf7bd.json#1008dc59adaf6bb7472c7ffe3a441eff99a9102e732d3a22e269c2ba95dbf7bd
- Evidence: 当前 T1／T2、Execution Log 002—004，evidence 内三份探针源及三份日志；TR-1 修复经 loopback 实际请求捕获，编译通过。
- Task fingerprint: 1c301eac48fb4dfab5f4518cad71d8e20cf458dc6debbe8b376f19a3722da291
## Review Reports

- Schema: task-review-reports/v1

| Round | Report | Snapshot | Outcome | SHA256 |
| --- | --- | --- | --- | --- |
| 01 | [Round 01](../artifacts/provider-thinking-level/reviews/round-01.md) | snapshot:tasks/artifacts/provider-thinking-level/evidence/06b2f3c492a8d0786952cd5f09a8be70fda16aa147d018197842d2435f793708.json#06b2f3c492a8d0786952cd5f09a8be70fda16aa147d018197842d2435f793708 | findings | d159c467ba239e039f452426a7f3347e12409c8107a4f97000cab838f6d2e3d1 |
| 02 | [Round 02](../artifacts/provider-thinking-level/reviews/round-02.md) | snapshot:tasks/artifacts/provider-thinking-level/evidence/1008dc59adaf6bb7472c7ffe3a441eff99a9102e732d3a22e269c2ba95dbf7bd.json#1008dc59adaf6bb7472c7ffe3a441eff99a9102e732d3a22e269c2ba95dbf7bd | no blocking findings | 30871b3a9f9597aa280c4936270b12496ee85b616b59d5dd06bfb903bf368d64 |

## Verification

- Passed: T1／T2 限定目标完成；复审 TR-1 resolved 且无新发现。审查后 check-evidence 验证当前25文件及两轮报告，git diff --check 无误，产物未漂移。前端类型／构建、最终Rust全目标编译、最终文档构建和双语页面／搜索检查有效，四档真实请求与默认映射、Qwen／DeepSeek本地连接捕获证据完整。自评依据及限制未变化。已清理本次QA页面与1431／3028预览服务，未操作用户1420服务。未运行测试套件、未打包原生桌面，Responses工具边界及max拒绝已披露。
