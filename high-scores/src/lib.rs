#[derive(Debug)]
pub struct HighScores<'a> {
    score_list: &'a [u32],
    // sorted_list: &'a Vec<&'a u32>,
    sorted_list: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        // let mut new_scores: &'a Vec<&'a u32> = scores.to_vec();
        // new_scores.extend_from_slice(scores);
        // let mut new_scores: &'a Vec<&'a u32> = vec!(scores);
        // new_scores.sort_by(|a, b| b.cmp(a));
        let mut new_scores: [u32] = *scores;
        new_scores.sort_by(|a, b| b.cmp(a));
        
        HighScores {
            score_list: scores,
            // sorted_list: new_scores,
            sorted_list: new_scores
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.score_list
    }

    pub fn latest(&self) -> Option<u32> {
        match self.score_list.len() {
            0 => None,
            _ => Some(self.score_list[(self.score_list.len() - 1)])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        match self.score_list.len() {
            0 => None,
            _ => Some(self.sorted_list[0])
        }
    }

    // pub fn personal_top_three(&self) -> Vec<&'a u32> {
    //     match self.score_list.len() {
    //         0 => [].to_vec(),
    //         n @ 1..=2 => {
    //             let mut new_vec: Vec<&'a u32> = self.sorted_list;
    //             new_vec.truncate(n);
    //             new_vec
    //         },
    //         _ => {
    //             let mut new_vec: Vec<&'a u32> = self.sorted_list;
    //             new_vec.truncate(3);
    //             new_vec
    //         }
    //     }
    // }
}
