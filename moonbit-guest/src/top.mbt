///|
pub fn on_load() -> Unit {
	@calculator.log(
		@calculator.LogLevel::info,
		"mod loaded"
	)
}

///|
pub fn on_update(dt : Float) -> Unit {
	let _ = dt
}