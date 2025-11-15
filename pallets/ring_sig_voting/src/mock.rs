use crate as ring_sig;
use frame::{prelude::*, runtime::prelude::*, testing_prelude::*};

type Block = frame_system::mocking::MockBlock<Test>;

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
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type Balances = pallet_balances;

    #[runtime::pallet_index(2)]
    pub type Preimage = pallet_preimage;

    #[runtime::pallet_index(3)]
    pub type RingSigVoting = ring_sig_voting;
}


impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Self>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
    type DoneSlashHandler = ();
}


parameter_types! {
    pub const PreimageHoldReason: RuntimeHoldReason =
        RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
    pub const PreimageBaseDeposit: Balance = 1 * MILLI_UNIT;
    pub const PreimageByteDeposit: Balance = 1 * MICRO_UNIT;
}

impl pallet_preimage::Config for Test {
    type WeightInfo = pallet_preimage::weights::SubstrateWeight<Self>;
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type ManagerOrigin = EnsureRoot<AccountId>;
    type Consideration = HoldConsideration<
        AccountId,
        Balances,
        PreimageHoldReason,
        LinearStoragePrice<PreimageBaseDeposit, PreimageByteDeposit, Balance>,
    >;
}

// System pallet configuration
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = Block;
}

impl ring_sig_voting::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Preimages = pallet_preimage::Pallet<Runtime>;
    type SubmissionDeposit = ConstU128<{ 10 * MICRO_UNIT }>;
    type CreatePollOrigin = frame_system::EnsureSigned<AccountId>;
    type ClosePollOrigin = EnsureRoot<AccountId>;
    type RingAdminOrigin = frame_system::EnsureSigned<AccountId>;
    type MaxDescriptionLength = ConstU32<256>;
    type MaxMembersInRing = ConstU32<16>;
    type NumRingLayers = ConstU32<2>;
    type WeightInfo = ring_sig_voting::weights::SubstrateWeight<Runtime>;
}

// Test externalities initialization
pub fn new_test_ext() -> TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
