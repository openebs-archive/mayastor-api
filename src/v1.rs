///!module to access v1 version of grpc APIs

// dont export the raw pb generated code
mod pb {
    #![allow(unknown_lints)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    include!(concat!(env!("OUT_DIR"), "/mayastor.v1.rs"));
}

/// v1 version of bdev grpc API
pub mod bdev {
    pub use super::pb::{
        bdev_rpc_client::BdevRpcClient,
        bdev_rpc_server::{BdevRpc, BdevRpcServer},
        Bdev, BdevShareRequest, BdevShareResponse, BdevUnshareRequest, CreateBdevRequest,
        CreateBdevResponse, DestroyBdevRequest, ListBdevOptions, ListBdevResponse,
    };
}

/// v1 version of json-rpc grpc API
pub mod json {
    pub use super::pb::{
        json_rpc_client::JsonRpcClient,
        json_rpc_server::{JsonRpc, JsonRpcServer},
        JsonRpcRequest, JsonRpcResponse,
    };
}

pub mod pool {
    pub use super::pb::{
        pool_rpc_client::PoolRpcClient,
        pool_rpc_server::{PoolRpc, PoolRpcServer},
        CreatePoolRequest, DestroyPoolRequest, ExportPoolRequest, ImportPoolRequest,
        ListPoolOptions, ListPoolsResponse, Pool, PoolState, PoolType,
    };
}

pub mod replica {
    pub use super::pb::{
        replica_rpc_client::ReplicaRpcClient,
        replica_rpc_server::{ReplicaRpc, ReplicaRpcServer},
        CreateReplicaRequest, DestroyReplicaRequest, ListReplicaOptions, ListReplicasResponse,
        Replica, ShareReplicaRequest, UnshareReplicaRequest,
    };
}
pub mod registration {
    pub use super::pb::{registration_client, ApiVersion, DeregisterRequest, RegisterRequest};
}
pub mod host {
    pub use super::pb::{
        block_device::{Filesystem, Partition},
        host_rpc_client::HostRpcClient,
        host_rpc_server::{HostRpc, HostRpcServer},
        BlockDevice, GetMayastorResourceUsageResponse, ListBlockDevicesRequest,
        ListBlockDevicesResponse, ListNvmeControllersResponse, MayastorFeatures,
        MayastorInfoResponse, NvmeController, NvmeControllerIoStats, NvmeControllerState,
        ResourceUsage, StatNvmeControllerRequest, StatNvmeControllerResponse,
    };
}

pub mod nexus {
    pub use super::pb::{
        nexus_rpc_client::NexusRpcClient,
        nexus_rpc_server::{NexusRpc, NexusRpcServer},
        AddChildNexusRequest, AddChildNexusResponse, Child, ChildAction, ChildOperationRequest,
        ChildOperationResponse, ChildState, ChildStateReason, CreateNexusRequest,
        CreateNexusResponse, DestroyNexusRequest, FaultNexusChildRequest, GetNvmeAnaStateRequest,
        GetNvmeAnaStateResponse, InjectNexusFaultRequest, InjectedFault,
        ListInjectedNexusFaultsReply, ListInjectedNexusFaultsRequest, ListNexusOptions,
        ListNexusResponse, Nexus, NexusState, NvmeAnaState, PauseRebuildRequest,
        PauseRebuildResponse, PublishNexusRequest, PublishNexusResponse, RebuildStateRequest,
        RebuildStateResponse, RebuildStatsRequest, RebuildStatsResponse, RemoveChildNexusRequest,
        RemoveChildNexusResponse, RemoveInjectedNexusFaultRequest, ResumeRebuildRequest,
        ResumeRebuildResponse, SetNvmeAnaStateRequest, SetNvmeAnaStateResponse,
        StartRebuildRequest, StartRebuildResponse, StopRebuildRequest, StopRebuildResponse,
        UnpublishNexusRequest, UnpublishNexusResponse,
    };
}
