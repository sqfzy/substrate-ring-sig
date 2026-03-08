- benchmark 测试，看看weight定多少合适 [x]
- 实现`提案生命周期管理`，设定结束日期
  - 到达deadline后，Scheduler将Poll的state设为`Tallying`，需要注意Poll当前状态是否为`Active`。我需要引入Scheduler pallet，并在create_poll时安排一个任务。
- 隐藏投票内容。用承诺需要学生两次操作，不太好，使用tlock自动允许解密？ [x] 弃用
- 也许需要pallet_collective? [x] 弃用
- 提供API，修改deadline。没到deadline，不允许close_poll
- 若计票员提前揭示私钥，则罚没押金 [x] 这在联盟链中行不通
- 提供两种投票方式，用公钥加密，使用承诺（需要自行揭示承诺）[ ] 暂弃用
- 使用ZKP证明上传的tally合法，而不是使用乐观验证模型
  - 链上存：vk
  - 链下存：pk
- Tally数据结构的定义也改为线下(更灵活)，线上只存hash？[x] 目前确定为u32类型 
- 去掉deposit相关代码 [x]
- 使用门限加密，教务处代表，学生会代表，教师工会代表等等共同生成tally公私钥对
- 为struct Poll 实现get_status，记得检查deadline, 实现set_status，确保状态转换合法 [x]
- Poll new() request metadata_hash; drop() unrequest metadata_hash; set_deadline [x]


- 使用pallet_collective管理私钥
- 实现链下生成proof的逻辑，用于test和benchmark通过

- 修改mock.rs tests.rs
- 增加zkp的测试
- 完成zkp.d2
