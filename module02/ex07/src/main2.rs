// enum ParseError {
//     InvalidWidth { arg: &'static str },
//     InvalidHeight { arg: &'static str },
//     InvalidPercentage { arg: &'static str },
//     TooManyArguments,
//     NotEnoughArguments,
// }

// #[derive(Copy, Clone)]
// enum Cell {
//     Dead,
//     Alive,
// }

// impl Cell {
//     fn is_alive(self) -> bool {
// 		match self {
// 			Cell::Alive => true,
// 			Cell::Dead => false,
// 		}
// 	}

// 	#[allow(dead_code)]
//     fn is_dead(self) -> bool {
// 		match self {
// 			Cell::Alive => false,
// 			Cell::Dead => true,
// 		}
// 	}
// }

// struct Board {
//     width: usize,
//     height: usize,
//     cells: Vec<Cell>,
// }

// impl Board {
//     fn new(width: usize, height: usize, percentage: u32) -> Self {
// 		Self {
// 			width,
// 			height,
// 			cells: (0..width * height)
// 				.map(|_| {
// 					if (ftkit::random_number(0..=100) as u32) < percentage {
// 						Cell::Alive
// 					} else {
// 						Cell::Dead
// 					}
// 				})
// 				.collect(),
// 		}
// 	}

//     fn from_args() -> Result<Self, ParseError> {
// 		if ftkit::ARGS.len() > 4 {
// 			return Err(ParseError::TooManyArguments);
// 		}
		
// 		if ftkit::ARGS.len() < 4 {
// 			return Err(ParseError::NotEnoughArguments);
// 		}
		
// 		let width = match ftkit::ARGS[1].parse() {
// 			Ok(ok) => ok,
// 			Err(_) => { 
// 				return Err(ParseError::InvalidWidth { arg: &ftkit::ARGS[1] })
// 			}
// 		};

// 		let height = match ftkit::ARGS[2].parse() {
// 			Ok(ok) => ok,
// 			Err(_) => { 
// 				return Err(ParseError::InvalidHeight { arg: &ftkit::ARGS[2] })
// 			}
// 		};

// 		let percentage = match ftkit::ARGS[3].parse() {
// 			Ok(ok) => ok,
// 			Err(_) => { 
// 				return Err(ParseError::InvalidPercentage { arg: &ftkit::ARGS[3] })
// 			}
// 		};
// 		return Ok(Self::new(width, height, percentage));
// 	}

// 	fn get(&self, mut x: isize, mut y: isize) -> Cell {
// 		if x < 0 {
// 			x += self.width as isize;
// 		}
// 		let x = x as usize % self.width;
// 		if y < 0 {
// 			y += self.height as isize;
// 		}
// 		let y = y as usize % self.height;
// 		return self.cells[y * self.width + x];

// 	}
//     fn step(&mut self) {
// 		let mut next_board = Vec::new();
// 		for y in 0..self.height as isize {
// 			for x in 0..self.width as isize {
// 				let count = self.get(x - 1, y - 1).is_alive() as u32 
// 					+ self.get(x - 1, y).is_alive() as u32
// 					+ self.get(x - 1, y + 1).is_alive() as u32
// 					+ self.get(x, y - 1).is_alive() as u32
// 					+ self.get(x, y + 1).is_alive() as u32
// 					+ self.get(x + 1, y - 1).is_alive() as u32
// 					+ self.get(x + 1, y).is_alive() as u32
// 					+ self.get(x + 1, y + 1).is_alive() as u32 ;
				
// 				let new_cell = match (self.get(x, y), count) {
// 					(Cell::Alive, 2 | 3) => Cell::Alive,
// 					(Cell::Dead, 3) => Cell::Alive,
// 					_ => Cell::Dead,
// 				};

// 				next_board.push(new_cell) 
// 			}
// 		}
// 		self.cells = next_board ;

// 	}

//     fn print(&self, clear: bool) {
// 		if clear {
//             print!("\x1B[{}A\x1B[J", self.height);
//         }
// 		for y in 0..self.height as isize {
// 			for x in 0..self.width as isize {
// 				match self.get(x, y) {
// 					Cell::Alive => print!("#"),
// 					Cell::Dead => print!(" ")
// 				}
// 			}
// 			println!();
// 		}
// 	}
// }

// fn main() {
// 	let mut board = match Board::from_args() {
// 		Ok(ok) => ok,
// 		Err(err) => {
// 			match err {
// 				ParseError::InvalidWidth { arg } => {
// 					eprintln!("error: '{arg}' is not a valid width")
// 				}
// 				ParseError::InvalidHeight { arg } => {
// 					eprintln!("error: '{arg}' is not a valid height")
// 				}
// 				ParseError::InvalidPercentage { arg } => {
// 					eprintln!("error: '{arg}' is not a valid percentage")
// 				}
// 				ParseError::TooManyArguments => eprintln!("error: too many arguments"),
// 				ParseError::NotEnoughArguments => eprintln!("error: not enough arguments"),
// 			}
// 			return;
// 		}
// 	};
	
// 	loop {
// 		board.print(true);
// 		board.step();
// 		std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
// 	}
// }