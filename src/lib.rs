const DEBIAN_ARCHIVE_KEYRING: &'static [u8] = include_bytes!("debian-archive-keyring.gpg");
const UBUNTU_ARCHIVE_KEYRING: &'static [u8] = include_bytes!("ubuntu-archive-keyring.gpg");

pub fn supported_debian_keys() -> &'static [u8] {
    DEBIAN_ARCHIVE_KEYRING
}

pub fn supported_ubuntu_keys() -> &'static [u8] {
    UBUNTU_ARCHIVE_KEYRING
}

pub fn supported_keys() -> Vec<u8> {
    let mut ret = Vec::with_capacity(DEBIAN_ARCHIVE_KEYRING.len() + UBUNTU_ARCHIVE_KEYRING.len());
    ret.extend_from_slice(DEBIAN_ARCHIVE_KEYRING);
    ret.extend_from_slice(UBUNTU_ARCHIVE_KEYRING);
    ret
}
