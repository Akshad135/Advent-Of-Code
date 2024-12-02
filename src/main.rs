mod Historian_Hysteria{
    pub mod puzzle1;
    pub mod puzzle2;
}

fn main() -> std::io::Result<()> {
    let _ = Historian_Hysteria::puzzle1::solve();
    let _ = Historian_Hysteria::puzzle2::solve();

    Ok(())
}
