fn show_output() -> () {
  println!("I should appear as the output.")
}

macro_rules! show_output {
  () => {
    show_output()
  }
}