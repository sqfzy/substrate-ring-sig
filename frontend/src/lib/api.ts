/**
 * Ring Signature Voting Pallet API
 *
 * 此文件提供了与后端 pallets/ring_sig_voting 模块对应的前端接口封装
 * 基于 Polkadot.js API 实现
 */

import { ApiPromise } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
import type { SubmittableExtrinsic } from "@polkadot/api/types";
import type { ISubmittableResult } from "@polkadot/types/types";
import type { Option, u32, Vec } from "@polkadot/types";
import type { Codec } from "@polkadot/types/types";
import type { EventRecord } from "@polkadot/types/interfaces";

// ============================================================================
// 类型定义
// ============================================================================

export type RingId = number;
export type PollId = number;
export type BlockNumber = number;

/**
 * 压缩的 Ristretto 点 (32 字节公钥)
 */
export type CompressedRistrettoWrapper = Uint8Array | string;

/**
 * Scalar 包装器 (32 字节私钥/标量)
 */
export type ScalarWrapper = Uint8Array | string;

/**
 * 投票状态
 */
export enum PollStatus {
  Active = "Active",
  Tallying = "Tallying",
  Completed = "Completed",
  Cancelled = "Cancelled",
  Paused = "Paused",
}

/**
 * 计票结果
 */
export interface Tally {
  yes: number;
  no: number;
  abstain: number;
}

/**
 * 投票信息
 * FIX: Polkadot.js toJSON() 会自动将 rust 的 snake_case 转为 camelCase
 */
export interface Poll {
  pollId: PollId;        // poll_id -> pollId
  ringId: RingId;        // ring_id -> ringId
  owner: string;
  description: string;
  metadataHash: string;  // metadata_hash -> metadataHash
  deadline: BlockNumber;
  pollPublicKey: CompressedRistrettoWrapper; // poll_public_key -> pollPublicKey
  status: PollStatus;
  tally?: Tally;
  // 注意：这个字段通常不在链上存储的 Poll 结构中，而是在创建时返回，
  // 但为了兼容性保留定义，使用 camelCase
  pollPrivateKey?: ScalarWrapper; 
}

/**
 * 加密投票
 */
export interface EncryptedVote {
  ephemeralPublicKey: CompressedRistrettoWrapper; // ephemeral_public_key -> ephemeralPublicKey
  ciphertext: Uint8Array | string; // Polkadot.js 可能返回 hex string
  keyImage: CompressedRistrettoWrapper; // key_image -> keyImage
}

/**
 * BLSAG 签名
 */
export interface BLSAGSignature {
  challenge: ScalarWrapper;
  responses: ScalarWrapper[];
  keyImage: CompressedRistrettoWrapper; // key_image -> keyImage
}

// ============================================================================
// API 类
// ============================================================================

export class RingSigVotingAPI {
  private api: ApiPromise;

  constructor(api: ApiPromise) {
    this.api = api;
  }

  // ==========================================================================
  // 管理员接口 (需要 AdminOrigin 权限)
  // ==========================================================================

  /**
   * 注册环签名组
   */
  registerRing(
    ring: CompressedRistrettoWrapper[],
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.registerRing(ring);
  }

  /**
   * 授权老师权限
   */
  authorizeTeacher(
    teacherAddress: string,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.authorizeTeacher(teacherAddress);
  }

  /**
   * 撤销老师权限
   */
  revokeTeacher(
    teacherAddress: string,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.revokeTeacher(teacherAddress);
  }

  /**
   * 手动触发计票状态 (停止投票)
   */
  tallyPoll(
    pollId: PollId,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.tallyPoll(pollId);
  }

  /**
   * 强制更改投票所有权
   */
  changeOwner(
    pollId: PollId,
    newOwner: string,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.changeOwner(pollId, newOwner);
  }

  // ==========================================================================
  // 老师/管理员接口 (需要 Teacher 或 AdminOrigin 权限)
  // ==========================================================================

  /**
   * 创建投票
   */
  createPoll(
    ringId: RingId,
    description: string,
    metadata: string,
    deadline: BlockNumber,
    pollPublicKey: CompressedRistrettoWrapper,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.createPoll(
      ringId,
      description,
      metadata,
      deadline,
      pollPublicKey,
    );
  }

  // ==========================================================================
  // 投票所有者接口 (需要是投票的 owner 或 AdminOrigin)
  // ==========================================================================

  /**
   * 取消投票
   */
  cancelPoll(
    pollId: PollId,
    reason: string,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.cancelPoll(pollId, reason);
  }

  /**
   * 暂停投票
   */
  pausePoll(
    pollId: PollId,
    reason: string,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.pausePoll(pollId, reason);
  }

  /**
   * 激活投票
   */
  activePoll(
    pollId: PollId,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.activePoll(pollId);
  }

  /**
   * 设置投票截止时间
   */
  setDeadline(
    pollId: PollId,
    newDeadline: BlockNumber,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.setDeadline(pollId, newDeadline);
  }

  // ==========================================================================
  // 公开接口 (无需特殊权限)
  // ==========================================================================

