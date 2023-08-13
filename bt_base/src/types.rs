// Internal Libraries


// External Libraries

use chrono::{NaiveDate, Local};

#[derive(Debug)]
pub enum GameInfo {
	Partial(NaiveDate),
	Full(NaiveDate, u8),
}

impl GameInfo {
	// Constructor
	pub fn build_part() -> Self {
		Self::Partial(Local::now().date_naive())
	}

	pub fn build_full() -> Self {
		Self::Full(Local::now().date_naive(), 1)
	}

	pub fn build_date(date: NaiveDate) -> Self {
		Self::Partial(date)
	}

	pub fn build_with(date: NaiveDate, game: u8) -> Self {
		Self::Full(date, game)
	}

	// Getter
	pub fn date(&self) -> &NaiveDate {
		match self {
			Self::Partial(date) | Self::Full(date, _) => date,
		}
	}

	pub fn game(&self) -> u8 {
		match self {
			Self::Full(_, game) => *game,
			_ => 0,
		}
	}

	pub fn date_mut(&mut self) -> &mut NaiveDate {
		match self {
			Self::Partial(date) | Self::Full(date, _) => date,
		}
	}

	pub fn game_mut(&mut self) -> &mut u8 {
		match self {
			Self::Full(_, game) => game,
			_ => panic!()
		}
	}
}

// `PartialEq` impl
impl std::cmp::PartialEq for GameInfo {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Partial(s_date), Self::Partial(o_date)) => s_date == o_date,
			(Self::Full(s_date, s_game), Self::Full(o_date, o_game)) => s_date == o_date && s_game == o_game,
			_ => false
		}
	}
}

// `From` impl
impl std::convert::From<NaiveDate> for GameInfo {
	fn from(value: NaiveDate) -> Self {
		Self::Partial(value)
	}
}
pub enum Frame {
	Uninit,
	TwoFrame(u8, u8),
	ThreeFrame(u8, u8, u8),
}

impl Frame {
	// Constructor
	pub fn build() -> Self {
		Self::Uninit
	}

	// Getter
	pub fn frame(&self) -> &Self {
		self
	}

	pub fn throw(&self, throw_num: usize) -> Option<u8> {
		match self {
			Self::Uninit => None,
			Self::TwoFrame(t1, t2) => {
				if throw_num == 1 {
					Some(*t1)
				} else if throw_num == 2 {
					Some(*t2)
				} else {
					None
				}
			}
			Self::ThreeFrame(t1, t2, t3) => {
				if throw_num == 1 {
					Some(*t1)
				} else if throw_num == 2 {
					Some(*t2)
				} else if throw_num == 3 {
					Some(*t3)
				} else {
					None
				}
			}
		}
	}

	pub fn frame_mut(&mut self) -> &mut Self {
		self
	}

	// Type-specific
	pub fn valid(&self) -> bool {
		!matches!(self, Self::Uninit)
	}

	pub fn score(&self) -> u8 {
		match self {
			Self::TwoFrame(t1, t2) => t1 + t2,
			Self::ThreeFrame(t1, t2, t3) => t1 + t2 + t3,
			_ => 0,
		}
	}

	pub fn strike(&self) -> bool {
		matches!(self.throw(1), Some(10))
	}

	pub fn spare(&self) -> bool {
		match (self.throw(1), self.throw(2)) {
			(Some(t1), Some(t2)) => t1 + t2 == 10,
			_ => false,
		}
	}
}

// `From` impl
impl std::convert::From<(u8, u8)> for Frame {
	fn from(value: (u8, u8)) -> Self {
		Self::TwoFrame(value.0, value.1)
	}
}

impl std::convert::From<(u8, u8, u8)> for Frame {
	fn from(value: (u8, u8, u8)) -> Self {
		Self::ThreeFrame(value.0, value.1, value.2)
	}
}

// `PartialEq` impl
impl std::cmp::PartialEq for Frame {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::TwoFrame(s_t1, s_t2), Self::TwoFrame(o_t1, o_t2)) => {
				s_t1 == o_t1 && s_t2 == o_t2
			}
			(Self::ThreeFrame(s_t1, s_t2, s_t3), Self::ThreeFrame(o_t1, o_t2, o_t3)) => {
				s_t1 == o_t1 && s_t2 == o_t2 && s_t3 == o_t3
			}
			(Self::Uninit, Self::Uninit) => true,
			_ => false,
		}
	}
}

pub struct Game {
	date: NaiveDate,
	game: u8,
	frames: Vec<Frame>,
}

// Constructor
impl Game {
	pub fn build() -> Game {
		Game {
			date: Local::now().date_naive(),
			game: 1,
			frames: Vec::with_capacity(10),
		}
	}

	pub fn build_date(date: NaiveDate) -> Game {
		Game {
			date,
			game: 1,
			frames: Vec::with_capacity(10),
		}
	}

	pub fn build_with(info: GameInfo) -> Game {
		match info {
			GameInfo::Partial(date) => Game {
				date,
				game: 1,
				frames: Vec::with_capacity(10),
			},
			GameInfo::Full(date, game) => Game {
				date,
				game,
				frames: Vec::with_capacity(10),
			},
		}
	}
}

// Getters
impl Game {
	pub fn date(&self) -> &NaiveDate {
		&self.date
	}

	pub fn game(&self) -> u8 {
		self.game
	}

	pub fn frame(&self, frame_num: usize) -> Option<&Frame> {
		if !(1..=10).contains(&frame_num) {
			return None;
		}

		Some(&self.frames[frame_num - 1])
	}

	pub fn frames(&self) -> &Vec<Frame> {
		&self.frames
	}

	pub fn date_mut(&mut self) -> &mut NaiveDate {
		&mut self.date
	}

	pub fn game_mut(&mut self) -> &mut u8 {
		&mut self.game
	}

	pub fn frame_mut(&mut self, frame_num: usize) -> Option<&mut Frame> {
		if !(1..=10).contains(&frame_num) {
			return None;
		}

		Some(&mut self.frames[frame_num - 1])
	}

	pub fn frames_mut(&mut self) -> &Vec<Frame> {
		&mut self.frames
	}
}

// Type impl
impl Game {
	pub fn valid(&self) -> bool {
		true
	}
}
