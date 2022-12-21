use lazy_static::lazy_static;
use packaging_version::Version;

lazy_static! {
    static ref VERSIONS: Vec<&'static str> = [
        // Implicit epoch of 0
        "1.0.dev456",
        "1.0a1",
        "1.0a2.dev456",
        "1.0a12.dev456",
        "1.0a12",
        "1.0b1.dev456",
        "1.0b2",
        "1.0b2.post345.dev456",
        "1.0b2.post345",
        "1.0b2-346",
        "1.0c1.dev456",
        "1.0c1",
        "1.0rc2",
        "1.0c3",
        "1.0",
        "1.0.post456.dev34",
        "1.0.post456",
        "1.1.dev1",
        "1.2+123abc",
        "1.2+123abc456",
        "1.2+abc",
        "1.2+abc123",
        "1.2+abc123def",
        "1.2+1234.abc",
        "1.2+123456",
        "1.2.r32+123456",
        "1.2.rev33+123456",
        // Explicit epoch of 1
        "1!1.0.dev456",
        "1!1.0a1",
        "1!1.0a2.dev456",
        "1!1.0a12.dev456",
        "1!1.0a12",
        "1!1.0b1.dev456",
        "1!1.0b2",
        "1!1.0b2.post345.dev456",
        "1!1.0b2.post345",
        "1!1.0b2-346",
        "1!1.0c1.dev456",
        "1!1.0c1",
        "1!1.0rc2",
        "1!1.0c3",
        "1!1.0",
        "1!1.0.post456.dev34",
        "1!1.0.post456",
        "1!1.1.dev1",
        "1!1.2+123abc",
        "1!1.2+123abc456",
        "1!1.2+abc",
        "1!1.2+abc123",
        "1!1.2+abc123def",
        "1!1.2+1234.abc",
        "1!1.2+123456",
        "1!1.2.r32+123456",
        "1!1.2.rev33+123456",
    ].to_vec();
}

#[test]
fn test_valid_versions() {
    for &version in VERSIONS.iter() {
        let v = Version::parse(version).expect(&format!("should be able to parse '{}'", version));
        println!("{:?}", v);
    }
}

#[test]
fn test_invalid_version() {
    let invalids = [
        "french toast",
        // Versions with invalid local versions
        "1.0+a+",
        "1.0++",
        "1.0+_foobar",
        "1.0+foo&asd",
        "1.0+1+1",
    ];

    for invalid in invalids {
        if let Some(v) = Version::parse(invalid) {
            panic!("invalid version spec parsed: {} was parsed to {:?}", invalid, v);
        }
    }
}