  /**
   * 提交投票 (无签名交易)
   */
  vote(
    pollId: PollId,
    ephemeralPublicKey: CompressedRistrettoWrapper,
    ciphertext: Uint8Array | number[],
    challenge: ScalarWrapper,
    responses: ScalarWrapper[],
    keyImage: CompressedRistrettoWrapper,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    // 确保 ciphertext 是 Vec<u8> 格式 (数组)
    const ciphertextArr = Array.isArray(ciphertext) ? ciphertext : Array.from(ciphertext);
    
    return this.api.tx.ringSigVoting.vote(
      pollId,
      ephemeralPublicKey,
      ciphertextArr,
      challenge,
      responses,
      keyImage,
    );
  }

  /**
   * 公布计票结果和私钥
   */
  tally(
    pollId: PollId,
    claimedTally: Tally,
    privateKey: ScalarWrapper,
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.ringSigVoting.tally(pollId, claimedTally, privateKey);
  }

  // ==========================================================================
  // 查询接口 (只读)
  // ==========================================================================

  /**
   * 查询环签名组
   */
  async getRing(ringId: RingId): Promise<CompressedRistrettoWrapper[] | null> {
    const ring = (await this.api.query.ringSigVoting.rings(
      ringId,
    )) as unknown as Option<Vec<Codec>>;
    if (ring.isSome) {
      return ring.unwrap().toJSON() as CompressedRistrettoWrapper[];
    }
    return null;
  }

  /**
   * 查询投票信息
   */
  async getPoll(pollId: PollId): Promise<Poll | null> {
    const poll = (await this.api.query.ringSigVoting.polls(
      pollId,
    )) as unknown as Option<Codec>;
    if (poll.isSome) {
      // FIX: 转换为 Poll 类型时，属性已经是 camelCase
      return poll.unwrap().toJSON() as unknown as Poll;
    }
    return null;
  }

  /**
   * 查询加密的投票
   */
  async getEncryptedVotes(pollId: PollId): Promise<EncryptedVote[]> {
    const votes = await this.api.query.ringSigVoting.encryptedVotes(pollId);
    const votesJson = votes.toJSON();
    if (Array.isArray(votesJson)) {
      return votesJson as unknown as EncryptedVote[];
    }
    return [];
  }

  /**
   * 检查密钥镜像是否已使用
   */
  async isKeyImageUsed(
    pollId: PollId,
    keyImage: CompressedRistrettoWrapper,
  ): Promise<boolean> {
    const used = (await this.api.query.ringSigVoting.usedKeyImages(
      pollId,
      keyImage,
    )) as unknown as Option<Codec>;
    return used.isSome;
  }

  /**
   * 查询投票计数器
   */
  async getPollCount(): Promise<number> {
    const count =
      (await this.api.query.ringSigVoting.pollCount()) as unknown as u32;
    return count.toNumber();
  }

  /**
   * 查询环签名组计数器
   */
  async getRingCount(): Promise<number> {
    const count =
      (await this.api.query.ringSigVoting.ringCount()) as unknown as u32;
    return count.toNumber();
  }

  /**
   * 检查账户是否是授权老师
   */
  async isTeacher(address: string): Promise<boolean> {
    const teacher = (await this.api.query.ringSigVoting.teachers(
      address,
    )) as unknown as Option<Codec>;
    return teacher.isSome;
  }

  // ==========================================================================
  // 事件订阅接口
  // ==========================================================================

  async subscribeEvents(
    callback: (event: {
      section: string;
      method: string;
      data: unknown;
    }) => void,
  ): Promise<() => void> {
    const unsubscribe = await this.api.query.system.events(
      (events: Vec<EventRecord>) => {
        events.forEach((record: EventRecord) => {
          const { event } = record;
          if (event.section === "ringSigVoting") {
            callback({
              section: event.section,
              method: event.method,
              data: event.data.toJSON(),
            });
          }
        });
      },
    );
    return unsubscribe as unknown as () => void;
  }

  // ==========================================================================
  // 辅助方法
  // ==========================================================================

  submitAndWait(
    tx: SubmittableExtrinsic<"promise", ISubmittableResult>,
    signer: KeyringPair,
  ): Promise<ISubmittableResult> {
    return new Promise((resolve, reject) => {
      tx.signAndSend(signer, (result: ISubmittableResult) => {
        if (result.status.isInBlock) {
          console.log(
            `Transaction included in block ${result.status.asInBlock}`,
          );
        }

        if (result.status.isFinalized) {
          console.log(
            `Transaction finalized in block ${result.status.asFinalized}`,
          );
          resolve(result);
        }

        if (result.status.isInvalid || result.status.isDropped) {
          reject(new Error("Transaction failed"));
        }
      }).catch(reject);
    });
  }

  batchSubmit(
    txs: SubmittableExtrinsic<"promise", ISubmittableResult>[],
  ): SubmittableExtrinsic<"promise", ISubmittableResult> {
    return this.api.tx.utility.batch(txs);
  }
}

// ============================================================================
// 工厂函数
// ============================================================================

export function createRingSigVotingAPI(api: ApiPromise): RingSigVotingAPI {
  return new RingSigVotingAPI(api);
}
