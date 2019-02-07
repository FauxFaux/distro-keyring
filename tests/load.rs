use std::io;

#[test]
fn load_debian() {
    let mut keyring = gpgrv::Keyring::new();
    keyring
        .append_keys_from(io::Cursor::new(distro_keyring::supported_debian_keys()))
        .unwrap();

    assert!(
        !keyring.keys_with_id(0xe0b11894f66aec98).is_empty(),
        "stretch automatic signing key should be present"
    );
    assert!(
        !keyring.keys_with_id(0xeda0d2388ae22ba9).is_empty(),
        "stretch security signing key should be present"
    );
}

#[test]
fn load_ubuntu() {
    let mut keyring = gpgrv::Keyring::new();
    keyring
        .append_keys_from(io::Cursor::new(distro_keyring::supported_ubuntu_keys()))
        .unwrap();

    assert!(
        !keyring.keys_with_id(0x871920d1991bc93c).is_empty(),
        "ubuntu 2018 key should be present"
    );
}

#[test]
fn load_combined() {
    let mut keyring = gpgrv::Keyring::new();
    keyring
        .append_keys_from(io::Cursor::new(distro_keyring::supported_keys()))
        .unwrap();

    assert!(
        !keyring.keys_with_id(0x871920d1991bc93c).is_empty(),
        "ubuntu 2018 key should be present"
    );
}