#[test]
fn test_normalized_versions() {
    let combinations = [
        // # Various development release incarnations
        ("1.0dev", "1.0.dev0"),
        ("1.0.dev", "1.0.dev0"),
        ("1.0dev1", "1.0.dev1"),
        ("1.0dev", "1.0.dev0"),
        ("1.0-dev", "1.0.dev0"),
        ("1.0-dev1", "1.0.dev1"),
        ("1.0DEV", "1.0.dev0"),
        ("1.0.DEV", "1.0.dev0"),
        ("1.0DEV1", "1.0.dev1"),
        ("1.0DEV", "1.0.dev0"),
        ("1.0.DEV1", "1.0.dev1"),
        ("1.0-DEV", "1.0.dev0"),
        ("1.0-DEV1", "1.0.dev1"),
        // # Various alpha incarnations
        ("1.0a", "1.0a0"),
        ("1.0.a", "1.0a0"),
        ("1.0.a1", "1.0a1"),
        ("1.0-a", "1.0a0"),
        ("1.0-a1", "1.0a1"),
        ("1.0alpha", "1.0a0"),
        ("1.0.alpha", "1.0a0"),
        ("1.0.alpha1", "1.0a1"),
        ("1.0-alpha", "1.0a0"),
        ("1.0-alpha1", "1.0a1"),
        ("1.0A", "1.0a0"),
        ("1.0.A", "1.0a0"),
        ("1.0.A1", "1.0a1"),
        ("1.0-A", "1.0a0"),
        ("1.0-A1", "1.0a1"),
        ("1.0ALPHA", "1.0a0"),
        ("1.0.ALPHA", "1.0a0"),
        ("1.0.ALPHA1", "1.0a1"),
        ("1.0-ALPHA", "1.0a0"),
        ("1.0-ALPHA1", "1.0a1"),
        // # Various beta incarnations
        ("1.0b", "1.0b0"),
        ("1.0.b", "1.0b0"),
        ("1.0.b1", "1.0b1"),
        ("1.0-b", "1.0b0"),
        ("1.0-b1", "1.0b1"),
        ("1.0beta", "1.0b0"),
        ("1.0.beta", "1.0b0"),
        ("1.0.beta1", "1.0b1"),
        ("1.0-beta", "1.0b0"),
        ("1.0-beta1", "1.0b1"),
        ("1.0B", "1.0b0"),
        ("1.0.B", "1.0b0"),
        ("1.0.B1", "1.0b1"),
        ("1.0-B", "1.0b0"),
        ("1.0-B1", "1.0b1"),
        ("1.0BETA", "1.0b0"),
        ("1.0.BETA", "1.0b0"),
        ("1.0.BETA1", "1.0b1"),
        ("1.0-BETA", "1.0b0"),
        ("1.0-BETA1", "1.0b1"),
        // # Various release candidate incarnations
        ("1.0c", "1.0rc0"),
        ("1.0.c", "1.0rc0"),
        ("1.0.c1", "1.0rc1"),
        ("1.0-c", "1.0rc0"),
        ("1.0-c1", "1.0rc1"),
        ("1.0rc", "1.0rc0"),
        ("1.0.rc", "1.0rc0"),
        ("1.0.rc1", "1.0rc1"),
        ("1.0-rc", "1.0rc0"),
        ("1.0-rc1", "1.0rc1"),
        ("1.0C", "1.0rc0"),
        ("1.0.C", "1.0rc0"),
        ("1.0.C1", "1.0rc1"),
        ("1.0-C", "1.0rc0"),
        ("1.0-C1", "1.0rc1"),
        ("1.0RC", "1.0rc0"),
        ("1.0.RC", "1.0rc0"),
        ("1.0.RC1", "1.0rc1"),
        ("1.0-RC", "1.0rc0"),
        ("1.0-RC1", "1.0rc1"),
        // # Various post release incarnations
        ("1.0post", "1.0.post0"),
        ("1.0.post", "1.0.post0"),
        ("1.0post1", "1.0.post1"),
        ("1.0post", "1.0.post0"),
        ("1.0-post", "1.0.post0"),
        ("1.0-post1", "1.0.post1"),
        ("1.0POST", "1.0.post0"),
        ("1.0.POST", "1.0.post0"),
        ("1.0POST1", "1.0.post1"),
        ("1.0POST", "1.0.post0"),
        ("1.0r", "1.0.post0"),
        ("1.0rev", "1.0.post0"),
        ("1.0.POST1", "1.0.post1"),
        ("1.0.r1", "1.0.post1"),
        ("1.0.rev1", "1.0.post1"),
        ("1.0-POST", "1.0.post0"),
        ("1.0-POST1", "1.0.post1"),
        ("1.0-5", "1.0.post5"),
        ("1.0-r5", "1.0.post5"),
        ("1.0-rev5", "1.0.post5"),
        // # Local version case insensitivity
        ("1.0+AbC", "1.0+abc"),
        // # Integer Normalization
        ("1.01", "1.1"),
        ("1.0a05", "1.0a5"),
        ("1.0b07", "1.0b7"),
        ("1.0c056", "1.0rc56"),
        ("1.0rc09", "1.0rc9"),
        ("1.0.post000", "1.0.post0"),
        ("1.1.dev09000", "1.1.dev9000"),
        ("00!1.2", "1.2"),
        ("0100!0.0", "100!0.0"),
        //# Various other normalizations
        ("v1.0", "1.0"),
        ("   v1.0\t\n", "1.0"),
    ];

    for (input, expected) in combinations {
        let parsed = Version::parse(input).unwrap();
        assert_eq!(format!("{}", parsed), expected, "input {} was wrongly parsed ({:?})", input, parsed);
    }
}

