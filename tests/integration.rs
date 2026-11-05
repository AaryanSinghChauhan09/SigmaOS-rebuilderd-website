use sigmaos_rebuilderd_website::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_rebuilderd_website::new("sigmaos-rebuilderd-website").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-rebuilderd-website"));
}
