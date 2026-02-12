///|
pub fn on_load() -> Unit {
  @log.log(@log.Level::INFO, "mod", "mod loaded!!!")
}

///|
pub fn on_update(dt : Float) -> Unit {
  @log.log(@log.Level::INFO, "mod", "dt=\{dt}")
}
