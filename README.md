## distro-keyrings

[![Build status](https://api.travis-ci.org/FauxFaux/distro-keyring.png)](https://travis-ci.org/FauxFaux/distro-keyring)
[![crates.io](https://img.shields.io/crates/v/distro-keyring.svg)](https://crates.io/crates/distro-keyring)

This package contains the GPG keyring files for various linux distributions.

You might want these if you are trying to download distribution package lists,
similar to [webpki-roots](https://crates.io/crates/webpki-roots) for fetching
things over HTTPS.

If you know you are using a specific distribution, release, or host system,
you should probably rely on the keys distributed with that.

They keys are in [rfc4880](https://tools.ietf.org/html/rfc4880) format, as byte slices.
This is the `.gpg` format which [gnupg](https://www.gnupg.org/) uses for keys
(and many other things). You need an `rfc4880` parser, such a `gpg`, to do anything
useful with them. There are rust bindings for [gpgme](https://crates.io/crates/gpgme),
or you can use a pure rust implementation, such as [gpgrv](https://crates.io/crates/gpgrv),
which is used in the tests.


### Included keys

Debian keys, from `debian-archive-keyring` `2017.7ubuntu1`:

```text
pub   rsa4096 2012-05-08 [SC] [expires: 2019-05-07]
      ED6D65271AACF0FF15D123036FB2A1C265FFB764
uid           Wheezy Stable Release Key <debian-release@lists.debian.org>
pub   rsa4096 2012-04-27 [SC] [expires: 2020-04-25]
      A1BD8E9D78F7FE5C3E65D8AF8B48AD6246925553
uid           Debian Archive Automatic Signing Key (7.0/wheezy) <ftpmaster@debian.org>
pub   rsa4096 2013-08-17 [SC] [expires: 2021-08-15]
      75DDC3C4A499F1A18CB5F3C8CBF8D6FD518E17E1
uid           Jessie Stable Release Key <debian-release@lists.debian.org>
pub   rsa4096 2014-11-21 [SC] [expires: 2022-11-19]
      126C0D24BD8A2942CC7DF8AC7638D0442B90D010
uid           Debian Archive Automatic Signing Key (8/jessie) <ftpmaster@debian.org>
pub   rsa4096 2014-11-21 [SC] [expires: 2022-11-19]
      D21169141CECD440F2EB8DDA9D6D8F6BC857C906
uid           Debian Security Archive Automatic Signing Key (8/jessie) <ftpmaster@debian.org>
pub   rsa4096 2017-05-20 [SC] [expires: 2025-05-18]
      067E3C456BAE240ACEE88F6FEF0F382A1A7B6500
uid           Debian Stable Release Key (9/stretch) <debian-release@lists.debian.org>
pub   rsa4096 2017-05-22 [SC] [expires: 2025-05-20]
      E1CF20DDFFE4B89E802658F1E0B11894F66AEC98
uid           Debian Archive Automatic Signing Key (9/stretch) <ftpmaster@debian.org>
sub   rsa4096 2017-05-22 [S] [expires: 2025-05-20]
pub   rsa4096 2017-05-22 [SC] [expires: 2025-05-20]
      6ED6F5CB5FA6FB2F460AE88EEDA0D2388AE22BA9
uid           Debian Security Archive Automatic Signing Key (9/stretch) <ftpmaster@debian.org>
sub   rsa4096 2017-05-22 [S] [expires: 2025-05-20]
```

License:

> The keys in the keyrings don't fall under any copyright.

---

Ubuntu keys, from `ubuntu-keyring` `2018.09.18.1`:

```text
pub   rsa4096 2012-05-11 [SC]
      790BC7277767219C42C86F933B4FE6ACC0B21F32
uid           Ubuntu Archive Automatic Signing Key (2012) <ftpmaster@ubuntu.com>
pub   rsa4096 2012-05-11 [SC]
      843938DF228D22F7B3742BC0D94AA3F0EFE21092
uid           Ubuntu CD Image Automatic Signing Key (2012) <cdimage@ubuntu.com>
pub   rsa4096 2018-09-17 [SC]
      F6ECB3762474EDA9D21B7022871920D1991BC93C
uid           Ubuntu Archive Automatic Signing Key (2018) <ftpmaster@ubuntu.com>
``` 

License:

> public-domain. The keys in the keyrings don't fall under any copyright.


### License

There is practically no code here, so licensing doesn't really apply.

The keyrings themselves are also probably not subject to licensing. Both have
helpfully explicitly written this in their source.

The crate as a whole is listed as `MIT OR Apache-2.0`, just in case.
