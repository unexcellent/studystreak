pub enum Attempt {
    Correct,
    Incorrect,
    WithHelp,
    PartiallyCorrect(u32, u32)
}