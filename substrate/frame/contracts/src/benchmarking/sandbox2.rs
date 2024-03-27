use crate::{
	exec::Stack, storage::meter::Meter, wasm::Runtime, BalanceOf, Config, DebugBufferVec,
	Determinism, GasMeter, Origin, Schedule, WasmBlob, Weight,
};
use sp_core::Get;
use wasmi::{Func, Store};

type StackExt<'a, T> = Stack<'a, T, WasmBlob<T>>;

struct Sandbox<T: Config> {
	dest: T::AccountId,
	origin: Origin<T>,
	gas_meter: GasMeter<T>,
	storage_meter: Meter<T>,
	schedule: Schedule<T>,
	value: BalanceOf<T>,
	debug_message: Option<DebugBufferVec<T>>,
	determinism: Determinism,
}

impl<T: Config> Sandbox<T> {
	fn new(dest: T::AccountId, origin: Origin<T>) -> Self {
		Self {
			dest,
			origin,
			gas_meter: GasMeter::new(Weight::MAX),
			storage_meter: Default::default(),
			schedule: T::Schedule::get(),
			value: 0u32.into(),
			debug_message: None,
			determinism: Determinism::Enforced,
		}
	}

	fn new_call<'a>(&'a mut self) -> StackExt<'a, T> {
		Stack::bench_new_call(
			self.dest.clone(),
			self.origin.clone(),
			&mut self.gas_meter,
			&mut self.storage_meter,
			&self.schedule,
			self.value,
			self.debug_message.as_mut(),
			self.determinism,
		)
		.0
	}

	fn prepare_call<'a>(
		&'a mut self,
		ext: &'a mut StackExt<'a, T>,
		module: WasmBlob<T>,
		input: Vec<u8>,
	) -> (Func, Store<Runtime<StackExt<'a, T>>>) {
		module.bench_prepare_call(ext, input)
	}
}
