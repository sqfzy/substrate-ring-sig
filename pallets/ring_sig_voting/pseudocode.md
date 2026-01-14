## 一、 角色定义 (Participants)

* **教务处代表**：系统管理方，负责环(Ring)与投票的生命周期管理。
* **学生会代表**：监督方/参与方。
* **教师工会代表**：监督方/参与方。
* **老师群体**：投票成员。
* **学生群体**：投票成员。

---

## 二、 存储架构 (Storage)

### 1. 链上存储 (On-chain Storage)

* `Rings`: 存储已注册的匿名环成员公钥集合。
* `Polls`: 存储投票详情（状态、截止日期、公钥、结果等）。
* `EncryptedVotes`: 存储所有用户提交的加密投票数据。
* `UsedKeyImages`: 存储已使用的 Key Images，用于防范双花（重复投票）。
* `PollCount`: 投票计数器。
* `RingCount`: 环计数器。

### 2. 链下存储 (Off-chain Storage)

* `Metadata`: 投票的详细描述、规则等富文本信息。
* `PollPrivateKey`: 投票阶段结束前，由创建者或受托人保管的解密私钥。

---

## 三、 密码学约定 (Cryptographic Conventions)

* **加解密关联数据 (AAD)**: `aad = genesis_hash || poll_id || key_image`
* **随机数 (Nonce)**: `nonce = [0; 32]`
* **签名消息格式**: `ephemeral_public_key || ciphertext`
* **环签名实现**: 使用 `nazgul` 库的 **BLSAG** (Back's Linkable Spontaneous Anonymous Group) 算法。
* **开发框架**: 基于 `frame` (Substrate) 核心库。

---

## 四、 核心功能 (Functions)

### 1. 链上调用 (Pallet Calls)

#### `register_ring(admin, ring)`

* **权限**: `ensure_admin`。
* **逻辑**: 校验 Ring 大小合规性，存储至 `Rings`。
* **事件**: 发送 `RingRegistered`。

#### `create_poll(admin, ring_id, description, metadata_hash, deadline, poll_public_key)`

* **权限**: `ensure_admin`。
* **逻辑**: 校验 `deadline`（须晚于当前块）及 `ring_id` 存在性。
* **初始化**:
* Status: `Active`
* PollPrivateKey: `None`
* Tally: `None`


* **存储**: 写入 `Polls`。
* **事件**: 发送 `PollCreated`。

#### `vote(origin, poll_id, ephemeral_public_key, ciphertext, challenge, responses, key_images)`

* **验证**:
* `ensure_signed` 验证发送者。
* 检查 `Poll` 状态是否为 `Active`。
* 校验 `ciphertext` 长度。


* **签名验证**:
* 构造消息: `message = {ephemeral_public_key, ciphertext}`
* 构造签名对象: `signature = {challenge, responses, ring, key_images}`
* 调用 `BLSAG::verify(signature, message)`。


* **防双花**: 检查 `key_images` 是否已存在于 `UsedKeyImages`。
* **存储**:
* 记录 `key_images`。
* `EncryptedVotes` 追加 (append) 投票密文。


* **事件**: 发送 `VoteSubmitted`。

#### `reveal_outcome(origin, poll_id, claimed_tally, private_key)`

* **注**: 乐观揭示逻辑，替代 ZK 证明。
* **权限**: 任何持有私钥的用户。
* **前置条件**: `Poll` 状态必须为 `Tallying`。
* **安全检查**:
* 计算 `derived_pub = RistrettoPoint::mul_base(private_key)`。
* 验证 `derived_pub == poll.poll_public_key`。


* **更新**:
* 更新 `tally` 为 `claimed_tally`。
* 公开 `poll_private_key`。
* 状态置为 `Completed`。


* **事件**: 发送 `PollOutcomeRevealed`。

#### 管理类函数

* `tally_poll(admin, poll_id)`: 手动触发截止，状态由 `Active` 转为 `Tallying`。
* `cancel_poll(admin, poll_id)`: 强制取消投票，状态置为 `Cancelled`。
* `pause_poll(admin, poll_id)`: 暂停投票，状态置为 `Paused`。
* `active_poll(admin, poll_id)`: 恢复投票，状态置为 `Active`。
* `set_deadline(admin, poll_id, new_deadline)`: 修改截止日期（需在 Active 状态下）。

---

### 2. 链下逻辑 (Client / Scripts)

#### `encrypt_and_vote(...)`

* **流程**:
1. 生成随机 `ephemeral_key`。
2. 使用 ECIES 配合 `poll_public_key` 加密选票。
3. 利用 `user_private_key` 和 `ring_members` 生成 BLSAG 环签名。
4. 发送 `vote` 交易。



#### `verify_poll_outcome(poll_id)`

* **流程**:
1. 获取链上已揭示的 `revealed_private_key`。
2. 拉取所有 `EncryptedVotes`。
3. **本地解密**: `plaintext = decrypt(revealed_private_key, ciphertext)`。
4. **本地统计**: 累加得到 `real_tally`。
5. **审计**: 将 `real_tally` 与链上 `poll.tally` 对比，若不符则发出“数据造假”警告。



---

## 五、 数据类型 (Types)

```rust
type PollId = u32;
type RingId = u32;
type Tally = u32;

// --- 基础密码学包装类型 ---

/// 对应 Curve25519 公钥/点
struct CompressedRistrettoWrapper([u8; 32]); 

/// 对应 Curve25519 私钥/标量
struct ScalarWrapper([u8; 32]);              

struct BLSAGWrapper {
    challenge: ScalarWrapper,
    responses: Vec<ScalarWrapper>,
    ring: Vec<CompressedRistrettoWrapper>,
    key_image: CompressedRistrettoWrapper,
}

struct EncryptedVote {
    ephemeral_public_key: CompressedRistrettoWrapper,
    ciphertext: Vec<u8>,
}

// --- 投票核心逻辑类型 ---

enum PollStatus {
    Active,    // 接受投票
    Tallying,  // 已截止，等待揭示私钥
    Completed, // 私钥已揭示，结果已公示
    Paused,    // 暂停
    Cancelled, // 取消
}

struct Poll {
    poll_id: PollId,
    ring_id: RingId,
    description: Vec<u8>,
    metadata_hash: H256,
    deadline: BlockNumber,
    
    /// 投票加密公钥
    poll_public_key: CompressedRistrettoWrapper,
    
    /// 统计私钥 (揭示后存储，初始为 None)
    poll_private_key: Option<ScalarWrapper>,
    
    /// 统计结果 (揭示后存储，初始为 None)
    tally: Option<Tally>,
    
    status: PollStatus,
}

```
