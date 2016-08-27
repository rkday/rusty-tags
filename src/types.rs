use std::fmt::{Debug, Display, Formatter, Error};
use std::path::PathBuf;
use app_result::{AppResult, app_err_msg};

/// For every `TagsRoot` a `rusty-tags.{vi,emacs}` file will be created.
///
/// `Proj` is the tags root of the current cargo project. Its tags file will contain the tags of
/// the source code of the cargo project and of its direct dependencies. The tags file will be
/// placed at the root of the cargo project, beside of the `Cargo.toml`.
///
/// `Lib` represents a direct or indirect (a dependency of a dependency) dependency of the cargo
/// project. For each dependency a tags file will be created containing the tags of the source
/// code of the dependency and its direct dependecies. The tags file will be placed at the root of
/// the source code of the dependency.
pub enum TagsRoot {
    /// the root directory of the cargo project
    /// and the dependencies of the cargo project
    Proj {
        root_dir: PathBuf,
        dependencies: Vec<SourceKind>
    },

    /// a library and its dependencies
    Lib {
        src_kind: SourceKind,
        dependencies: Vec<SourceKind>
    }
}

pub type TagsRoots = Vec<TagsRoot>;

impl Debug for TagsRoot {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            TagsRoot::Proj { ref root_dir, ref dependencies } => {
                write!(f, "TagsRoot::Proj ( root_dir: {}, dependencies: {:?} )", root_dir.display(), dependencies)
            },

            TagsRoot::Lib { ref src_kind, ref dependencies } => {
                write!(f, "TagsRoot::Lib ( src_kind: {}, dependencies: {:?} )", src_kind, dependencies)
            }
        }
    }
}

/// Where the source code of a dependency is from. From a git repository, from `crates.io` or from
/// a local path.
#[derive(Clone)]
pub enum SourceKind {
    /// the source is from a git repository
    Git {
        lib_name: String,
        commit_hash: String
    },

    /// the source is from crates.io
    CratesIo {
        lib_name: String,
        version: String
    },

    /// the source is from a local directory
    Path {
        lib_name: String,
        path: PathBuf
    }
}

impl SourceKind {
    pub fn tags_file_name(&self, tags_spec: &TagsSpec) -> String {
        match *self {
            SourceKind::Git { ref lib_name, ref commit_hash } => {
                format!("{}-{}.{}", lib_name, commit_hash, tags_spec.file_extension())
            },

            SourceKind::CratesIo { ref lib_name, ref version } => {
                format!("{}-{}.{}", lib_name, version, tags_spec.file_extension())
            },

            SourceKind::Path { .. } => {
                tags_spec.file_name().to_owned()
            }
        }
    }

    pub fn get_lib_name(&self) -> String {
        match *self {
            SourceKind::Git { ref lib_name, .. } => {
                lib_name.clone()
            },

            SourceKind::CratesIo { ref lib_name, .. } => {
                lib_name.clone()
            },

            SourceKind::Path { ref lib_name, .. } => {
                lib_name.clone()
            }
        }
    }

    fn display(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            SourceKind::Git { ref lib_name, ref commit_hash } => {
                write!(f, "{}-{}", lib_name, commit_hash)
            },

            SourceKind::CratesIo { ref lib_name, ref version } => {
                write!(f, "{}-{}", lib_name, version)
            },

            SourceKind::Path { ref lib_name, ref path } => {
                write!(f, "{}: {}", lib_name, path.display())
            }
        }
    }
}

impl Debug for SourceKind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.display(f)
    }
}

impl Display for SourceKind {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.display(f)
    }
}

pub struct Tags {
    /// the root directory of the source code
    /// for which the tags have been created
    pub src_dir: PathBuf,

    /// the tags file of the sources in `src_dir`
    pub tags_file: PathBuf,

    /// indicates if the tags file is already existing
    /// and the cached tags file is returned
    cached: bool
}

impl Tags {
    pub fn new(src_dir: &PathBuf, tags_file: &PathBuf, cached: bool) -> Tags {
        Tags { src_dir: src_dir.clone(), tags_file: tags_file.clone(), cached: cached }
    }

    pub fn is_up_to_date(&self, tags_spec: &TagsSpec) -> bool {
        if ! self.cached {
            return false;
        }

        let mut src_tags = self.src_dir.clone();
        src_tags.push(tags_spec.file_name());

        src_tags.as_path().is_file()
    }
}

impl Debug for Tags {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Tags ( src_dir: {}, tags_file: {}, cached: {} )",
        self.src_dir.display(), self.tags_file.display(), self.cached)
    }
}

/// which kind of tags are created
arg_enum! {
    #[derive(Eq, PartialEq, Debug)]
    pub enum TagsKind {
        Vi,
        Emacs
    }
}

/// holds additional info for the kind of tags, which extension
/// they use for caching and which user viewable file names they get
pub struct TagsSpec {
    pub kind: TagsKind,

    /// the file name for vi tags
    vi_tags: String,

    /// the file name for emacs tags
    emacs_tags: String
}

impl TagsSpec {
    pub fn new(kind: TagsKind, vi_tags: String, emacs_tags: String) -> AppResult<TagsSpec> {
        if vi_tags == emacs_tags {
            return Err(app_err_msg(format!("It's not recommended to use the same tags name '{}' for vi and emacs!", vi_tags)));
        }

        Ok(TagsSpec {
            kind: kind,
            vi_tags: vi_tags,
            emacs_tags: emacs_tags
        })
    }

    pub fn file_extension(&self) -> &'static str {
        match self.kind {
            TagsKind::Vi    => "vi",
            TagsKind::Emacs => "emacs"
        }
    }

    pub fn file_name(&self) -> &str {
        match self.kind {
            TagsKind::Vi    => &self.vi_tags,
            TagsKind::Emacs => &self.emacs_tags
        }
    }

    pub fn ctags_option(&self) -> Option<&'static str> {
        match self.kind {
            TagsKind::Vi    => None,
            TagsKind::Emacs => Some("-e")
        }
    }
}
