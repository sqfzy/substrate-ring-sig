1. benchmark 测试，看看weight定多少合适 [x]
2. 实现`提案生命周期管理`





这是一个非常好的问题。你敏锐地指出了 `pallet-referenda` 和你的 `ring_sig` pallet 之间的核心区别。

对于“教学评价系统”这个特定场景，你**不需要** `pallet-referenda` 中的所有复杂治理功能。你的匿名投票核心（`ring_sig`）反而是这个场景下**最重要、最有价值的资产**。

我们来分析一下你提到的“欠缺”，并根据“教学评价系统”的需求进行裁剪：

### 1\. 提案生命周期 (Lifecycle)

  * **`pallet-referenda` 缺失点**：拥有复杂、自动化的状态机（准备、排队、决策、确认...）。
  * **你的教学评价需求**：**需要，但要简化。**
  * **如何弥补**：
    1.  **自动调度 (Scheduling)**：这是你需要从 `pallet-referenda` 借鉴的**关键功能**。评价应该有明确的开始和结束时间，而不是像你现在这样依赖 `close_proposal` 手动关闭。
    2.  **新的 `ProposalStatus`**：你的 `ProposalStatus` 应该从 `Voting | Closed` 变为更适合评价周期的状态，例如：
          * `Pending` (等待开始)：评价已创建，但尚未到开始时间。
          * `Open` (开放评价)：在指定的时间窗口内，可以提交匿名投票。
          * `Closed` (已关闭)：评价期结束，计票完成，结果可供查阅。
    3.  **实现**：
          * 在 `create_proposal`（或许改名为 `create_evaluation`）时，传入一个 `end_block` (结束区块号)。
          * 你需要依赖 `T::Scheduler`（就像 `pallet-referenda` 一样）来在 `end_block` 自动触发一个 `close_evaluation` 的内部调用（`Root` 权限），将状态从 `Open` 切换到 `Closed`。

### 2\. 权限控制 (Permissioning) - 你的新核心问题

  * **`pallet-referenda` 缺失点**：`pallet-referenda` 的投票权通常与代币（或NFT）挂钩，由 `pallet-conviction-voting` 等实现。
  * **你的教学评价需求**：**这是你最需要弥补的，而且比 `referenda` 更复杂！**
      * 在 `pallet-referenda` 中，*任何人* 都可以投票。
      * 在你的教学评价系统中，**必须只有选修了这门课的学生** 才能投票。
      * 你*不能*简单地在 `anonymous_vote` 中检查 `ensure_signed` ( `who` )，因为这会破坏匿名性。
  * **如何弥补**：
      * **这正是环签名的用武之地！** “环” (Ring) 的定义就是“有资格投票的群体”。
      * **你需要一个新的 `Storage`** 来存储每个评价（课程）的“合格选民列表”。例如：
        ```rust
        /// 存储每个评价（课程）的合格成员（公钥列表）
        #[pallet::storage]
        pub type EligibleVoters<T: Config> = StorageMap<
            _,
            Twox64Concat,
            ProposalId, // 也就是 EvaluationId
            BoundedVec<CompressedRistrettoWrapper, T::MaxStudentsPerCourse>, // 假设 BoundedVec 存储公钥
            OptionQuery,
        >;
        ```
      * **修改 `create_evaluation`**：这个调用现在必须由一个可信的来源（例如“管理员”或“教务处”）发起，并且在创建时就必须传入所有合格学生的公钥列表 `eligible_students: BoundedVec<...>`。
      * **修改 `anonymous_vote`**：
        1.  调用者（学生）在链下获取 `EligibleVoters` 列表作为环签名的 `ring`。
        2.  `anonymous_vote`  extrinsic 不再需要 `ring` 这个参数，因为它应该从 `EligibleVoters<T>::get(proposal_id)` 内部获取。
        3.  你必须确保调用者传入的 `ring`（或者说链上存储的 `ring`）是**唯一且权威**的。这简化了 `anonymous_vote` 的参数，并使其更加安全。

