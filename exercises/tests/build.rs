fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
       .duration_since(std::time::UNIX_EPOCH)
       .unwrap()
       .as_secs(); 
    // 计算一个合适的环境变量值，使得当前时间戳在该值到该值加 10 的范围内
    let test_foo_value = timestamp.saturating_sub(5); 
    println!("cargo:rustc-env=TEST_FOO={}", test_foo_value);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}