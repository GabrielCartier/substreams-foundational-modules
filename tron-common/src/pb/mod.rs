// @generated
// @@protoc_insertion_point(attribute:protocol)
pub mod protocol {
    include!("protocol.rs");
    // @@protoc_insertion_point(protocol)
}
pub mod sf {
    pub mod substreams {
        pub mod tron {
            // @@protoc_insertion_point(attribute:sf.substreams.tron.v1)
            pub mod v1 {
                include!("sf.substreams.tron.v1.rs");
                // @@protoc_insertion_point(sf.substreams.tron.v1)
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
    pub mod tron {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.tron.type.v1)
            pub mod v1 {
                include!("sf.tron.type.v1.rs");
                // @@protoc_insertion_point(sf.tron.type.v1)
            }
        }
    }
}
