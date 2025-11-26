# 环签名匿名投票系统文档 (Ring Sig Voting Documentation)

## 1\. 系统上下文 (System Context)

本系统实现了一个基于 Substrate 的去中心化匿名评价系统。以“教学评价”为例：

  * **老师 (Teacher/Creator)**：创建评价问卷，设定截止日期，并掌握解密私钥（但在评价结束前不能查看内容）。
  * **学生 (Student/Voter)**：使用环签名技术提交评价。系统可以验证“这是某个学生的真实评价”，但无法追踪“具体是哪位学生”。
  * **管理员 (Ring Admin)**：维护合法的公钥环（即合法的学生名单）。
  * **区块链 (Blockchain)**：作为可信的时间戳和验证层，防止双重评价，并在结束时验证统计结果的合法性。

<!-- end list -->

```mermaid
graph TD
    subgraph "Users (Off-chain)"
        Teacher["老师 (Creator/Closer)"]
        Student["学生 (Voter)"]
        Admin["管理员 (Ring Admin)"]
    end

    subgraph "Substrate Chain (On-chain)"
        Pallet["Ring Sig Voting Pallet"]
        Storage["Storage (Polls, Rings, Votes)"]
    end

    Admin -- "1. 注册公钥环 (Register Ring)" --> Pallet
    Teacher -- "2. 创建评价 (Create Poll)" --> Pallet
    Teacher -- "4. 结束并提交统计 (Close & Tally)" --> Pallet
    Student -- "3. 匿名提交评价 (Anonymous Vote)" --> Pallet

    Pallet -- "Read/Write" --> Storage
```

-----

## 2\. 评价生命周期 (Poll Lifecycle)

评价 (Poll) 的状态机非常简洁，只有两个状态：`Voting` (进行中) 和 `Closed` (已结束)。设计重点在于状态转换时的强制校验。

  * **创建时**：状态立即变为 `Voting`，并锁定押金。
  * **投票期间**：接收加密选票，检查 Key Image 防止双花。
  * **关闭时**：必须提供与创建时公钥匹配的私钥，同时提交链下计算好的 Tally 结果。

<!-- end list -->

```mermaid
stateDiagram-v2
    [*] --> Voting: create_poll(description, ring_id, pub_key)

    state "Voting (进行中)" as Voting {
        [*] --> AcceptVote
        AcceptVote --> CheckKeyImages: 接受 anonymous_vote
        CheckKeyImages --> StoreVote: 检查 UsedKeyImages (防双花)
        StoreVote --> [*]: 存储 EncryptedVote
    }

    Voting --> Closed: close_poll(private_key, tally)
    
    note right of Closed
        条件:
        1. 当前块高 > deadline
        2. derived(private_key) == pub_key
    end note

    state "Closed (已结算)" as Closed {
        [*] --> RevealKey
        RevealKey --> StoreTally: 公开 Private Key
        StoreTally --> RefundDeposit: 存储最终 Tally
        RefundDeposit --> [*]: 退还押金
    }

    Closed --> [*]
```

-----

## 3\. 详细交互流程 (Detailed Interactions)

### 3.1 准备阶段 (Setup Phase)

此阶段确立了“谁可以投票”（通过 Ring）以及“投票的加密参数”（通过 Poll Creation）。

```mermaid
sequenceDiagram
    autonumber
    participant Admin as 管理员 (Ring Admin)
    participant Teacher as 老师 (Poll Creator)
    participant Chain as Blockchain Node

    rect rgb(249, 249, 249)
        Note over Admin,Chain: Identity Management
        Admin->>Admin: 收集学生公钥 (Ristretto Points)
        Admin->>Chain: register_ring_group(ring_matrix)
        Chain->>Chain: 分配 RingID, 存储公钥矩阵
        Chain-->>Admin: Event: RingGroupRegistered(RingID)
    end

    rect rgb(230, 247, 255)
        Note over Teacher,Chain: Poll Creation
        Teacher->>Teacher: 生成一次性加密密钥对 (Ephem Keypair)
        Note right of Teacher: PrivKey: 保密 (用于最后解密)<br/>PubKey: 公开 (用于加密选票)
        Teacher->>Chain: create_poll(desc, RingID, PubKey, deposit)
        Chain->>Chain: 检查 RingID 存在
        Chain->>Chain: 锁定押金 (Reserve Deposit)
        Chain->>Chain: State = Voting
        Chain-->>Teacher: Event: PollCreated(PollID)
    end
```

### 3.2 投票阶段 (Voting Phase) - 核心逻辑

这是系统最复杂的部分。为了保护隐私，大部分加密计算在客户端（学生端）完成。链上只负责验证零知识证明（环签名）和防重放。

**关键点说明**：

  * **ECIES 加密**：学生使用 `Poll PubKey` 和自己生成的 `Ephemeral Key` 进行 ECDH 协商，加密评价内容。
  * **Linkable Ring Signature (CLSAG)**：
      * **Anonymity**: 签名证明了“我是 Ring 中的一员”。
      * **Linkability**: 生成唯一的 `Key Image`。如果同一个私钥尝试第二次签名，会生成相同的 `Key Image`，链上会拒绝。

