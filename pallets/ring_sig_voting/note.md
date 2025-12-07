# `getrandom` 在 `wasm32-unknown-unknown` 环境下的使用
出错：
```
error: the wasm*-unknown-unknown targets are not supported by default, you may need to enable the "js" feature. For more information see: https://docs.rs/getrandom/#webassembly-support
    --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.2.16/src/lib.rs:346:9
    |
346 | /         compile_error!("the wasm*-unknown-unknown targets are not supported by \
347 | |                         default, you may need to enable the \"js\" feature. \
348 | |                         For more information see: \
349 | |                         https://docs.rs/getrandom/#webassembly-support");
    | |________________________________________________________________________^

error[E0433]: failed to resolve: use of unresolved module or unlinked crate `imp`
    --> /home/sqfzy/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.2.16/src/lib.rs:402:9
    |
402 |         imp::getrandom_inner(dest)?;
    |         ^^^ use of unresolved module or unlinked crate `imp`
    |
    = help: if you wanted to use a crate named `imp`, use `cargo add imp` to add it to your `Cargo.toml`

```
解决：
```
nazgul = { version= "2.1", default-features = false, features = ["no_std"] }
getrandom = { version = "0.2", default-features = false, features = ["js"] }
```
`nazgul` 依赖`getrandom`库，我们需要开启`getrandom`的`js`特性以支持wasm环境。

# 区块链不允许使用随机数
区块链环境下，所有节点必须达成共识，因此不能使用不确定的随机数。任何节点生成的随机数都必须是可预测和可验证的。如果使用随机数，例如在智能合约中生成随机数，可能会导致不同节点生成不同的结果，从而破坏共识机制。


# 添加新的pallet
1. `runtime/Cargo.toml` 添加这个pallet依赖，例如：
```
foo = { path = "../pallets/foo", default-features = false }
```
2. `runtime/src/lib.rs` 让`Runtime`知道这个pallet，例如：
```rust
#[runtime::pallet_index(53)]
pub type Foo = foo;
```
3. `runtime/src/configs/mod.rs` 配置这个pallet，例如：
```rust
impl foo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}
```

# `DecodeWithMemTracking` 让 `pallet::event` 适配自定义类型
```
#[derive(Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, DecodeWithMemTracking)]
pub struct PublicKey(pub [u8; 32]);

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    AnonymousMessagePosted { ring_id: u32, key_image: PublicKey },
}
```

# 更新后端代码后前端没有显示更新
Ctrl + F5 强制刷新浏览器缓存

# 与`polkadot.js`交互时，需要注意使用的类型
1. 使用`Vec<u8>` 或 `H256` 而不是`[u8; N]`，

# stroage map
```rust
StorageMap<
        _,
        Blake2_128,
        CompressedRistrettoWrapper,
        (),
        OptionQuery, 
    >;
```

## 我该选什么 Hasher ?
| Hasher                 | 存储的数据库键      | 支持遍历? | 碰撞安全性    | 性能          | 何时使用 (最佳实践)                                          |
| ---------------------- | ------------------- | --------- | ------------- | ------------- | ------------------------------------------------------------ |
| **`Blake2_128Concat`** | `Hash(Key)` + `Key` | **是**    | 高 (加密级)   | 较慢          | **(最常用)** 当你**需要遍历** Map，并且 Key 是任意长度或有攻击风险时 (如 `AccountId`)。 |
| **`Twox64Concat`**     | `Hash(Key)` + `Key` | **是**    | 中 (非加密级) | 极快          | 当你**需要遍历** Map，并且 Key 较短且无攻击风险时 (如 `u32`, `u64`)。 |
| **`Blake2_128`**       | `Hash(Key)`         | **否**    | 高 (加密级)   | 较慢          | **(最高效)** 当你**永不遍历**，只需要 `get(Key)`，且 Key 有攻击风险时 (如 `AccountId`, `KeyImage`)。 |
| **`Twox64`**           | `Hash(Key)`         | **否**    | 中 (非加密级) | 极快          | 当你**永不遍历**，只需要 `get(Key)`，且 Key 短且无风险时 (如 `u32`)。 |
| **`Identity`**         | `Key` (不哈希)      | **是**    | 无            | 最快 (无操作) | **(谨慎使用)** 当 Key **本身已经是一个加密哈希** (如 `H256`)，或者是一个短的、受信任的键 (如 `EraIndex`)。 |


# 分清楚哪些操作是链上的，哪些是链下的

