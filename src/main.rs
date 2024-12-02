mod Historian_Hysteria{
    pub mod puzzle1;
    pub mod puzzle2;
}

mod Red_Nosed_Reports{
    pub mod puzzle1;
    pub mod puzzle2;
}

mod Mull_It_Over{
    pub mod puzzle1;
    pub mod puzzle2;
}
fn main() -> std::io::Result<()> {
    println!("\n\x1b[1;31mHistorian \x1b[1;32mHysteria \x1b[0m");
    let _ = Historian_Hysteria::puzzle1::solve();
    let _ = Historian_Hysteria::puzzle2::solve();

    println!("\n\x1b[1;31mRed-Nosed \x1b[1;32mReports \x1b[0m");
    let _ = Red_Nosed_Reports::puzzle1::solve();
    let _ = Red_Nosed_Reports::puzzle2::solve();

    println!("\n\x1b[1;31mMull \x1b[1;32mIt \x1b[1;31mOver \x1b[0m");
    let _ = Mull_It_Over::puzzle1::solve();
    let _ = Mull_It_Over::puzzle2::solve();

    println!();
    Ok(())
}
