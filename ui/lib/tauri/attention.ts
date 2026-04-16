import { invoke } from './invoke';

export async function clearAttention(sessionId: number): Promise<void> {
  return invoke('clear_attention', { sessionId });
}