<!-- end list -->

```mermaid
sequenceDiagram
    autonumber
    participant Student as 学生 (Voter)
    participant Chain as Blockchain Node

    Note over Student, Chain: 前提: 获取 PollID, RingID, Poll PubKey

    rect rgb(240, 248, 255)
        Note right of Student: === 链下计算 (Off-chain) ===
        Student->>Student: 1. 生成临时密钥 (Vote Ephemeral Key)
        Student->>Student: 2. ECDH 加密: Encrypt(Vote, Poll PubKey)
        Student->>Student:    生成 Ciphertext & AuthTag
        Student->>Student: 3. 构造环签名 (CLSAG)
        Student->>Student:    Input: 自己的私钥, Ring 所有公钥
        Student->>Student:    Output: Signature, Key Image
    end

    Student->>Chain: anonymous_vote(PollID, Ciphertext, Tag, Signature, KeyImage)

    rect rgb(255, 240, 245)
        Note left of Chain: === 链上验证 (On-chain) ===
        Chain->>Chain: 1. 检查 Poll 状态 == Voting
        Chain->>Chain: 2. 检查 Poll 是否过期 (Now < Deadline)
        Chain->>Chain: 3. 防双花检查 (Anti-Replay)
        Chain->>Chain:    Assert: UsedKeyImages[PollID][KeyImage] is Empty
        Chain->>Chain: 4. 验证环签名 (Verify CLSAG)
        Chain->>Chain:    验证签名者属于 Ring，且未篡改 Ciphertext
        
        alt 验证失败
            Chain-->>Student: Error: InvalidSignature / AlreadyVoted
        else 验证成功
            Chain->>Chain: 记录 UsedKeyImages[PollID][KeyImage] = True
            Chain->>Chain: 存储 EncryptedVote
            Chain-->>Student: Event: EncryptedVoteCast
        end
    end
```

### 3.3 结算阶段 (Settlement Phase) - 乐观计票

按照你的代码逻辑，计票由 Creator 在链下完成，链上验证解密权限。

```mermaid
sequenceDiagram
    autonumber
    participant Teacher as 老师 (Closer)
    participant Chain as Blockchain Node

    rect rgb(240, 248, 255)
        Note right of Teacher: === 链下计票 (Off-chain Tally) ===
        Teacher->>Chain: 获取 Poll 所有 EncryptedVotes
        Teacher->>Teacher: 使用保存的 Encryption Private Key
        loop 每一张选票
            Teacher->>Teacher: 解密 Ciphertext -> Vote Content
        end
        Teacher->>Teacher: 聚合结果 (Calculate Tally)
    end

    Teacher->>Chain: close_poll(PollID, PrivateKey, Tally)

    rect rgb(255, 240, 245)
        Note left of Chain: === 链上验证与结算 ===
        Chain->>Chain: 1. 验证 PrivateKey 匹配 Poll PubKey
        Chain->>Chain:    Derived(PrivateKey) == Stored PubKey?
        Chain->>Chain: 2. 更新状态: Status = Closed
        Chain->>Chain: 3. 存储 Tally 和 PrivateKey (公开用于审计)
        Chain->>Chain: 4. 退还 Creator 押金
        Chain-->>Teacher: Event: PollClosed(Tally)
    end
```

-----

## 4\. 数据模型与存储 (Data Model & Storage)

为了支持上述的匿名投票流程，链上存储设计必须兼顾“隐私保护”和“可验证性”。以下实体关系图 (ER Diagram) 展示了各个存储单元 (StorageMap) 之间的逻辑关联。

**设计亮点**：

  * **解耦设计**：`RingGroups`（公钥环）与 `Polls`（评价/投票）分离。同一个学生名单（班级）可以被多个评价复用，节省存储空间。
  * **双花防范**：`UsedKeyImages` 绑定了 `PollId` 和 `KeyImage`。这意味着同一个学生可以在“期中评价”和“期末评价”中分别投票，但在同一个评价中只能投一次。

<!-- end list -->

```mermaid
erDiagram
    %% 核心实体：投票/评价
    POLL ||--o{ ENCRYPTED_VOTE : "收集 (Collection)"
    POLL ||--|| RING_GROUP : "基于 (Uses)"
    POLL ||--o{ USED_KEY_IMAGE : "防双花记录 (Key Images)"
    POLL ||--|| TALLY_RESULT : "最终结果 (Result)"

    POLL {
        u64 poll_id PK "唯一标识符"
        AccountId creator "老师或创建者"
        bytes description "评价说明"
        string status "Voting或Closed"
        u32 deadline "截止块高"
        bytes32 encryption_pubkey "加密公钥(R)"
        bytes32 encryption_privkey "解密私钥(结束时揭示)"
    }

    RING_GROUP {
        u64 ring_id PK "环ID"
        matrix public_keys "公钥矩阵(学生名单)"
    }

    ENCRYPTED_VOTE {
        u64 poll_id FK "关联Poll"
        bytes32 ephemeral_pubkey "临时公钥"
        bytes ciphertext "加密内容(评价)"
        bytes32 tag "认证标签"
        struct ring_signature "CLSAG签名证明"
    }

    USED_KEY_IMAGE {
        u64 poll_id FK "关联Poll"
        bytes32 key_image PK "关键镜像(衍生自私钥)"
    }

    TALLY_RESULT {
        u64 poll_id FK "关联Poll"
        struct tally "统计结果(如5分10人4分3人)"
    }
```

