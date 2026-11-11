---
title: 项目通用经验
summary: 文档折叠的状态与缓存边界，以及模型接入、思考强度适配和相关性评分验证的经验。
---

- [2026-11-10] 在 `DocumentFold.svelte` 中，关闭阅读界面与停止生成是两个独立动作：`start()` 在 `open` 为真时调用 `close()`，清除 `active/open/ready`，经 `tick()` 后仅在原先 `active` 时恢复原文滚动位置；关闭保留 `source/result/job`，生成完成以 `active=open` 避免重新弹出，再次打开时原文 HTML 相同且已有结果便复用。进度条叉号调用 `cancel()`，停止本地进度订阅并请求后端 `cancel_query`；组件销毁也会取消仍在运行的任务。此状态关系可用于判断关闭后继续生成、缓存复用与重新弹出问题；依据为源码核对，未作运行验证。
- [2026-11-10] 排查文档折叠切换模型后未重新生成、失败重试或请求数量时，后端存在两层跨模型磁盘缓存：应用数据目录的 `document-folds/documents/` 保存整篇结果，键由文档结构及提示词版本信息计算；`document-folds/requests/` 按系统提示词、请求提示词及版本计算键，保存通过结构校验的成功请求。两类键均不含模型标识；整篇命中直接返回，批次失败后重试复用已缓存的成功请求。分批数量取运行时配置，源码中的 40 段默认值注释不能代表当前实际值。以上为源码事实，未作运行验证。
- [2026-11-10] Jev 接入 OpenAI Decisions 时，`core/src/models.rs` 将原有相关性问题转换为 predicate 请求，校验返回候选数量、标识顺序及 `probability` 的 0～1 范围。会话报告 Rust 调用链的两个合成片段评分为 0.12、0.00，相关片段排在前面但绝对得分偏低；连接成功与评分映射可用不足以证明原有 Typesafe Noul 阈值适用，也不证明真实文件检索质量。该质量结论的边界适用于跨决策协议验证，尚无真实文件评估结果。
- [2026-11-11] 排查思考强度仅显示“默认”时，SHOULD 先核对 `src/lib/model-capabilities.json` 与 `src/lib/reasoning.ts`：静态能力表要求小写模型 ID 与端点 hostname 同时匹配，未匹配时 `ReasoningControl.svelte` 不显示强度滑块。聊天接通不等于已适配强度选项，界面缺少档位也不足以证明服务不支持推理参数。补充映射前 SHOULD 核对实际端点接受的参数及档位，官方直连与代理接口不能仅按模型名称推定一致。当前表已加入 `gpt-6.1-sol`，仅匹配 `api.openai.com`，显式档位为 `low / medium / high / xhigh`；会话报告文档还列出 `max`，但实际接口对四档均正常回答、对 `max` 返回 HTTP 400 并仅列出前四档，故未提供 `max`。此实测不证明其他端点或未来版本支持范围相同。
- [2026-11-11] `Settings.svelte` 的“模型设置 → 问答模型（LLM）”与“问答设置”共用 `model_profiles[profileModel].reasoning_effort`，按选中聊天模型保存；供应商页连接验证复制配置后，将 `model` 设为 `profileModel`、`reasoning_effort` 设为该模型档位或 `null`。会话报告浏览器验证了两处同步、连接请求携带 `xhigh`、保存重开后保留、恢复默认清除显式档位及切换未适配模型不沿用 Sol 档位。此验证不等于原生桌面完整聊天工具链验证；会话引用官方文档称 Sol 工具调用需要 Responses API，而本次未扩展现有 Chat Completions 协议，不能由强度请求成功推断工具调用可用。
- [2026-11-11] 验证聊天模型思考档位时，SHOULD 让所有供应商的连接测试与实际问答复用参数构造，并通过本地请求捕获核对显式档位、默认值和不支持档位的发送前拒绝行为；仅前端传入配置或仅官方地址验证成功不足以覆盖其他端点。会话报告修复了 `core/src/models.rs` 非 `api.openai.com` 分支固定 JSON 遗漏档位的问题，本地捕获确认 Qwen 携带 `enable_thinking/thinking_budget`、DeepSeek 携带 `reasoning_effort`，默认值保持各自原有语义，不支持档位在发送前报错；该验证未访问外部模型服务，不能据此推断远端接受参数或原生完整聊天工具链可用。
