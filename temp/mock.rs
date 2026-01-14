pub const RING_SIZE: usize = 10;
pub const SECRET_INDEX: usize = 3;
#[cfg(test)]
pub const DEADLINE: u64 = 100;
#[cfg(feature = "runtime-benchmarks")]
pub const DEADLINE: u32 = 100;
pub const POLL_DESCRIPTION: &[u8] = b"Test Poll";
pub const POLL_METADATA: &[u8] = b"Poll metadata";
pub const SIMPLE_CIPHERTEXT: &[u8] = &[1, 2, 3, 4, 5];


#[cfg(test)]
pub mod tests {
    use super::*;
    use frame::testing_prelude::*;
    use polkadot_sdk::{pallet_preimage, pallet_scheduler};

    pub const ALICE: u64 = 1;
    pub const BOB: u64 = 2;

    // Configure a mock runtime to test the pallet.
    #[frame_construct_runtime]
    mod runtime {
        #[runtime::runtime]
        #[runtime::derive(
            RuntimeCall,
            RuntimeEvent,
            RuntimeError,
            RuntimeOrigin,
            RuntimeFreezeReason,
            RuntimeHoldReason,
            RuntimeSlashReason,
            RuntimeLockId,
            RuntimeTask,
            RuntimeViewFunction
        )]
        pub struct Test;

        #[runtime::pallet_index(0)]
        pub type System = frame_system;

        #[runtime::pallet_index(1)]
        pub type Preimage = pallet_preimage;

        #[runtime::pallet_index(2)]
        pub type Scheduler = pallet_scheduler;

        #[runtime::pallet_index(3)]
        pub type RingSigVoting = ring_sig_voting;
    }

    parameter_types! {
        pub MaxWeight: Weight = Weight::from_parts(2_000_000_000_000, u64::MAX);
        pub const TallyVkBytes: &'static [u8] = include_bytes!("../../srs12.bin");
    }
    ord_parameter_types! {
        pub const AliceAccount: u64 = ALICE;
    }

    // System pallet configuration
    #[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
    impl frame_system::Config for Test {
        type Block = MockBlock<Test>;
    }

    impl pallet_preimage::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type WeightInfo = ();
        type Currency = ();
        type ManagerOrigin = EnsureRoot<u64>;
        type Consideration = ();
    }
    impl pallet_scheduler::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type RuntimeOrigin = RuntimeOrigin;
        type PalletsOrigin = OriginCaller;
        type RuntimeCall = RuntimeCall;
        type MaximumWeight = MaxWeight;
        type ScheduleOrigin = EnsureRoot<u64>;
        type MaxScheduledPerBlock = ConstU32<100>;
        type WeightInfo = ();
        type OriginPrivilegeCmp = EqualPrivilegeOnly;
        type Preimages = Preimage;
        type BlockNumberProvider = frame_system::Pallet<Test>;
    }

    impl ring_sig_voting::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type RuntimeCall = RuntimeCall;
        type Scheduler = Scheduler;
        type Preimages = Preimage;
        type MaxDescriptionLength = ConstU32<256>;
        type MaxRingSize = ConstU32<16>;
        type MaxVkLength = ConstU32<2048>;
        type MaxCiphertextLength = ConstU32<128>;
        type MaxVoteNum = ConstU32<1000>;
        type AdminOrigin = EnsureSignedBy<AliceAccount, u64>;
    }

    // Test externalities initialization
    pub fn new_test_ext() -> TestExternalities {
        let storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        storage.into()
    }
}