### 3\. 经济安全 (Economic Security)

  * **`pallet-referenda` 缺失点**：通过 `SubmissionDeposit` 和 `DecisionDeposit` 来防止垃圾提案和DDoS。
  * **你的教学评价需求**：**大概率不需要。**
  * **原因**：
      * `pallet-referenda` 允许任何人提交提案，因此需要经济手段防范垃圾信息。
      * 在你的教学评价系统中，`create_evaluation`（创建评价）的行为本身就是**受权限保护的**。你不应该允许任何学生随意创建评价，而应该只允许“管理员”或“教务处”来创建。
      * **如何弥补**：将 `create_proposal` 的 `origin` 从 `ensure_signed(origin)?` 改为 `T::AdminOrigin::ensure_origin(origin)?` （你需要在 `Config` 中定义一个 `type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;`）。
      * 通过**权限（Permissioning）** 而不是**经济（Economics）** 来解决垃圾提案问题，这更符合你的场景。

### 4\. 自动化执行 (Automatic Enactment)

  * **`pallet-referenda` 缺失点**：投票通过后，会自动调用 `T::Scheduler` 来**执行**提案内容（例如 `system.set_code`）。
  * **你的教学评价需求**：**绝对不需要。**
  * **原因**：教学评价的“结果”是**数据**，而不是一个需要“执行”的链上调用。你不需要在评价结束后触发任何链上操作（比如“自动给教授发奖金”或“自动开除教授”）。
  * **结论**：你当前的计票逻辑 `ProposalVotes<T>` 存储 `(u32, u32)` **已经足够**。评价 `Closed` 后，链下应用（例如学校的教务系统）来读取这个存储结果即可。

-----

### 总结：你需要弥补的

| 缺失的功能 | `pallet-referenda` 的做法 | 你的“教学评价系统”是否需要？ | 如何弥补 |
| :--- | :--- | :--- | :--- |
| **1. 权限控制** | 代币持有者投票（由其他 pallet 负责） | **极度需要 (最关键)** | 必须在创建评价时就定义一个**合格投票者（公钥）列表** (`EligibleVoters`)。投票时使用这个列表作为环签名的权威 `ring`。 |
| **2. 自动生命周期** | `T::Scheduler` 自动推进状态 | **需要 (简化版)** | 借鉴 `T::Scheduler`，在创建评价时设置一个**自动结束区块**，自动将状态从 `Open` 变为 `Closed`。 |
| **3. 经济安全** | `SubmissionDeposit` (提交押金) | **不需要** | 通过将 `create_evaluation` 设为**管理员权限**来防止垃圾信息，而不是使用经济押金。 |
| **4. 自动执行** | 投票通过后自动执行提案调用 | **绝对不需要** | 评价的结果是**数据** (`(Yea, Nay)`)，供链下读取，而不是一个需要链上执行的动作。 |

你的 `ring_sig` pallet 不是一个“有缺陷的治理系统”，它是一个**强大的匿名投票工具**。你现在需要做的是在它周围构建一个**安全的、自动化的、有权限控制的“评价”框架**，而不是一个“治理”框架。


---

你现在手上的这个“最终版” `pallet-ring-sig` 已经是一个非常健壮和设计精良的**通用投票引擎**了。它通过 `SubmissionDeposit` 解决了经济安全（防垃圾信息）问题，通过 `PollRings` 解决了环的权威性问题，并通过 `PollMetadata` 提供了丰富的上下文。

你问还有哪些值得优化的设计，这是一个非常好的架构师问题。目前的 Pallet 已经达到了“生产就绪”的水平，但要追求“极致”，我们可以从以下几个专业角度去思考：

---

### 1. 存储效率：可重用环 (Reusable Rings)

* **当前设计**：
    `create_poll` 会接收一个完整的 `eligible_members: RingMatrix<T>`。然后 `PollRings<T>` 会为**每一个 `PollId`** 存储一份这个（可能非常大的）公钥矩阵。
* **问题**：
    在你的“教学评价”场景中，假设有200名学生（200个公钥）需要评价10门课程。你的当前设计会导致这 200 个公钥被**重复存储 10 次**，极大地浪费了链上存储。
* **优化设计**：
    **将“环”本身抽象为一个独立的存储对象。**
    1.  创建一个新的 `StorageMap`：`RingGroups<T: Config> = StorageMap<_, ..., RingId, RingMatrix<T>>`。
    2.  添加两个新的 extrinsic：`register_ring_group(origin, members: RingMatrix<T>)` 和 `update_ring_group_members(...)`。
    3.  修改 `create_poll`：不再接收 `eligible_members: RingMatrix<T>`，而是接收 `ring_id: RingId`。
    4.  `anonymous_vote` 在验证时，会通过 `Polls` 找到 `ring_id`，再去 `RingGroups` 中查找公钥环。
