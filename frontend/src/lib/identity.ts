import { writable, get } from 'svelte/store';
import { generate_private_key, derive_public_key } from './wasm';
import { addLog } from './store';

export const myPrivateKey = writable<string>('');
export const myPublicKey = writable<string>('');

/**
 * 生成新身份
 */
export function generateNewIdentity() {
    try {
        const priv = generate_private_key();
        const pub = derive_public_key(priv);
        
        myPrivateKey.set(priv);
        myPublicKey.set(pub);
        
        addLog('KEYGEN', `新身份已生成: ${pub.slice(0, 8)}...`);
    } catch (e) {
        addLog('ERROR', `生成身份失败: ${e}`);
    }
}

/**
 * 从私钥推导公钥 (用于用户手动粘贴私钥时)
 */
export function restoreIdentity(privKey: string) {
    try {
        if (!privKey) return;
        const pub = derive_public_key(privKey);
        myPrivateKey.set(privKey);
        myPublicKey.set(pub);
        addLog('KEYGEN', `身份已恢复: ${pub.slice(0, 8)}...`);
    } catch (e) {
        addLog('ERROR', `无效的私钥: ${e}`);
    }
}
