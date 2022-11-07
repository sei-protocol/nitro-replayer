use nitro_replayer::replay::*;

pub fn main() {
    let mut accounts: Vec<ByteSliceView> = vec![];
    // for i in 0..44 {
    //     accounts.push(ByteSliceView::from_str(&format!("/Users/tonychen/repos/nitro-merkle/proofgenerator/formatted/accounts/{}", i)));
    // }
    let mut sysvar_accounts: Vec<ByteSliceView> = vec![];
    // for i in 0..44 {
    //     sysvar_accounts.push(ByteSliceView::from_str(&format!("/Users/tonychen/repos/nitro-merkle/proofgenerator/formatted/sysvar/{}", i)));
    // }
    let mut programs: Vec<ByteSliceView> = vec![];
    // for i in 0..15 {
    //     programs.push(ByteSliceView::from_str(&format!("/Users/tonychen/repos/nitro-merkle/proofgenerator/formatted/programs/{}", i)));
    // }
    let mut transactions: Vec<ByteSliceView> = vec![];
    // for i in 0..6 {
    //     transactions.push(ByteSliceView::from_str(&format!("/Users/tonychen/repos/nitro-merkle/proofgenerator/formatted/txs/{}", i)));
    // }
    let output_directory = ByteSliceView::from_str("/Users/tonychen/repos/nitro-replayer/investigation/");
    replay(
        FilePaths::from_vec(&accounts),
        FilePaths::from_vec(&sysvar_accounts),
        FilePaths::from_vec(&programs),
            FilePaths::from_vec(&transactions),
        output_directory,
    );
}