#[test]
fn test_version_str_repr() {
    let combinations = [
        ("1.0.dev456", "1.0.dev456"),
        ("1.0a1", "1.0a1"),
        ("1.0a2.dev456", "1.0a2.dev456"),
        ("1.0a12.dev456", "1.0a12.dev456"),
        ("1.0a12", "1.0a12"),
        ("1.0b1.dev456", "1.0b1.dev456"),
        ("1.0b2", "1.0b2"),
        ("1.0b2.post345.dev456", "1.0b2.post345.dev456"),
        ("1.0b2.post345", "1.0b2.post345"),
        ("1.0rc1.dev456", "1.0rc1.dev456"),
        ("1.0rc1", "1.0rc1"),
        ("1.0", "1.0"),
        ("1.0.post456.dev34", "1.0.post456.dev34"),
        ("1.0.post456", "1.0.post456"),
        ("1.0.1", "1.0.1"),
        ("0!1.0.2", "1.0.2"),
        ("1.0.3+7", "1.0.3+7"),
        ("0!1.0.4+8.0", "1.0.4+8.0"),
        ("1.0.5+9.5", "1.0.5+9.5"),
        ("1.2+1234.abc", "1.2+1234.abc"),
        ("1.2+123456", "1.2+123456"),
        ("1.2+123abc", "1.2+123abc"),
        ("1.2+123abc456", "1.2+123abc456"),
        ("1.2+abc", "1.2+abc"),
        ("1.2+abc123", "1.2+abc123"),
        ("1.2+abc123def", "1.2+abc123def"),
        ("1.1.dev1", "1.1.dev1"),
        ("7!1.0.dev456", "7!1.0.dev456"),
        ("7!1.0a1", "7!1.0a1"),
        ("7!1.0a2.dev456", "7!1.0a2.dev456"),
        ("7!1.0a12.dev456", "7!1.0a12.dev456"),
        ("7!1.0a12", "7!1.0a12"),
        ("7!1.0b1.dev456", "7!1.0b1.dev456"),
        ("7!1.0b2", "7!1.0b2"),
        ("7!1.0b2.post345.dev456", "7!1.0b2.post345.dev456"),
        ("7!1.0b2.post345", "7!1.0b2.post345"),
        ("7!1.0rc1.dev456", "7!1.0rc1.dev456"),
        ("7!1.0rc1", "7!1.0rc1"),
        ("7!1.0", "7!1.0"),
        ("7!1.0.post456.dev34", "7!1.0.post456.dev34"),
        ("7!1.0.post456", "7!1.0.post456"),
        ("7!1.0.1", "7!1.0.1"),
        ("7!1.0.2", "7!1.0.2"),
        ("7!1.0.3+7", "7!1.0.3+7"),
        ("7!1.0.4+8.0", "7!1.0.4+8.0"),
        ("7!1.0.5+9.5", "7!1.0.5+9.5"),
        ("7!1.1.dev1", "7!1.1.dev1"),
    ];
    for (input, expected) in combinations {
        let parsed = Version::parse(input).unwrap();
        assert_eq!(format!("{}", parsed), expected, "input {} was wrongly parsed ({:?})", input, parsed);
    }
}

#[test]
fn test_version_rc_and_c_equals() {
    assert_eq!(Version::parse("1.0rc1").unwrap(), Version::parse("1.0c1").unwrap());
}

#[test]
fn test_lt_combinatorial() {
    for (i, x) in VERSIONS.iter().enumerate() {
        for y in &VERSIONS[i+1..] {
            let x_v = Version::parse(x).unwrap();
            let y_v = Version::parse(y).unwrap();
            assert!(x_v < y_v, "{} < {} was false ({:?} / {:?})", x, y, x_v.key(), y_v.key());
        }
    }
}

#[test]
fn test_le_combinatorial() {
    for (i, x) in VERSIONS.iter().enumerate() {
        for y in &VERSIONS[i..] {
            let x_v = Version::parse(x).unwrap();
            let y_v = Version::parse(y).unwrap();
            assert!(x_v <= y_v, "{} < {} was false ({:?} / {:?})", x, y, x_v.key(), y_v.key());
        }
    }
}

#[test]
fn test_ne_combinatorial() {
    for (i, x) in VERSIONS.iter().enumerate() {
        for (j, y) in VERSIONS.iter().enumerate() {
            if i != j {
                let x_v = Version::parse(x).unwrap();
                let y_v = Version::parse(y).unwrap();
                assert_ne!(x_v, y_v, "{} != {} was false ({:?} / {:?})", x, y, x_v.key(), y_v.key());
            }
        }
    }
}

#[test]
fn test_gt_combinatorial() {
    for (i, x) in VERSIONS.iter().enumerate() {
        for y in &VERSIONS[..i] {
            let x_v = Version::parse(x).unwrap();
            let y_v = Version::parse(y).unwrap();
            assert!(x_v > y_v, "{} > {} was false ({:?} / {:?})", x, y, x_v.key(), y_v.key());
        }
    }
}

#[test]
fn test_ge_combinatorial() {
    for (i, x) in VERSIONS.iter().enumerate() {
        for y in &VERSIONS[..i + 1] {
            let x_v = Version::parse(x).unwrap();
            let y_v = Version::parse(y).unwrap();
            assert!(x_v >= y_v, "{} >= {} was false ({:?} / {:?})", x, y, x_v.key(), y_v.key());
        }
    }
}
