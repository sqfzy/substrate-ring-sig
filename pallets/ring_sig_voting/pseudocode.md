# system
人物：教务处代表，学生会代表，教师工会代表，老师群体，学生群体
链上存储：tally_vk，rings，polls, encrypted_votes, used_key_images, tallies, poll_count, ring_count
链下存储：tally_pk, metadata, poll_private_key

# 约定
投票加解密使用 `aad = genesis_hash || poll_id || key_image`,`nonce = [0;32]`

签名的消息格式为 `ephemeral_public_key || ciphertext`

zk-SNARKs使用 Groth16

使用2.1版本nazgul库的BLSAG实现环签名验证

使用`frame`库（一个大包大揽的完整substrate框架库），不需要引入`sp-std`, `frame_support`等子库

# function
## 链上
1. register_ring(admin, ring)
    1. 权限检查
    2. 存储ring到Rings
    3. 发送事件 RingRegistered 

2. create_poll(admin, ring_id, description, metadata_hash, deadline, poll_public_key, tally_vk)
    1. 权限检查
    2. 检验截止日期是否合法，RingId是否存在
    3. Poll::new(ring_id, description, metadata_hash, deadline, poll_public_key, tally_vk)
    4. poll.set_status(Active)。存储poll到Polls
    5. 发送事件 PollCreated

3. vote(origin, poll_id, ephemeral_public_key, ciphertext, challenge, responses, key_images)
    1. 确保poll.get_status() == Active.
    2. ring_id = Polls.get(poll_id).ring_id; ring = Rings.get(ring_id)
    3. encrypted_vote = {ephemeral_public_key, ciphertext}
    4. signature = {challenge, responses, ring, key_images}
    5. BLSAG::verify(signature, encrypted_vote)
    6. UsedKeyImages 检查 key_images 是否被使用过
    7. 存储 key_images 到 UsedKeyImages; 存储 encrypted_vote 到 EncryptedVotes
    8. 发送事件 VoteSubmitted

4. tally(admin, poll_id, tally, zk_proof)
    1. 权限检查
    2. 确保 poll.get_status() == Tallying.
    3. verify(tally_vk, tally, proof)
    4. poll.set_status(Completed)
    5. 发送事件 PollTallied

5. tally_poll(admin, poll_id)
    1. 权限检查
    2. poll.set_status(Tallying)
    3. 发送事件 PollTallying

6. cancel_poll(admin, poll_id)
    1. 权限检查
    2. poll.set_status(Cancelled)
    3. 发送事件 PollCancelled

7. pause_poll(admin, poll_id)
    1. 权限检查
    2. poll.set_status(Paused)
    3. 发送事件 PollPaused

8. active_poll(admin, poll_id)
    1. 权限检查
    2. poll.set_status(Active)
    3. 发送事件 PollActivated

8. set_deadline(admin, poll_id, new_deadline)
    1. 权限检查
    2. 确保 poll.get_status() == Active.
    3. 检验 new_deadline 是否合法
    4. poll.set_deadline(new_deadline)
    5. 发送事件 PollDeadlineUpdated


## 链下
1. prove(tally_pk, circuit)
2. decrypt(poll_private_key, encrypted_vote)
3. encrypt(poll_public_key, vote)


