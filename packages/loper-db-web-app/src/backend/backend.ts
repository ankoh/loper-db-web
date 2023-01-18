/// Typedef for a local stream reader id
export type LocalStreamReaderId = number;
/// Typedef for a local connection id
export type LocalConnectionId = number;
/// Typedef for a local session id
export type LocalSessionId = number;
/// Typedef for a global session id
export type GlobalSessionId = string;

/// A Loper service backend
export interface LoperServiceBackend {
    /// Configure the Loper service backen
    configure(): Promise<void>;
    /// Open a connection to a Loper service
    createConnection(): Promise<LoperServiceConnection>;
};

/// A Loper service connection
export interface LoperServiceConnection {
    /// The local connection id used by the client
    localConnectionId: LocalConnectionId;
    /// Close a session in a Loper connection
    close(): Promise<void>;
    /// Open a connection to a Loper service
    createSession(): Promise<LoperServiceSession>;
}

/// A Loper session descriptor
export interface LoperServiceSession {
    /// The global session id sent to loper
    globalSessionId: GlobalSessionId;
    /// The local session id used by the client
    localSessionId: LocalSessionId;
    /// Close a connection to a Loper service
    close(): Promise<void>;
    /// Execute a query in
    executeQuery(query: string): Promise<QueryResultStreamReader>;
}

/// A reader for query results
export interface QueryResultStreamReader {
    /// The local connection id used by the client
    localStreamReaderId: LocalStreamReaderId;
    /// Close the result stream reader
    close(): Promise<void>;
    /// Get the next  Arrow IPC chunk or return null if the end of the stream is reached.
    next(): Promise<Uint8Array | null>;
}

/// A backend for the Loper client application
export interface Backend {
    /// A backend to interact with the Loper gRPC service
    loperService: LoperServiceBackend
};