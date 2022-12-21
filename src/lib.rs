use std::cmp::Ordering;
use lazy_static::lazy_static;
use regex::{Regex, Match};

lazy_static! {
    static ref VERSION_PATTERN: Regex = Regex::new(r#"(?xi)
    v?
    (?:
        (?:(?P<epoch>[0-9]+)!)?                           # epoch
        (?P<release>[0-9]+(?:\.[0-9]+)*)                  # release segment
        (?P<pre>                                          # pre-release
            [-_\.]?
            (?P<pre_l>(a|b|c|rc|alpha|beta|pre|preview))
            [-_\.]?
            (?P<pre_n>[0-9]+)?
        )?
        (?P<post>                                         # post release
            (?:-(?P<post_n1>[0-9]+))
            |
            (?:
                [-_\.]?
                (?P<post_l>post|rev|r)
                [-_\.]?
                (?P<post_n2>[0-9]+)?
            )
        )?
        (?P<dev>                                          # dev release
            [-_\.]?
            (?P<dev_l>dev)
            [-_\.]?
            (?P<dev_n>[0-9]+)?
        )?
    )
    (?:\+(?P<local>[a-z0-9]+(?:[-_\.][a-z0-9]+)*))?       # local version
"#).unwrap();
    static ref LOCAL_VERSION_SEPARATOR: Regex = Regex::new(r#"[\._-]"#).unwrap();
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum PrePostDevType<'v> {
    NegativeInf,
    Tuple(&'v str, u64),
    PositiveInf,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum LocalTypePart {
    AlphanumVersion(String),
    NumericVersion(u64),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum LocalType<'v> {
    NegativeInf,
    ConcreteVersion(&'v [LocalTypePart]),
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct VersionCmpKey<'v> {
    epoch: u64,
    release: &'v [u64],
    pre: PrePostDevType<'v>,
    post: PrePostDevType<'v>,
    dev: PrePostDevType<'v>,
    local: LocalType<'v>
}


#[derive(Debug)]
pub struct Version {
    pub epoch: u64,
    pub release: Vec<u64>,
    pub pre: Option<(String, u64)>,
    pub post: Option<(String, u64)>,
    pub dev: Option<(String, u64)>,
    pub local: Option<Vec<LocalTypePart>>
}

impl<'v> From<&'v Version> for VersionCmpKey<'v> {
    fn from(value: &'v Version) -> Self {
        let release_len = value.release.len();
        let mut last_nonzero_elem_cursor = release_len;
        while last_nonzero_elem_cursor > 0 && value.release[last_nonzero_elem_cursor - 1] == 0 {
            last_nonzero_elem_cursor -= 1;
        }
        let release = &value.release[..last_nonzero_elem_cursor];

        let pre = if let Some((s, u)) = &value.pre {
            PrePostDevType::Tuple(s.as_str(), *u)
        } else {
            if value.post.is_none() && value.dev.is_some() {
                PrePostDevType::NegativeInf
            } else {
                PrePostDevType::PositiveInf
            }
        };

        let post = if let Some((s, u)) = &value.post {
            PrePostDevType::Tuple(s.as_str(), *u)
        } else {
            PrePostDevType::NegativeInf
        };

        let dev = if let Some((s, u)) = &value.post {
            PrePostDevType::Tuple(s.as_str(), *u)
        } else {
            PrePostDevType::PositiveInf
        };

        let local = if let Some(local) = &value.local {
            LocalType::ConcreteVersion(local.as_slice())
        } else {
            LocalType::NegativeInf
        };

        VersionCmpKey {
            epoch: value.epoch,
            release,
            pre,
            post,
            dev,
            local
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        let cmp_key: VersionCmpKey = self.into();
        cmp_key.eq(&other.into())
    }
}

impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp_key: VersionCmpKey = self.into();
        cmp_key.partial_cmp(&other.into())
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_key: VersionCmpKey = self.into();
        cmp_key.cmp(&other.into())
    }
}

fn parse_letter_version<'a>(letter: Option<Match<'a>>, number: Option<Match<'a>>) -> Option<(String, u64)> {
    if let Some(letter) = letter {
        let letter = letter.as_str().to_ascii_lowercase();
        let number: u64 = number.map(|m| m.as_str().parse().unwrap()).unwrap_or(0);

        let canonical_letter = match letter.as_str() {
            "alpha" => "a",
            "beta" => "b",
            "c" | "pre" | "preview" => "rc",
            "rev" | "r" => "post",
            other => other
        };

        Some((canonical_letter.to_string(), number))
    } else {
        if let Some(m) = number {
            let number: u64 = m.as_str().parse().unwrap();
            Some(("post".to_string(), number))
        } else {
            None
        }
    }
}

fn parse_local_version(local: Option<Match>) -> Option<Vec<LocalTypePart>> {
    if let Some(m) = local {
        let local = m.as_str();
        let ret: Vec<_> = LOCAL_VERSION_SEPARATOR.split(local).map(|part| {
            if let Ok(i) = part.parse::<u64>() {
                LocalTypePart::NumericVersion(i)
            } else {
                LocalTypePart::AlphanumVersion(part.to_string())
            }
        }).collect();
        Some(ret)
    } else {
        None
    }
}

impl Version {
    pub fn parse(input: &str) -> Option<Self> {
        let captures = VERSION_PATTERN.captures(input)?;
        let epoch: u64 = captures.name("epoch").map(|m| m.as_str().parse().unwrap()).unwrap_or(0);
        let release: Vec<_> = captures.name("release").map(|m| m.as_str().split(".").map(|s| s.parse().unwrap()).collect()).unwrap_or(vec![]);
        let pre = parse_letter_version(captures.name("pre_l"), captures.name("pre_n"));
        let post = parse_letter_version(captures.name("post_l"), captures.name("post_n1").or(captures.name("post_n2")));
        let dev = parse_letter_version(captures.name("dev_l"), captures.name("dev_n"));
        let local = parse_local_version(captures.name("local"));

        Some(Self {
            epoch,
            release,
            pre,
            post,
            dev,
            local
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_local_version() {
        let regex = Regex::new(r"(?P<local>[a-z0-9]+(?:[-_\.][a-z0-9]+)*)").unwrap();
        let m = regex.find("abc.1.twelve");
        assert_eq!(
            parse_local_version(m),
            Some(vec![
                LocalTypePart::AlphanumVersion("abc".to_string()),
                LocalTypePart::NumericVersion(1),
                LocalTypePart::AlphanumVersion("twelve".to_string())
            ])
        );
    }

}