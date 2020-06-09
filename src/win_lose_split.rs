pub struct WinLoseSplit {
    pub wins: i32,
    pub losses: i32,
    pub splits: i32
}

impl WinLoseSplit {
    pub fn new() -> WinLoseSplit {
        WinLoseSplit {
            wins: 0,
            losses: 0,
            splits: 0
        }
    }

    pub fn total(&self) -> i32{
        self.wins + self.losses + self.splits
    }

    pub fn win_pct(&self) -> f32 {
        100.0 * (self.wins as f32) / (self.total() as f32)
    } 

    pub fn losses_pct(&self) -> f32 {
        100.0 * (self.losses as f32) / (self.total() as f32)
    }

    pub fn splits_pct(&self) -> f32 {
        100.0 * (self.splits as f32) / (self.total() as f32)
    }
}

impl Copy for WinLoseSplit {}

impl Clone for WinLoseSplit {
    fn clone(&self) -> Self {
        WinLoseSplit {
            wins: self.wins,
            losses: self.losses,
            splits: self.splits
        }
    }
}

impl std::fmt::Display for WinLoseSplit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.2}% - {:.2}% - {:.2}%", self.win_pct(), self.losses_pct(), self.splits_pct())
    }
}