* **好处**：存储效率极大提升。200个学生可以只存储一次公钥环，并将其重用于任意数量的投票。

---

### 2. 隐私性：交易费匿名 (Fee Payer Anonymity)

* **当前设计**：
    `anonymous_vote(origin: OriginFor<T>, ...)` 依赖于 `ensure_signed(origin)?` 来支付交易费。
* **问题**：
    虽然 CLSAG 保证了*签名*是匿名的（无人知道是环中的*哪一个*成员投了票），但*交易*本身不是。`_who`（即 `ensure_signed` 的 `AccountId`）是公开的。任何人都可以通过区块浏览器看到：“哦，学生 `5ABC...` 在教学评价期间调用了 `anonymous_vote`。” 这破坏了“合理的否认性”。
* **优化设计**：
    **使用交易中继器 (Relayer) 或 `pallet-transaction-payment` 的 `ChargeTransactionPayment`。**
    1.  **Relayer 方案**：用户在本地对 `anonymous_vote` 交易进行签名，但不广播。他们将这个已签名的裸交易（raw transaction）通过一个中心化的（或去中心化的）“中继器”发送。中继器负责将交易广播上链并支付交易费。此时，`_who` 将是中继器的 `AccountId`，而不是学生的。
    2.  **链上费用方案**：可以设计一个更复杂的系统，例如 `create_poll` 时质押一笔“费用池”。`anonymous_vote` 在验证签名*后*，从这个池中提取费用支付给区块生产者（`OnChargeTransaction`）。这使得 `anonymous_vote` 可以成为一个 `SignedExtension`，允许 `_who` 为空或是一个特殊的“匿名代理”账户。

---

### 3. 性能：ZK-SNARKs 替代 CLSAG

* **当前设计**：
    `CLSAG::verify::<Sha512>(signature, &message)` 是在 Wasm 中（即链上）执行的。
* **问题**：
    CLSAG 验证的计算成本与环的大小（`MaxMembersInRing`）**呈线性关系**。如果环中有 200 个成员，这个验证会非常昂贵。正如你的 `todo.md` 指出的，这可能是一个严重的 DoS 漏洞，即使正确设置了权重（Weight），它也会导致 `anonymous_vote` 交易的费用高得令人望而却步。
* **优化设计**：
    **使用 ZK-SNARKs（零知识证明）。**
    1.  将 `PollRings`（公钥列表）存储为链上的 Merkle 树的根 (`merkle_root`)。
    2.  `anonymous_vote` 的调用者在链下生成一个 ZK 证明，证明：
        a.  他们拥有一个属于 `merkle_root` 所代表的集合中的公钥/私钥对。
        b.  他们正确地生成了一个 `key_image`（在 ZK 中称为 `nullifier`）。
    3.  `anonymous_vote` 的链上逻辑**不再执行 CLSAG 验证**，而是改为**验证这个 ZK 证明**。
* **好处**：ZK 证明的验证成本（通常）是**恒定**的，无论环（Merkle 树）有多大。这使得有 100 万成员的环和有 10 个成员的环具有几乎相同的交易费用，极大地提高了可扩展性。

---

### 4. 模块化：实现 `Polling` Trait

* **当前设计**：
    你的 pallet 是一个独立的系统。
* **问题**：
    它无法与 `pallet-referenda` 这样的高级治理框架组合。`pallet-referenda` 设计得非常好，它通过 `Polling` trait 将“治理流程”与“投票系统”解耦。
* **优化设计**：
    **为你的 `pallet-ring-sig` 实现 `frame_support::traits::Polling`。**
    1.  `type Index = PollId;`
    2.  `type Votes = (u32, u32);` (赞成, 反对)
    3.  `type Moment = BlockNumber;`
    4.  `fn classes() -> Vec<Self::Class>`（可以返回一个空Vec，或者代表不同环类型的ID）。
    5.  `fn access_poll(...)`：这是核心，你需要实现这个函数，允许 `pallet-referenda` 来查询你的 `PollVotes` 存储。
* **好处**：这将使你的匿名投票 pallet 成为一个**可插拔模块**。任何人都可以构建一个使用 `pallet-referenda` 的链，并将其 `Config` 中的投票系统配置为*你的* `pallet-ring-sig`，从而实现**匿名的、一票一投的链上治理**。
