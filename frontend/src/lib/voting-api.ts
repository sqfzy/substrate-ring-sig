/**
 * Ring Signature Voting High-Level API
 *
 * 此文件提供高层封装,整合了:
 * 1. 底层区块链 API (api.ts)
 * 2. WASM 密码学模块 (frontend/crypto)
 *
 * 提供完整的投票流程支持
 */

import { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { ISubmittableResult } from "@polkadot/types/types";
import {
  type BlockNumber,
  type CompressedRistrettoWrapper,
  type Poll,
  type PollId,
  type RingId,
  RingSigVotingAPI,
  type Tally,
} from "./api.ts";

// FIX: 导入 init 函数 (默认导出) 和所有其他导出
// deno-lint-ignore no-sloppy-imports
import init, * as wasm from "./wasm/crypto.js";

// ============================================================================
// WASM 初始化
// ============================================================================

let wasmInitialized = false;

/**
 * 初始化 WASM 模块
 * 必须在调用任何 WASM 函数前执行
 */
export async function initWasm() {
  if (!wasmInitialized) {
    // 默认情况下 init() 会尝试加载同目录下的 .wasm 文件
    // 在 Vite/SvelteKit 中通常能正常工作
    await init();
    wasmInitialized = true;
    console.log("WASM module initialized");
  }
}

// ============================================================================
// 扩展类型定义
// ============================================================================

/**
 * 投票选项
 */
export enum VoteChoice {
  Yes = 0,
  No = 1,
  Abstain = 2,
}

/**
 * 用户密钥对
 */
export interface UserKeyPair {
  privateKey: string; // hex
  publicKey: string; // hex
}

/**
 * 投票参数
 */
export interface VoteParams {
  pollId: PollId;
  choice: VoteChoice;
  userPrivateKey: string; // hex
  ringPublicKeys: string[]; // hex array
  secretIndex: number; // 用户公钥在 ring 中的位置
}

/**
 * 环签名数据
 */
export interface RingSignatureData {
  challenge: string;
  responses: string[];
  keyImage: string;
  ring: string[]; // 包含用户公钥的完整环
}

/**
 * 加密投票数据
 */
export interface EncryptedVoteData {
  ephemeralPublicKey: string;
  ciphertext: Uint8Array;
}

/**
 * 完整的投票提交数据
 */
export interface VoteSubmission {
  pollId: PollId;
  ephemeralPublicKey: string;
  ciphertext: Uint8Array;
  challenge: string;
  responses: string[];
  keyImage: string;
}

/**
 * 解密后的投票
 */
export interface DecryptedVote {
  choice: VoteChoice;
  ephemeralPublicKey: string;
  keyImage: string;
}

// ============================================================================
// 高层 API 类
// ============================================================================

export class VotingAPI {
  baseApi: RingSigVotingAPI;
  private api: ApiPromise;

  constructor(api: ApiPromise) {
    this.api = api;
    this.baseApi = new RingSigVotingAPI(api);
  }

  // ==========================================================================
  // 1. 密钥管理
  // ==========================================================================

  /**
   * 生成新的用户密钥对
   *
   * @returns 用户密钥对
   */
  generateKeyPair(): UserKeyPair {
    const privateKey = wasm.generate_private_key();
    const publicKey = wasm.derive_public_key(privateKey);
    return { privateKey, publicKey };
  }

  /**
   * 从私钥派生公钥
   *
   * @param privateKey 私钥 (hex)
   * @returns 公钥 (hex)
   */
  derivePublicKey(privateKey: string): string {
    return wasm.derive_public_key(privateKey);
  }

  /**
   * 生成密钥镜像
   *
   * @param privateKey 私钥 (hex)
   * @returns 密钥镜像 (hex)
   */
  generateKeyImage(privateKey: string): string {
    return wasm.generate_key_image(privateKey);
  }

  // ==========================================================================
  // 2. 管理员操作
  // ==========================================================================

  /**
   * 注册环签名组
   *
   * @param signer 管理员账户
   * @param publicKeys 公钥数组 (hex)
   * @returns 环签名组 ID
   */
  async registerRing(
    signer: KeyringPair,
    publicKeys: string[],
  ): Promise<RingId> {
    const currentCount = await this.baseApi.getRingCount();
    // FIX: 为公钥添加 0x 前缀，以便 Polkadot.js 正确解析为 Hex 字节
    const formattedKeys = publicKeys.map((k) => this.toHexString(k));
    const tx = this.baseApi.registerRing(formattedKeys);
    await this.baseApi.submitAndWait(tx, signer);
    return currentCount;
  }

  /**
   * 授权老师权限
   *
   * @param signer 管理员账户
   * @param teacherAddress 老师地址
   */
  async authorizeTeacher(
    signer: KeyringPair,
    teacherAddress: string,
  ): Promise<void> {
    const tx = this.baseApi.authorizeTeacher(teacherAddress);
    await this.baseApi.submitAndWait(tx, signer);
  }

  /**
   * 撤销老师权限
   *
   * @param signer 管理员账户
   * @param teacherAddress 老师地址
   */
  async revokeTeacher(
    signer: KeyringPair,
    teacherAddress: string,
  ): Promise<void> {
    const tx = this.baseApi.revokeTeacher(teacherAddress);
    await this.baseApi.submitAndWait(tx, signer);
  }

  /**
   * 手动触发计票
   *
   * @param signer 管理员账户
   * @param pollId 投票 ID
   */
  async startTallying(
    signer: KeyringPair,
    pollId: PollId,
  ): Promise<void> {
    const tx = this.baseApi.tallyPoll(pollId);
    await this.baseApi.submitAndWait(tx, signer);
  }

  // ==========================================================================
  // 3. 投票创建与管理 (老师/管理员)
  // ==========================================================================

  /**
   * 创建投票
   *
   * @param signer 老师或管理员账户
   * @param params 投票参数
   * @returns 投票 ID 和投票私钥
   */
  async createPoll(
    signer: KeyringPair,
    params: {
      ringId: RingId;
      description: string;
      metadata: string;
      deadline: BlockNumber;
    },
  ): Promise<{ pollId: PollId; pollPrivateKey: string }> {
    // 生成投票密钥对
    const pollKeyPair = this.generateKeyPair();

    const currentCount = await this.baseApi.getPollCount();

    // FIX: 为投票公钥添加 0x 前缀
    const tx = this.baseApi.createPoll(
      params.ringId,
      params.description,
      params.metadata,
      params.deadline,
      this.toHexString(pollKeyPair.publicKey),
    );

    await this.baseApi.submitAndWait(tx, signer);

    return {
      pollId: currentCount,
      pollPrivateKey: pollKeyPair.privateKey,
    };
  }

  /**
   * 取消投票
   *
   * @param signer 投票所有者或管理员
   * @param pollId 投票 ID
   * @param reason 取消原因
   */
  async cancelPoll(
    signer: KeyringPair,
    pollId: PollId,
    reason: string,
  ): Promise<void> {
    const tx = this.baseApi.cancelPoll(pollId, reason);
    await this.baseApi.submitAndWait(tx, signer);
  }

  /**
   * 暂停投票
   *
   * @param signer 投票所有者或管理员
   * @param pollId 投票 ID
   * @param reason 暂停原因
   */
  async pausePoll(
    signer: KeyringPair,
    pollId: PollId,
    reason: string,
  ): Promise<void> {
    const tx = this.baseApi.pausePoll(pollId, reason);
    await this.baseApi.submitAndWait(tx, signer);
  }

  /**
   * 激活投票
   *
   * @param signer 投票所有者或管理员
   * @param pollId 投票 ID
   */
  async activePoll(
    signer: KeyringPair,
    pollId: PollId,
  ): Promise<void> {
    const tx = this.baseApi.activePoll(pollId);
    await this.baseApi.submitAndWait(tx, signer);
  }

  /**
   * 更新投票截止时间
   *
   * @param signer 投票所有者或管理员
   * @param pollId 投票 ID
   * @param newDeadline 新截止时间
   */
  async updateDeadline(
    signer: KeyringPair,
    pollId: PollId,
    newDeadline: BlockNumber,
  ): Promise<void> {
    const tx = this.baseApi.setDeadline(pollId, newDeadline);
    await this.baseApi.submitAndWait(tx, signer);
  }

  // ==========================================================================
  // 4. 投票操作 (核心功能)
  // ==========================================================================

  /**
   * 准备投票数据 (链下操作)
   *
   * 此方法执行所有链下密码学操作:
   * 1. 加密投票选择
   * 2. 生成环签名
   *
   * @param params 投票参数
   * @returns 投票提交数据
   */
  async prepareVote(params: VoteParams): Promise<VoteSubmission> {
    // 1. 获取投票信息
    const poll = await this.baseApi.getPoll(params.pollId);
    if (!poll) {
      throw new Error(`Poll ${params.pollId} not found`);
    }

    // 2. 获取 genesis hash (用于 AAD)
    const genesisHash = this.api.genesisHash.toHex();

    // 3. 加密投票选择
    const choiceBytes = new Uint8Array([params.choice]);

    // 构造 AAD: genesis_hash || poll_id
    const aad = this.constructAAD(genesisHash, params.pollId);

    // FIX: 使用 camelCase 属性 (pollPublicKey)
    const [ephemeralPubKey, ciphertextHex] = wasm.encrypt_vote(
      this.ensureHex(poll.pollPublicKey),
      choiceBytes,
      aad,
    );

    const ciphertext = this.hexToUint8Array(ciphertextHex);

    // 4. 构造待签名消息: ephemeral_public_key || ciphertext
    const message = this.concatenateBytes(
      this.hexToUint8Array(ephemeralPubKey),
      ciphertext,
    );

    // FIX: 生成环签名时，底层库期望传入的是"诱饵"（不包含自己的其他成员），
    // 并且会自动将自己的公钥插入到 secretIndex 位置。
    // 因此，我们需要先从完整环中移除自己。
    const decoys = params.ringPublicKeys.filter((_, i) => i !== params.secretIndex);

    // 5. 生成环签名
    const signatureData = this.signWithRing(
      decoys, // 传入诱饵列表
      message,
      params.userPrivateKey,
      params.secretIndex,
    );

    return {
      pollId: params.pollId,
      ephemeralPublicKey: ephemeralPubKey,
      ciphertext,
      challenge: signatureData.challenge,
      responses: signatureData.responses,
      keyImage: signatureData.keyImage,
    };
  }

  /**
   * 提交投票 (链上操作,无签名交易)
   *
   * @param voteData 投票提交数据
   * @returns 交易结果
   */
  submitVote(voteData: VoteSubmission): Promise<ISubmittableResult> {
    // FIX: 为所有 Hex 字符串参数添加 0x 前缀，以便 Polkadot.js 正确编码
    const tx = this.baseApi.vote(
      voteData.pollId,
      this.toHexString(voteData.ephemeralPublicKey),
      voteData.ciphertext,
      this.toHexString(voteData.challenge),
      voteData.responses.map((r) => this.toHexString(r)),
      this.toHexString(voteData.keyImage),
    );

    // 无签名交易,直接发送
    return new Promise((resolve, reject) => {
      tx.send((result: ISubmittableResult) => {
        if (result.status.isInBlock) {
          console.log(`Vote included in block ${result.status.asInBlock}`);
        }

        if (result.status.isFinalized) {
          console.log(`Vote finalized in block ${result.status.asFinalized}`);
          resolve(result);
        }

        if (result.status.isInvalid || result.status.isDropped) {
          reject(new Error("Vote transaction failed"));
        }
      }).catch(reject);
    });
  }

  /**
   * 一键投票 (准备 + 提交)
   *
   * @param params 投票参数
   * @returns 交易结果
   */
  async vote(params: VoteParams): Promise<ISubmittableResult> {
    const voteData = await this.prepareVote(params);
    return await this.submitVote(voteData);
  }

  // ==========================================================================
  // 5. 计票与解密
  // ==========================================================================

  /**
   * 解密所有选票并统计
   *
   * @param pollId 投票 ID
   * @param pollPrivateKey 投票私钥 (hex)
   * @returns 计票结果
   */
  async decryptAndTallyVotes(
    pollId: PollId,
    pollPrivateKey: string,
  ): Promise<{ tally: Tally; decryptedVotes: DecryptedVote[] }> {
    // 1. 获取所有加密投票
    const encryptedVotes = await this.baseApi.getEncryptedVotes(pollId);

    // 2. 获取 genesis hash
    const genesisHash = this.api.genesisHash.toHex();
    const aad = this.constructAAD(genesisHash, pollId);

    // 3. 解密每一票
    const decryptedVotes: DecryptedVote[] = [];
    const tally: Tally = { yes: 0, no: 0, abstain: 0 };

    for (const encVote of encryptedVotes) {
      try {
        // 尝试安全地获取 ciphertext 的 hex
        let ciphertextHex: string;
        if (typeof encVote.ciphertext === "string") {
          // 如果已经是 hex string (Polkadot.js toJSON 常见行为)
          ciphertextHex = this.ensureHex(encVote.ciphertext);
        } else {
          ciphertextHex = this.uint8ArrayToHex(
            encVote.ciphertext as unknown as Uint8Array,
          );
        }

        // FIX: 使用 camelCase 属性 (ephemeralPublicKey)
        const plaintext = wasm.decrypt_vote(
          this.ensureHex(encVote.ephemeralPublicKey),
          ciphertextHex,
          pollPrivateKey,
          aad,
        );

        const choice = plaintext[0] as VoteChoice;

        decryptedVotes.push({
          choice,
          ephemeralPublicKey: this.ensureHex(encVote.ephemeralPublicKey),
          keyImage: this.ensureHex(encVote.keyImage),
        });

        // 统计
        switch (choice) {
          case VoteChoice.Yes:
            tally.yes++;
            break;
          case VoteChoice.No:
            tally.no++;
            break;
          case VoteChoice.Abstain:
            tally.abstain++;
            break;
        }
      } catch (error) {
        console.error(`Failed to decrypt vote:`, error);
        // 跳过无法解密的投票
      }
    }

    return { tally, decryptedVotes };
  }

  /**
   * 公布计票结果
   *
   * @param signer 签名账户 (通常是投票创建者)
   * @param pollId 投票 ID
   * @param tally 计票结果
   * @param pollPrivateKey 投票私钥
   */
  async publishTallyResult(
    signer: KeyringPair,
    pollId: PollId,
    tally: Tally,
    pollPrivateKey: string,
  ): Promise<void> {
    // FIX: 私钥也需要添加 0x 前缀
    const tx = this.baseApi.tally(
      pollId,
      tally,
      this.toHexString(pollPrivateKey),
    );
    await this.baseApi.submitAndWait(tx, signer);
  }

  /**
   * 完整的计票流程
   *
   * 1. 解密所有选票
   * 2. 统计结果
   * 3. 提交到链上
   *
   * @param signer 签名账户
   * @param pollId 投票 ID
   * @param pollPrivateKey 投票私钥
   * @returns 计票结果
   */
  async completeTallyProcess(
    signer: KeyringPair,
    pollId: PollId,
    pollPrivateKey: string,
  ): Promise<Tally> {
    // 1. 解密并统计
    const { tally, decryptedVotes } = await this.decryptAndTallyVotes(
      pollId,
      pollPrivateKey,
    );

    console.log(`Tallied ${decryptedVotes.length} votes:`, tally);

    // 2. 提交结果
    await this.publishTallyResult(signer, pollId, tally, pollPrivateKey);

    return tally;
  }

  // ==========================================================================
  // 6. 查询接口
  // ==========================================================================

  /**
   * 获取投票信息
   */
  async getPoll(pollId: PollId): Promise<Poll | null> {
    return await this.baseApi.getPoll(pollId);
  }

  /**
   * 获取环签名组
   */
  async getRing(ringId: RingId): Promise<CompressedRistrettoWrapper[] | null> {
    return await this.baseApi.getRing(ringId);
  }

  /**
   * 检查用户是否已投票
   */
  async hasUserVoted(
    pollId: PollId,
    userPrivateKey: string,
  ): Promise<boolean> {
    const keyImage = this.generateKeyImage(userPrivateKey);
    // 查询时通常不需要 0x 前缀，或者 Polkadot.js 会处理，
    // 但为了保险起见，如果底层需要编码，可以在 api.ts 层面处理。
    // 这里 API 主要是查询 storage key map，Polkadot.js 会自动编码参数。
    // 如果 keyImage 是纯 hex，可能需要 0x。
    // 让我们加上 0x 以防万一，因为底层是 map key hasher。
    return await this.baseApi.isKeyImageUsed(
      pollId,
      this.toHexString(keyImage),
    );
  }

  /**
   * 获取投票计数
   */
  async getPollCount(): Promise<number> {
    return await this.baseApi.getPollCount();
  }

  /**
   * 获取环组计数
   */
  async getRingCount(): Promise<number> {
    return await this.baseApi.getRingCount();
  }

  /**
   * 检查是否是老师
   */
  async isTeacher(address: string): Promise<boolean> {
    return await this.baseApi.isTeacher(address);
  }

  /**
   * 获取当前区块高度
   */
  async getCurrentBlockNumber(): Promise<BlockNumber> {
    const header = await this.api.rpc.chain.getHeader();
    return header.number.toNumber();
  }

  // ==========================================================================
  // 7. 事件订阅
  // ==========================================================================

  /**
   * 订阅投票事件
   */
  async subscribeEvents(
    callback: (event: {
      section: string;
      method: string;
      data: unknown;
    }) => void,
  ): Promise<() => void> {
    return await this.baseApi.subscribeEvents(callback);
  }

  // ==========================================================================
  // 8. 辅助私有方法
  // ==========================================================================

  /**
   * 确保值为不带 0x 前缀的 Hex 字符串 (用于传给 WASM)
   */
  private ensureHex(val: CompressedRistrettoWrapper): string {
    if (typeof val === "string") {
      return val.replace(/^0x/, "");
    }
    return this.uint8ArrayToHex(val);
  }

  /**
   * 确保值为带 0x 前缀的 Hex 字符串 (用于传给 Polkadot.js)
   * Polkadot.js 需要 0x 前缀来将 hex 字符串正确识别为 bytes
   */
  private toHexString(val: string): string {
    if (val.startsWith("0x")) {
      return val;
    }
    return `0x${val}`;
  }

  /**
   * 生成环签名
   */
  private signWithRing(
    ringPublicKeys: string[],
    message: Uint8Array,
    privateKey: string,
    secretIndex: number,
  ): RingSignatureData {
    // 调用 WASM sign_blsag
    // 返回格式: [Challenge, KeyImage, Ring_0...Ring_N, Response_0...Response_N]
    const result = wasm.sign_blsag(
      ringPublicKeys,
      message,
      privateKey,
      secretIndex,
    );

    // 解析返回值
    const challenge = result[0];
    const keyImage = result[1];

    // 计算环的大小: (总长度 - 2) / 2
    const ringSize = (result.length - 2) / 2;

    const ring = result.slice(2, 2 + ringSize);
    const responses = result.slice(2 + ringSize);

    return {
      challenge,
      responses,
      keyImage,
      ring,
    };
  }

  /**
   * 构造 AAD (Additional Authenticated Data)
   * 格式: genesis_hash || poll_id
   */
  private constructAAD(genesisHash: string, pollId: PollId): Uint8Array {
    const genesisBytes = this.hexToUint8Array(genesisHash.replace("0x", ""));
    const pollIdBytes = new Uint8Array(4);
    new DataView(pollIdBytes.buffer).setUint32(0, pollId, true); // little-endian

    return this.concatenateBytes(genesisBytes, pollIdBytes);
  }

  /**
   * 连接字节数组
   */
  private concatenateBytes(...arrays: Uint8Array[]): Uint8Array {
    const totalLength = arrays.reduce((sum, arr) => sum + arr.length, 0);
    const result = new Uint8Array(totalLength);
    let offset = 0;
    for (const arr of arrays) {
      result.set(arr, offset);
      offset += arr.length;
    }
    return result;
  }

  /**
   * Hex 转 Uint8Array
   */
  private hexToUint8Array(hex: string): Uint8Array {
    const cleaned = hex.replace("0x", "");
    const bytes = new Uint8Array(cleaned.length / 2);
    for (let i = 0; i < bytes.length; i++) {
      bytes[i] = parseInt(cleaned.substr(i * 2, 2), 16);
    }
    return bytes;
  }

  /**
   * Uint8Array 转 Hex
   */
  private uint8ArrayToHex(bytes: Uint8Array): string {
    return Array.from(bytes)
      .map((b) => b.toString(16).padStart(2, "0"))
      .join("");
  }
}

// ============================================================================
// 工厂函数
// ============================================================================

/**
 * 创建 VotingAPI 实例
 *
 * @param api Polkadot.js API 实例
 * @returns VotingAPI 实例
 */
export function createVotingAPI(api: ApiPromise): VotingAPI {
  return new VotingAPI(api);
}