# benchmarking
1. `pallets/ring_sig_voting/Cargo.toml`
```toml
runtime-benchmarks = ["frame/runtime-benchmarks"]
```
2. `runtime/src/benchmarks.rs`
```rust
polkadot_sdk::frame_benchmarking::define_benchmarks!(
    // ...
    [ring_sig_voting, RingSigVoting]
);
```
3. `touch pallets/ring_sig_voting/src/weights.rs`
4. `cargo build --features runtime-benchmarks --release`
5. `frame-omni-bencher v1 benchmark pallet --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.wasm --pallet "ring_sig_voting" --extrinsic "" --template ./pallets/ring_sig_voting/frame-weight-template.hbs --output ./pallets/ring_sig_voting/src/weights.rs`
6. `weights.rs`加上：
```rust
use frame::deps::frame_support;
use frame::deps::frame_system;
```

# benchmarking.rs
对于`#[extrinsic_call]`:
```rust
#[extrinsic_call]
RingSigVoting::close_poll(RawOrigin::Signed(caller), poll_id);
```
对于非`#[extrinsic_call]`:
```rust
RingSigVoting::<T>::register_ring_group(RawOrigin::Signed(caller.clone()).into(), ring).unwrap();
```
注意`clone()`和`into()`的使用

# 复用polkadot-sdk的pallet
1. 以`pallet-scheduler`为例，修改`runtime/Cargo.toml`
```toml
polkadot-sdk = { workspace = true, features = [
    # ...
    "pallet-scheduler",
], default-features = false }
```
2. 修改`runtime/src/lib.rs`
```rust
#[frame_support::runtime]
mod runtime {
    // ...
    #[runtime::pallet_index(53)]
    pub type Scheduler = pallet_scheduler;
}
```
3. 修改`runtime/src/configs/mod.rs`
```rust
parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		RuntimeBlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	#[cfg(feature = "runtime-benchmarks")]
	type MaxScheduledPerBlock = ConstU32<512>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MaxScheduledPerBlock = ConstU32<50>;
	type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type Preimages = polkadot_sdk::pallet_preimage::Pallet<Runtime>;
	type BlockNumberProvider = frame_system::Pallet<Runtime>;
}
```

# 可以在`polkadot-sdk/substrate/bin/node/runtime/src/lib.rs`中找到所有pallet的配置参考

# Diagram
1. 一轮评价的生命周期过程
2. 教师端
3. 学生端
4. 

# 应当满足
1. 不能刷评：可链接性
2. 评价不能关联到特定学生：通过环签名实现
3. 隐藏评价内容：防止羊群效应影响后面的人进行评价
  1. 使用承诺：学生需要vote, reveal两次操作，用户体验差
  2. 使用tlock：自动允许解密


# 使用pallet_collective作为CreatePollOrigin等type的类型，不能满足返回AccoutId。问题在于返回集合中谁的AccountId呢

# 投票的加解密都在链下，因此Vote的数据结构也在线下定义

# 投票生命周期
Active
Tallying
Paused
Cancelled
Completed

# 为什么 EncryptedVote 需要 AAD
我们让 aad = genesis_hash || poll_id || key_image
若没有AAD，假如 Alice 想让 Bob 投赞成，那么它可以强迫 Bob 的投票密文必须与 Alice 的一样（Alice 投了赞成票）。
有了AAD，即便明文相同，密文也会因为 aad 不同而不同。从而防止强迫。

目的：**让每一个密文都变成“一次性”、“特定人”、“特定场次”的专属物品，从而杜绝任何形式的复制、重放和强迫。**

# 为什么 EncryptedVote 的 nonce 可以全0
因为 ECIES 的每个 $K_{session}$ 都是唯一的，因此即便 Nonce 是全 0，或者是一个固定的常数，$(Key, Nonce)$ 这个组合依然是全局唯一的。

# 解密算法完全离线
可以自行使用阈值加密算法。最后公布私钥时，公布合并后的私钥

# 引入其它pallet
**必须通过polkadot-sdk否则会出现版本不一致的情况，特别是在编写mock.rs时**
```toml
polkadot-sdk = { workspace = true, default-features = false, features = [
  "pallet-balances",
  "pallet-preimage",
  "pallet-scheduler",
] }
```

# 使用preimage pallet
```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type Preimages: QueryPreimage<H = Self::Hashing> + StorePreimage;
}
```

