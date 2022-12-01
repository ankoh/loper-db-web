import * as proto from "../../loper-db-proto-es/dist/loper-db-proto.module";
import * as bufconnect from "@bufbuild/connect-web";

interface ScriptTokens {
    /// The offsets
    offsets: Uint32Array;
    /// The type tags
    types: Uint8Array;
}

class LoperServiceConnection {
    /// The service client
    service: LoperServiceClient;

    constructor(service: LoperServiceClient) {
        this.service = service;
    }

    /// Disconnect from loper service
    public async disconnect(): Promise<number> {
        return 42;
    }
    /// Run a query
    public async runQuery(text: string): Promise<Uint8Array[]> {
        const request = new proto.service_pb.QueryParam({
            query: text
        });
        let buffers = [];
        for await (const response of this.service.client.executeQuery(request)) {
            switch (response.result.case) {
                case "arrowIpcDataChunk": {
                    buffers.push(response.result.value.data);
                }
            }
        }
        return buffers;
    }
}

export class LoperServiceClient {
    client: bufconnect.PromiseClient<typeof proto.service_grpc.HyperDatabaseService>;
    public url: string;

    constructor(url: string, credentials?: RequestCredentials) {
        this.url = url;
        const transport = bufconnect.createGrpcWebTransport({
            baseUrl: url,
            credentials
        });
        this.client = bufconnect.createPromiseClient(proto.service_grpc.HyperDatabaseService, transport);
    }

    /// Get version
    public async getVersion(): Promise<string> { return "someversion"; }
    /// Tokenize a text
    public async tokenize(text: string): Promise<ScriptTokens> {
        return { offsets: new Uint32Array(), types: new Uint8Array() }
    }
    /// Connect to loper service
    public async connect(): Promise<LoperServiceConnection> {
        return new LoperServiceConnection(this);
    }
};