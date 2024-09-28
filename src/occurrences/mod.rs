pub mod foundation {
    pub mod v1 {
        tonic::include_proto!("foundation.v1");
    }
}
pub mod spatial_planning {
    pub mod v1 {
        tonic::include_proto!("spatial_planning.v1");
    }
}

pub mod occurrence {
    pub mod v1 {
        tonic::include_proto!("occurrence.v1");
    }
    pub mod c1 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c1.v1");
        }
    }
    pub mod c2 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c2.v1");
        }
    }
    pub mod c3 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c3.v1");
        }
    }
    pub mod c4 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c4.v1");
        }
    }

    pub mod c9 {
        pub mod v1 {
            tonic::include_proto!("occurrence.c9.v1");
        }
    }
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("file_descriptor_set");

mod convert;
