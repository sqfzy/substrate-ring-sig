# Pallet Ring Sig Voting (匿名环签名投票模块)

这是一个基于 Substrate 框架构建的区块链匿名投票模块。它利用 **可链接环签名 (Linkable Ring Signatures, BLSAG)** 技术，在保护投票者隐私的同时，防止双重投票（Double Voting）。

该项目通过结合链下计算（加密与签名）与链上验证，实现了一个安全、透明且可审计的投票系统。

## 📖 项目简介

`pallet-ring-sig-voting` 旨在为去中心化组织（DAO）或社区提供高隐私保护的治理工具。传统的区块链投票往往会暴露投票者的地址和选择，而本模块通过加密学手段实现了以下目标：

1. **匿名性 (Anonymity)**：通过环签名技术，外部观察者无法确定具体的投票者是谁，只能确定投票者属于某个合法的选民组（Ring）。
2. **不可伪造性 (Unforgeability)**：只有拥有私钥的合法成员才能生成有效签名。
3. **防双花 (Linkability)**：通过“密钥镜像 (Key Image)”机制，系统可以检测并拒绝同一个私钥产生的重复投票，而无需通过解密投票内容。
4. **全民可验证 (Verifiability)**：投票结束后，管理员揭示私钥，任何人都可以解密所有选票并验证统计结果。

## 🌟 核心特性

* **基于 BLSAG 的环签名验证**：集成 `nazgul` 库实现 BLSAG 签名验证。
* **完整的投票生命周期管理**：支持创建、暂停、取消、截止日期设置及结果揭示。
* **链上状态机**：严格的状态流转控制（Active -> Tallying -> Completed/Cancelled）。
* **防双重投票**：使用 `UsedKeyImages` 存储已使用的密钥镜像，杜绝重放攻击。
* **自动调度**：利用 Substrate Scheduler 自动处理投票截止时间。

## 🔄 工作流程 (Workflow)

根据 `docs/poll life.d2` 等文档，系统工作流分为三个阶段：

### 阶段一：系统初始化 (Setup)

1. **注册环 (Register Ring)**：管理员收集选民公钥，将其注册为链上的一个 Ring Group。
2. **创建投票 (Create Poll)**：管理员生成一对临时的投票公私钥。公钥在创建投票时公布，用于选民加密选票；私钥由管理员保管，直到计票阶段。

### 阶段二：匿名投票 (Voting)

1. **链下生成**：选民在本地使用 ECIES 加密投票内容，并结合 Ring 成员公钥生成环签名和 Key Image。
2. **链上提交**：调用 `vote` 接口提交数据。
3. **链上验证**：
* 验证环签名的有效性（证明是成员之一）。
* 检查 Key Image 是否已存在（防止重复投票）。
* 验证通过后，加密选票被存储。



### 阶段三：揭示与验证 (Reveal & Audit)

1. **停止投票**：到达截止时间或管理员手动调用 `tally_poll`，状态变为 `Tallying`。
2. **结果揭示**：管理员在链下解密统计，调用 `tally` 接口提交统计结果和**投票私钥**。
3. **公钥验证**：合约验证提交的私钥是否与创建投票时的公钥匹配。若匹配，状态变为 `Completed`。
4. **全民审计**：由于私钥已公开，任何人都可以从链上获取加密选票并进行解密验证。

## 🛠 接口说明 (Extrinsics)

### 管理员接口 (Admin)

| 方法名 | 参数 | 描述 |
| --- | --- | --- |
| `register_ring` | `ring: Vec<CompressedRistrettoWrapper>` | 注册一个包含多个公钥的环签名组。 |
| `create_poll` | `ring_id`, `description`, `metadata`, `deadline`, `poll_public_key` | 创建新投票，需指定使用的 Ring ID 和用于加密选票的公钥。 |
| `tally_poll` | `poll_id` | 结束投票阶段，进入统计阶段 (Status: Tallying)。 |
| `tally` | `poll_id`, `claimed_tally`, `private_key` | 提交统计结果并揭示私钥。合约会验证 `private_key` 是否匹配。 |
| `cancel_poll` | `poll_id` | 取消投票。 |
| `pause_poll` | `poll_id` | 暂停投票。 |
| `set_deadline` | `poll_id`, `new_deadline` | 更新投票截止区块高度。 |

### 用户接口 (User)

| 方法名 | 参数 | 描述 |
| --- | --- | --- |
| `vote` | `poll_id`, `ephemeral_public_key`, `ciphertext`, `challenge`, `responses`, `key_image` | 提交匿名投票。`key_image` 用于防重，`ciphertext` 是加密后的选票内容。 |

## 📦 存储结构 (Storage)

* `Rings`: 存储环成员公钥列表。
* `Polls`: 存储投票的元数据、状态、公钥以及最终揭示的私钥和结果。
* `EncryptedVotes`: 存储每个投票的加密数据（仅在 `Active` 状态下可写入）。
* `UsedKeyImages`: 记录已使用的密钥镜像，防止双花。

## 🧪 测试与开发

项目包含完整的单元测试，涵盖了从注册环到揭示结果的全流程。

```bash
# 运行所有测试
cargo test

# 运行特定测试 (例如投票流程)
cargo test vote_works

```

主要测试场景包括：

* `register_ring_works`: 环注册测试。
* `vote_works`: 正常投票流程。
* `vote_prevents_double_voting`: 测试重复使用 Key Image 导致的失败。
* `tally_works`: 测试结果揭示与私钥验证。
* `tally_fails_with_wrong_key`: 测试提交错误私钥被拒绝。

## 依赖项 (Dependencies)

* `frame`: Substrate 开发框架。
* `curve25519-dalek`: 椭圆曲线加密操作 (Ristretto Group)。
* `nazgul`: 提供 BLSAG 环签名实现。
* `rand_core`: 随机数生成（主要用于测试）。

## ⚠️ 免责声明

本项目处于开发阶段，使用了复杂的密码学原语。在生产环境使用前，请务必对 `nazgul` 库及本模块代码进行严格的安全审计。
