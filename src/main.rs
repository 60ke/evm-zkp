use evm::Config;
use std::{collections::BTreeMap, str::FromStr};
use primitive_types::{H160, U256};
use evm::executor::stack::{PrecompileFn,PrecompileFailure,StackSubstateMetadata,PrecompileOutput,MemoryStackState,StackExecutor};
use evm::Context;
use evm::backend::{MemoryAccount, MemoryBackend, MemoryVicinity};

// pub type PrecompileFn =
// 	fn(&[u8], Option<u64>, &Context, bool) -> Result<(PrecompileOutput, u64), PrecompileFailure>;

fn test(input:&[u8],gas_limit:Option<u64>, context:&Context, is_static:bool)-> Result<(PrecompileOutput, u64), PrecompileFailure>{
    let out = PrecompileOutput{
        exit_status:evm::ExitSucceed::Returned,
        output: "test".as_bytes().to_vec(),
    };
    return Ok((out,64))
}


fn main() {
    let config = Config::istanbul();

	let vicinity = MemoryVicinity {
		gas_price: U256::zero(),
		origin: H160::default(),
		block_hashes: Vec::new(),
		block_number: Default::default(),
		block_coinbase: Default::default(),
		block_timestamp: Default::default(),
		block_difficulty: Default::default(),
		block_gas_limit: Default::default(),
		chain_id: U256::one(),
		block_base_fee_per_gas: U256::zero(),
	};

	let mut state = BTreeMap::new();
	state.insert(
		H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
		MemoryAccount {
			nonce: U256::one(),
			balance: U256::from(10000000i128),
			storage: BTreeMap::new(),
			code: hex::decode("6080604052348015600f57600080fd5b506004361060285760003560e01c80630f14a40614602d575b600080fd5b605660048036036020811015604157600080fd5b8101908080359060200190929190505050606c565b6040518082815260200191505060405180910390f35b6000806000905060005b83811015608f5760018201915080806001019150506076565b508091505091905056fea26469706673582212202bc9ec597249a9700278fe4ce78da83273cb236e76d4d6797b441454784f901d64736f6c63430007040033").unwrap(),
		}
	);
	state.insert(
		H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
		MemoryAccount {
			nonce: U256::one(),
			balance: U256::from(10000000i128),
			storage: BTreeMap::new(),
			code: Vec::new(),
		},
	);
	let backend = MemoryBackend::new(&vicinity, state);
	let metadata = StackSubstateMetadata::new(u64::MAX, &config);
	let state = MemoryStackState::new(metadata, &backend);    

    println!("Hello, world!");
    let mut pre = BTreeMap::new();
    let v:PrecompileFn = test;
    // let context = Context { address: H160::from_str("0x000000000000000000000000000000000000001").unwrap(), caller: H160::from_str("0x000000000000000000000000000000000000101").unwrap(), apparent_value: U256::from_str("0x5").unwrap() };
    pre.insert(H160::from_str("0x0000000000000000000000000000000000000001").unwrap(), v);
    println!("{:?}",config);


    // let precompiles = BTreeMap::new();

	let mut executor = StackExecutor::new_with_precompiles(state, &config, &pre);

	let ret = executor.transact_call(
		H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
		H160::from_str("0x0000000000000000000000000000000000000001").unwrap(),
		U256::zero(),
		hex::decode("0f14a4060000000000000000000000000000000000000000000000000000000000b71b00")
			.unwrap(),
		// hex::decode("0f14a4060000000000000000000000000000000000000000000000000000000000002ee0").unwrap(),
		u64::MAX,
		Vec::new(),
	);
    println!("ret: {:?}",ret)
}



