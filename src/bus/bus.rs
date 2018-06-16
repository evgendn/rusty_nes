pub struct Bus {
	cpu: CPU,
	wram: RAM,
}

impl Bus {
	pub fn new(cpu: CPU, wram: RAM) -> Bus {
		Bus {
			cpu,
			wram,
		}
	}
}
