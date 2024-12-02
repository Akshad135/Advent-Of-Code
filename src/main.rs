mod Historian_Hysteria{
    pub mod puzzle1;
    pub mod puzzle2;
}

mod Red_Nosed_Reports{
    pub mod puzzle1;
    pub mod puzzle2;
}
fn main() -> std::io::Result<()> {
    println!("\nHistorian Hysteria");
    let _ = Historian_Hysteria::puzzle1::solve();
    let _ = Historian_Hysteria::puzzle2::solve();

    println!("\nRed-Nosed Reports");
    let _ = Red_Nosed_Reports::puzzle1::solve();
    let _ = Red_Nosed_Reports::puzzle2::solve();


    Ok(())
}
