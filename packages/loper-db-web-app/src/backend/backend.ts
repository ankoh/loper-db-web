/// Typedef for a local connection id
export type LocalConnectionId = number;
/// Typedef for a local session id
export type LocalSessionId = number;
/// Typedef for a global session id
export type GlobalSessionId = string;

/// An Arrow IPC stream that asynchronously provides IPC message buffers
export interface ArrowIPCStreamReader {
    /// Get the next  Arrow IPC chunk or return null if the end of the stream is reached.
    next(): Promise<Uint8Array | null>;
}

/// A Loper service connection
export interface LoperServiceConnectionDescriptor {
    /// The local connection id used by the client
    localConnectionId: LocalConnectionId;
}

/// A Loper session descriptor
export interface LoperServiceSessionDescriptor {
    /// The global session id sent to loper
    globalSessionId: GlobalSessionId;
    /// The local session id used by the client
    localSessionId: LocalSessionId;
}

/// A Loper service backend
export interface LoperServiceBackend {
    /// Configure the Loper service backen
    configure(): Promise<void>;
    /// Open a connection to a Loper service
    openConnection(host: string): Promise<LoperServiceConnectionDescriptor>;
    /// Close a connection to a Loper service
    closeConnection(conn: LoperServiceConnectionDescriptor): Promise<LoperServiceSessionDescriptor>;
    /// Create a session in a Loper connection
    createSession(conn: LoperServiceConnectionDescriptor): Promise<LoperServiceSessionDescriptor>;
    /// Close a session in a Loper connection
    closeSession(session: LoperServiceSessionDescriptor): Promise<void>;
    /// Execute a query in
    executeQuery(session: LoperServiceSessionDescriptor, query: string): Promise<ArrowIPCStreamReader>;
};

/// A backend for the Loper client application
export interface Backend {
    /// A backend to interact with the Loper gRPC service
    loperService: LoperServiceBackend
};