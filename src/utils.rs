pub fn banner(address: &str) {
    println!(
        r#"
    ╔══════════════════════════════════════╗
      🚀 Axum Server Launched
      🌍 Listening on: http://{address}
      🛠️  Ready and awaiting requests...
    ╚══════════════════════════════════════╝
    "#,
        address = &address
    );
}
