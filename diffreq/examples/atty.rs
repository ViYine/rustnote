fn main() {
    // 直接输出的时候就是atty
    if atty::is(atty::Stream::Stdout) {
        println!("stdout is a tty");
    } else {
        // 如果做了重定向，或者管道，就不是atty
        println!("stdout is not a tty");
    }
}
