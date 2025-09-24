export type SendTxParams = {
  from: string;
  to: string;
  value: string;
  blobId?: string;
};

export class TeqlaClient {
  rpcUrl: string;

  constructor({ rpcUrl }: { rpcUrl: string }) {
    this.rpcUrl = rpcUrl;
  }

  async connect(): Promise<void> {
    // TODO: handshake / status fetch
    return;
  }

  async sendTx(params: SendTxParams): Promise<string> {
    // TODO: perform PoUW loop + reference tips from node
    return "0x" + Math.random().toString(16).slice(2).padEnd(64, "0");
  }

  async awaitFinality(txid: string, { timeoutMs = 30000 } = {}): Promise<{ status: 'finalized' | 'pending' | 'reverted' }> {
    // TODO: poll node for checkpoint inclusion
    await new Promise(r => setTimeout(r, Math.min(timeoutMs, 500)));
    return { status: "finalized" };
  }

  async query(filter: Record<string, any>): Promise<any[]> {
    // TODO: query indexer (REST/GraphQL)
    return [];
  }
}
