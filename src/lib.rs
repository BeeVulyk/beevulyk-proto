//! Canonical gRPC contracts for the BeeVulyk platform.
//!
//! One module per proto package, mirroring the `proto/` directory layout.

pub mod identity {
    pub mod users {
        pub mod v1 {
            tonic::include_proto!("identity.users.v1");
        }
    }
    pub mod profiles {
        pub mod v1 {
            tonic::include_proto!("identity.profiles.v1");
        }
    }
}
