//! Canonical gRPC contracts for the BeeVulyk platform.
//!
//! One module per proto package, mirroring the `proto/` directory layout.

pub mod beekeeping {
    pub mod reference {
        pub mod v1 {
            tonic::include_proto!("beekeeping.reference.v1");
        }
    }
}

pub mod common {
    pub mod geo {
        pub mod v1 {
            tonic::include_proto!("common.geo.v1");
        }
    }
    pub mod money {
        pub mod v1 {
            tonic::include_proto!("common.money.v1");
        }
    }
}

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

pub mod marketplace {
    pub mod reference {
        pub mod v1 {
            tonic::include_proto!("marketplace.reference.v1");
        }
    }
    pub mod listings {
        pub mod v1 {
            tonic::include_proto!("marketplace.listings.v1");
        }
    }
    pub mod orders {
        pub mod v1 {
            tonic::include_proto!("marketplace.orders.v1");
        }
    }
}
