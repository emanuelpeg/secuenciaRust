use rand::{thread_rng, Rng};

pub fn iniciar_secuencia() -> [u16;4] {

	let semilla:u8 = thread_rng().gen_range(0..3);

	match semilla {
	    0 => return generar_secuencia_par(),
	    1 => return generar_secuencia_inpar(),
        _ => return generar_fibonacci()
	}
}

fn generar_secuencia_par() -> [u16;4] {
	let mut resultado:[u16;4] = [0; 4];
	let semilla:u16 = thread_rng().gen_range(0..30);
	for i in 0..4 {
		resultado[i] = semilla * 2 + (i as u16) * 2;
	}
	resultado
}

fn generar_secuencia_inpar() -> [u16;4] {
	let mut resultado:[u16;4] = [0; 4];
	let semilla:u16 = thread_rng().gen_range(0..30);
	for i in 0..4 {
		resultado[i] = semilla * 2 + (i as u16) * 2 + 1;
	}
	resultado
}


fn generar_fibonacci() -> [u16;4] {
	let mut _resultado:[u16;4] = [0; 4];
	let semilla:u16 = thread_rng().gen_range(0..30);

	for i in 0..4 {
		if i < 2 {
			_resultado[i] = semilla;
		} else {
			_resultado[i] = _resultado[i-1] + _resultado[i-2];
		}
	}
	_resultado
}