# types
## 链上
```rust
type PollId =  u64
type RingId = u64
type Tally = u64

```
```rust
#[derive(
    Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen,
)]
pub struct CompressedRistrettoWrapper([u8; 32]);

impl from H256 for CompressedRistrettoWrapper;
impl from RistrettoPoint for CompressedRistrettoWrapper;
impl from CompressedRistrettoWrapper for RistrettoPoint;
impl from CompressedRistretto for CompressedRistrettoWrapper;
impl from CompressedRistrettoWrapper for CompressedRistretto;
impl deref CompressedRistretto for CompressedRistrettoWrapper;

#[derive(
    Clone, Debug, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen,
)]
pub struct ScalarWrapper([u8; 32]);

impl from H256 for ScalarWrapper;
impl from Scalar for ScalarWrapper;
impl from ScalarWrapper for Scalar;
impl deref Scalar for ScalarWrapper;

#[derive(
    CloneNoBound,
    DebugNoBound,
    PartialEqNoBound,
    EqNoBound,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
pub struct BLSAGWrapper {
    pub challenge: ScalarWrapper,
    pub responses: Vec<ScalarWrapper>,
    pub ring: Vec<CompressedRistrettoWrapper>,
    pub key_image: CompressedRistrettoWrapper,
}

impl from BLSAG for BLSAGWrapper;
impl from BLSAGWrapper for BLSAG;
impl deref BLSAG for BLSAGWrapper;

/// 投票状态
#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, RuntimeDebug)]
pub enum PollStatus {
    /// 活跃状态，可以投票
    Active,
    /// 统计中，不能投票
    Tallying,
    /// 已完成
    Completed,
    /// 已暂停
    Paused,
    /// 已取消
    Cancelled,
}

#[derive(CloneNoBound, Encode, Decode, TypeInfo, PartialEqNoBound, EqNoBound, MaxEncodedLen, RuntimeDebug)]
#[scale_info(skip_type_params(T))]
pub struct Poll<T: crate::Config> {
    /// 创建者
    pub creator: T::AccountId,
    /// 投票 ID
    pub poll_id: PollId,
    /// 环签名组 ID
    pub ring_id: RingId,
    /// 描述
    pub description: BoundedVec<u8, T::MaxDescriptionLength>,
    /// 元数据哈希
    pub metadata_hash: H256,
    /// 截止时间
    pub deadline: T::BlockNumber,
    /// 统计公钥
    pub poll_public_key: CompressedRistrettoWrapper,
    /// 统计验证密钥（Groth16）
    pub tally_vk: BoundedVec<u8, T::MaxVkLength>,
    /// 投票状态
    pub status: PollStatus,
}

impl Poll {
    pub fn new(...) {
        check deadline is valid
        
        return Self
    }

    
    pub fn get_status(...) {
        if self.status == Active && now > deadline {
            self.status = Tallying;
            同步更新 Polls 中对应的 poll
        }

        return self.status
    }

    pub fn set_status(...) {
        if status == Active && now > deadline {
            self.status = Tallying;
        }
        match (current_status, new_status) {
              (Active, Tallying) |
              (Active, Paused) |
              (Active, Cancelled) |
              (Tallying, Completed) |
              (Tallying, Paused) |
              (Paused, Active) |
              (Paused, Tallying) |
              (Paused, Cancelled) => {
                  self.status = new_status;
              },
              _ => return Err,
        }
        同步更新 Polls 中对应的 poll
    }

    pub fn set_deadline(...) {
        if self.status != PollStatus::Active {
            return Err;
        }
        if deadline <= now {
            return Err;
        }
        self.deadline = deadline;
    }
}


#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, RuntimeDebug)]
pub struct ProofWrapper(BoundedVec<u8, ConstU32<256>>);

impl from Proof<Bls12_381> for ProofWrapper;
impl from ProofWrapper for Proof<Bls12_381>;
impl deref Proof<Bls12_381> for ProofWrapper;

#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, RuntimeDebug)]
pub struct VkWrapper(BoundedVec<u8, ConstU32<2048>>);

impl from VerifyingKey<Bls12_381> for VkWrapper;
impl from VkWrapper for VerifyingKey<Bls12_381>;
impl deref VerifyingKey<Bls12_381> for VkWrapper;

#[derive(Clone, Encode, Decode, TypeInfo, PartialEq, Eq, MaxEncodedLen, RuntimeDebug)]
pub struct PublicInputs {
    pub poll_id: u32,
    /// 所有加密投票的哈希（承诺）
    pub encrypted_votes_hash: H256,
    /// 统计结果
    pub tally: BoundedVec<u32, ConstU32<64>>,
}

impl from PublicInputs for Vec<<Bls12_381 as Pairing>::ScalarField>

```