```rust
pub trait StorePreimage: QueryPreimage {
    const MAX_LENGTH: usize;

    // Required method
    fn note(
        bytes: Cow<'_, [u8]>,
    ) -> Result<<Self::H as Hasher>::Out, DispatchError>;

    // Provided methods
    fn unnote(hash: &<Self::H as Hasher>::Out) { ... }
    fn bound<T: Encode>(t: T) -> Result<Bounded<T, Self::H>, DispatchError> { ... }
}

pub trait QueryPreimage {
    type H: Hash;

    // Required methods
    fn len(hash: &<Self::H as Hasher>::Out) -> Option<u32>;
    fn fetch(hash: &<Self::H as Hasher>::Out, len: Option<u32>) -> FetchResult;
    fn is_requested(hash: &<Self::H as Hasher>::Out) -> bool;
    fn request(hash: &<Self::H as Hasher>::Out);
    fn unrequest(hash: &<Self::H as Hasher>::Out);

    // Provided methods
    fn hold<T>(bounded: &Bounded<T, Self::H>) { ... }
    fn drop<T>(bounded: &Bounded<T, Self::H>) { ... }
    fn have<T>(bounded: &Bounded<T, Self::H>) -> bool { ... }
    fn pick<T>(hash: <Self::H as Hasher>::Out, len: u32) -> Bounded<T, Self::H> { ... }
    fn peek<T: Decode>(
        bounded: &Bounded<T, Self::H>,
    ) -> Result<(T, Option<u32>), DispatchError> { ... }
    fn realize<T: Decode>(
        bounded: &Bounded<T, Self::H>,
    ) -> Result<(T, Option<u32>), DispatchError> { ... }
}
```

# 使用scheduler pallet
```rust
use frame_support::traits::{
	schedule::{
		v3::{Anon as ScheduleAnon, Named as ScheduleNamed},
	},
};


type Scheduler: schedule::v3::Anon<
		BlockNumberFor<Self>,
		RuntimeCallFor<Self>,
		Self::RuntimeOrigin,
		Hasher = Self::Hashing,
	> + schedule::v3::Named<
		BlockNumberFor<Self>,
		RuntimeCallFor<Self>,
		Self::RuntimeOrigin,
		Hasher = Self::Hashing,
	>;
```

```rust
pub trait Anon<BlockNumber, Call, Origin> {
    type Address: Codec + MaxEncodedLen + Clone + Eq + EncodeLike + Debug + TypeInfo;
    type Hasher: Hash;

    // Required methods
    fn schedule(
        when: DispatchTime<BlockNumber>,
        maybe_periodic: Option<Period<BlockNumber>>,
        priority: Priority,
        origin: Origin,
        call: Bounded<Call, Self::Hasher>,
    ) -> Result<Self::Address, DispatchError>;
    fn cancel(address: Self::Address) -> Result<(), DispatchError>;
    fn reschedule(
        address: Self::Address,
        when: DispatchTime<BlockNumber>,
    ) -> Result<Self::Address, DispatchError>;
    fn next_dispatch_time(
        address: Self::Address,
    ) -> Result<BlockNumber, DispatchError>;
}

pub trait Named<BlockNumber, Call, Origin> {
    type Address: Codec + MaxEncodedLen + Clone + Eq + EncodeLike + Debug;
    type Hasher: Hash;

    // Required methods
    fn schedule_named(
        id: TaskName,
        when: DispatchTime<BlockNumber>,
        maybe_periodic: Option<Period<BlockNumber>>,
        priority: Priority,
        origin: Origin,
        call: Bounded<Call, Self::Hasher>,
    ) -> Result<Self::Address, DispatchError>;
    fn cancel_named(id: TaskName) -> Result<(), DispatchError>;
    fn reschedule_named(
        id: TaskName,
        when: DispatchTime<BlockNumber>,
    ) -> Result<Self::Address, DispatchError>;
    fn next_dispatch_time(id: TaskName) -> Result<BlockNumber, DispatchError>;
}
```

```rust
let current_block = frame_system::Pallet::<T>::block_number();
let when = DispatchTime::At(current_block + delay);

// 创建要调度的调用
let call = Call::<T>::execute_hello_world {}. into();
let bounded_call = T::Preimages::bound(call)
    .map_err(|_| Error::<T>::ScheduleFailed)?;

// 调度任务
let task_id = b"hello_world_task".to_vec();
T::Scheduler::schedule_named(
    task_id. clone(),
    when,
    None, // 不重复
    127,  // 优先级
    frame_system::RawOrigin::Root.into(),
    bounded_call,
)
.map_err(|_| Error::<T>::ScheduleFailed)?;
```
