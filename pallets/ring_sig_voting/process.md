椭圆曲线使用 **Curve25519**
对称加密算法使用 **ChaCha20-Poly1305**
使用 **ECDH** 生成共享密钥，用于加密投票
可链接签名使用 **BLSAG**


# 系统主体
- **管理员**：授权某个账户为教师；注册公钥环；处理争议
- **教师**：发起/暂停/取消投票；揭示计票结果
- **学生**：提交匿名投票；审计计票结果

# 系统流程
## 创建环
### 学生（线下）
1. 生成一对公私钥（student_pk, student_sk），公钥提交给管理员，私钥自己保存。

### 管理员
1. **线下**收集每个班级中所有人的公钥，组成环，存储在链上
2. 执行**register_ring**
  1. 将环成员的公钥列表存储到链上。

## 创建投票
### 老师
1. **线下**选定 Poll 对应的：
  - ring：环成员
  - deadline：截止日期
  - description：描述
  - metadata_hash：相关资料的哈希值，例如调查问卷与计票公式的哈希值，防止学生被钓鱼。
  - poll_public_key：学生用来加密投票的公钥。
2. 执行**create_poll**
  1. 将 Poll 相关信息存储在链上。

## 投票
### 学生
1. **线下**
  1. 生成随机临时公钥 `ephemeral_public_key = r * G`，从区块链获取 `ring`。
  2. 计算共享密钥 `shared_key = r * poll_public_key`。关联数据 `AAD = genesis_hash || poll_id || key_image` 
  3. 使用 `shared_key` 和 ChaCha20-Poly1305 算法加密投票内容，得到密文 `ciphertext`。
  4. 构造待签名消息为 `message = ephemeral_public_key || ciphertext`。
  5. 生成 BLSAG 签名 `BLSAG(ring, message, student_sk)`，得到 `challenge`, `responses`。
2. 选取一个**平凡账户**执行**vote**
  1. 验证 BLSAG 签名以及 `key_image` 是否唯一。
  2. 将加密投票存储在链上

## 乐观计票
### `ephemeral_private_key` 持有人（通常是教师）
1. **线下**
  1. 从链上获取该 Poll 对应的 [(ephemeral_public_key, ciphertext, key_image)]。
  2. `shared_key = ephemeral_public_key * poll_public_key`。`AAD = genesis_hash || poll_id || key_image`。
  3. 使用 `shared_key` 和 ChaCha20-Poly1305 算法解密 `ciphertext`，得到投票内容 `plaintext`。
  4. 统计投票结果。系统不约束问卷形式与计票方式，但统计结果必须是一个字符串（例如 "100"，"优秀"或者存储为JSON格式的字符串）。
2. 执行**tally**
  1. 将计票结果和 `poll_private_key` 存储在链上

## 全民验票
### 任何人（线下）
1. 使用公示的 `poll_private_key`，自己重新计票。
2. 若对计票结果，则需要向管理员反应。

### 管理员（线下）
1. 若收到计票异议，管理员需要调查核实，必要时重新计票。
2. 若确实计票有误，管理员需要执行**change_owner** 接管异常的 Poll，之后可以暂停或取消 Poll，也可以重新提交计票结果。

  
# 安全性
## 学生不能投两次票
每个 Poll 对应一个公钥环，以及一个 Key Image 集合。投票时，区块链会验证 Key Image 是否合法，以及是否唯一。

## 学生不能在非授权班级投票
学生在非授权班级投票时，由于该 Poll 的公钥环不包含其公钥，因此他无法通过 BLSAG 签名验证。

## 避免学生跟风投票
投票内容使用 ChaCha20-Poly1305 加密，只有学生自己以及课程老师才拥有共享密钥，能解密投票内容，其它学生无法获知投票内容。

## 管理员和教师不能修改学生投票
投票内容存储在区块链上，任何人都无法篡改。

## 管理员和教师不能追踪投票来源
`vote` 调用允许调用者不对交易进行签名，因此学生可以“冒用”其它人的账户进行投票（例如规定所有人都使用 Alice 账户进行投票），因此在网络层面，签名者的身份无法被追踪。
投票时使用 BLSAG 签名，因此签名者身份无法通过公钥追踪。

## 审计投票结果
任何人都可以使用公示的 `poll_private_key` 重新计票，确保计票结果的正确性。

## 防止重放
`AAD = genesis_hash || poll_id || key_image` 意味着每一张加密投票都仅限本区块链、本次 Poll以及本人。学生不能通过复制别人的投票密文来盲目照抄别人的投票内容。
