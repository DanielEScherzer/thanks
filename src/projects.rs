use crate::Project;
use crate::git::VersionTag;
use git2::Repository;
use semver::Version;

pub struct Rust;

impl Project for Rust {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn url(&self) -> &'static str {
        "rust"
    }

    fn is_homepage(&self) -> bool {
        true
    }

    fn repo_url(&self) -> &'static str {
        "https://github.com/rust-lang/rust.git"
    }

    fn augment_versions(&self, repo: &Repository, versions: &mut Vec<VersionTag>) {
        let last_full_stable = versions
            .iter()
            .rfind(|v| v.raw_tag.ends_with(".0"))
            .unwrap()
            .version
            .clone();

        // The nightly branch is the default one, fall back to "main" if it cannot
        // be read
        let nightly_branch = match repo.head() {
            Ok(reference) => match reference.shorthand() {
                Some(name) => name.to_string(),
                None => "main".to_string(),
            },
            Err(_) => "main".to_string(),
        };

        versions.push(VersionTag {
            name: String::from("Beta"),
            version: {
                let mut last = last_full_stable.clone();
                last.minor += 1;
                last
            },
            raw_tag: String::from("beta"),
            commit: repo
                .revparse_single("beta")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id(),
            in_progress: true,
        });
        versions.push(VersionTag {
            name: String::from("Nightly"),
            version: {
                // main is plus 1 minor versions off of beta, which we just pushed
                let mut last = last_full_stable.clone();
                last.minor += 2;
                last
            },
            raw_tag: nightly_branch,
            commit: repo
                .revparse_single("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id(),
            in_progress: true,
        });
    }
}

pub struct Rustup;

impl Project for Rustup {
    fn name(&self) -> &'static str {
        "Rustup"
    }

    fn url(&self) -> &'static str {
        "rustup"
    }

    fn repo_url(&self) -> &'static str {
        "https://github.com/rust-lang/rustup.git"
    }

    fn augment_versions(&self, repo: &Repository, versions: &mut Vec<VersionTag>) {
        let last_full_stable = versions
            .iter()
            .rfind(|v| v.raw_tag.ends_with(".0"))
            .unwrap()
            .version
            .clone();

        versions.push(VersionTag {
            name: String::from("Nightly"),
            version: {
                let mut last = last_full_stable.clone();
                last.minor += 1;
                last
            },
            raw_tag: String::from("main"),
            commit: repo
                .revparse_single("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id(),
            in_progress: true,
        });
    }
}

pub struct PHP;

impl Project for PHP {
    fn name(&self) -> &'static str {
        "PHP"
    }

    fn url(&self) -> &'static str {
        "PHP"
    }

    fn is_homepage(&self) -> bool {
        false
    }

    fn repo_url(&self) -> &'static str {
        "https://github.com/php/php-src.git"
    }

    /// Identify the versions that have been tagged.
    fn get_versions(&self, repo: &Repository) -> Result<Vec<VersionTag>, Box<dyn std::error::Error>> {
        let tags = repo
            .tag_names(None)?
            .into_iter()
            .flatten()
            .filter(|v| v.starts_with("php-"))
            .map(|v| v.to_owned())
            .collect::<Vec<_>>();
        let mut versions = tags
            .iter()
            .filter_map(|tag| {
                Version::parse(tag)
                    .or_else(|_| Version::parse(tag.strip_prefix("php-").expect("filtered")))
                    .or_else(|_| Version::parse(&format!("{}.0", tag)))
                    .ok()
                    .map(|v| VersionTag {
                        name: format!("PHP {}", v),
                        version: v,
                        raw_tag: tag.clone(),
                        commit: repo
                            .revparse_single(tag)
                            .unwrap()
                            .peel_to_commit()
                            .unwrap()
                            .id(),
                        in_progress: false,
                    })
            })
            .collect::<Vec<_>>();
        versions.sort();

        let last_full_stable = versions
            .iter()
            .rfind(|v| v.raw_tag.ends_with(".0"))
            .unwrap()
            .version
            .clone();

        versions.push(VersionTag {
            name: String::from("Nightly"),
            version: {
                let mut last = last_full_stable.clone();
                last.minor += 1;
                last
            },
            raw_tag: "master".to_string(),
            commit: repo
                .revparse_single("HEAD")
                .unwrap()
                .peel_to_commit()
                .unwrap()
                .id(),
            in_progress: true,
        });
        Ok(versions)
    }

}
