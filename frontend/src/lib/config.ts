/**
 * 应用配置
 */

export const config = {
  // WebSocket 节点地址
  wsUrl: import.meta.env.VITE_WS_URL || 'ws://localhost:9944',

  // 区块时间 (秒)
  blockTime: 6,

  // 默认投票持续时间 (区块数)
  defaultVotingDuration: 1000,

  // 链信息
  chain: {
    name: 'Ring Signature Voting Chain',
    version: '1.0.0',
  },

  // 测试账户
  testAccounts: {
    admin: '//Alice',
    teacher: '//Bob',
    student: '//Charlie',
  },
};