-----

## 5\. 接口与 API (Integration & API)

本节主要面向前端开发者。为了完成一次完整的评价流程，前端需要依次调用以下 Extrinsics（外部交易），并监听相应的 Events（事件）。

### 5.1 核心交易 (Extrinsics)

| 方法名 (Method) | 角色 | 参数说明 (Parameters) | 功能描述 |
| :--- | :--- | :--- | :--- |
| **`register_ring_group`** | Admin | `ring: Vec<Vec<Pubkey>>` | **注册班级名单**。<br>提交一组学生的公钥，生成一个 `RingId`。这是为了复用名单，避免每次投票都上传几百个公钥。 |
| **`create_poll`** | Teacher | `desc: Bytes`<br>`ring_id: u64`<br>`deadline: BlockNum`<br>`pubkey: [u8;32]` | **发起评价**。<br>指定使用的班级 (`ring_id`) 和加密用的公钥。需支付押金。 |
| **`anonymous_vote`** | Student | `poll_id: u64`<br>`ciphertext: Bytes`<br>`auth_tag: [u8;16]`<br>`signature: CLSAG` | **提交匿名评价**。<br>核心逻辑：前端生成临时密钥加密内容，并计算环签名。如果验证通过，票会被记录。 |
| **`close_poll`** | Teacher | `poll_id: u64`<br>`privkey: [u8;32]`<br>`tally: TallyType` | **结束评价**。<br>老师在本地解密所有选票，统计结果，然后将私钥和结果上链。链上验证私钥匹配后，公示结果。 |

### 5.2 关键事件 (Events)

前端应当监听这些事件以更新 UI 状态。

```mermaid
classDiagram
    class Event {
        <<Enumeration>>
    }
    
    class RingGroupRegistered {
        +u64 ring_id
        +AccountId admin
        Note: "名单注册成功，获取 ID 用于创建投票"
    }
    
    class PollCreated {
        +u64 poll_id
        +u64 ring_id
        +AccountId creator
        +[u8;32] encryption_pubkey
        Note: "新评价开始，学生需获取 pubkey 加密"
    }
    
    class EncryptedVoteCast {
        +u64 poll_id
        +CompressedRistretto key_image
        Note: "有人投票成功 (匿名)，UI 更新投票数"
    }
    
    class PollClosed {
        +u64 poll_id
        +Tally tally
        +[u8;32] private_key_revealed
        Note: "评价结束，展示统计结果和解密私钥"
    }

    Event <|-- RingGroupRegistered
    Event <|-- PollCreated
    Event <|-- EncryptedVoteCast
    Event <|-- PollClosed
```

-----

## 6\. 技术栈与密码学参数 (Tech Stack & Crypto Specs)

为了确保集成的兼容性，以下是系统使用的具体密码学参数：

  * **椭圆曲线 (Curve)**: `Ristretto255` (基于 Curve25519 的素数阶群，杜绝了 cofactor 问题)。
  * **环签名方案 (Ring Signature)**: `CLSAG` (Compact Linkable Spontaneous Anonymous Group Signature)。
      * *特性*: 签名大小较小，验证速度快，且具备可链接性 (Linkability)。
  * **加密方案 (Encryption)**: `ECIES` 变体。
      * 使用 `Ristretto255` 点进行 Diffie-Hellman 密钥交换。
      * 派生出的对称密钥用于加密评价内容 (Payload)。
  * **哈希算法 (Hash)**: `SHA-512` (用于将点映射到标量) 和 `Blake2` (用于 Substrate 存储哈希)。

-----

## 7\. 安全性与局限性 (Security & Limitations)

### 7.1 安全假设

1.  **匿名性 (Anonymity)**: 只要环中至少有一个其他成员是诚实的，攻击者就无法以显著高于随机猜测的概率确定签名者。
2.  **不可伪造性 (Unforgeability)**: 只有拥有对应公钥私钥的人才能生成有效的环签名。
3.  **可链接性 (Linkability)**: 同一私钥对同一消息（上下文）的两次签名必然产生相同的 Key Image。

### 7.2 已知局限 (Current Limitations)

  * **链下计票信任模型**: 目前系统的计票结果 (`Tally`) 是由创建者在链下计算并提交的。虽然链上强制揭示了私钥，允许任何人事后验证计票的正确性（**可审计**），但如果创建者故意提交错误的 Tally，需要依赖社区的监督和举报机制（或者在未来版本中引入 ZK-Proof 来证明计票正确性）。
  * **扩展性**: 环签名的大小与环成员数量呈线性关系 (Linear Size)。对于几百人的班级评价完全没问题，但如果扩展到数万人的全校投票，交易体积会变大，可能需要考虑更高阶的零知识证明方案 (如 zk-SNARKs)。
