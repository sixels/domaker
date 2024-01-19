use std::fmt;

use codespan::{ByteIndex, ByteOffset, Span};
use codespan_reporting::files::{Error, Files, SimpleFile};

type FilesResult<T> = Result<T, Error>;

#[derive(Debug, Default)]
pub struct SourceFiles {
    files: Vec<SourceFile>,
}

impl SourceFiles {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, name: String, content: String) {
        let offset = self
            .files
            .last()
            .map_or(ByteIndex(0), |f| ByteIndex(f.file.source().len() as u32));

        self.files.push(SourceFile::new(name, content, offset));
    }

    pub fn get(&self, offset: ByteIndex) -> Option<&SourceFile> {
        self.files
            .binary_search_by(|f| {
                let fspan = f.span();
                if fspan.start() > offset {
                    std::cmp::Ordering::Greater
                } else if fspan.end() <= offset {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .ok()
            .map(|i| &self.files[i])
    }
}

pub struct SourceFile {
    file: SimpleFile<String, String>,
    offset: ByteIndex,
}

impl SourceFile {
    pub fn new(name: String, content: String, offset: ByteIndex) -> Self {
        Self {
            file: SimpleFile::new(name, content),
            offset,
        }
    }

    pub fn span(&self) -> Span {
        Span::new(
            self.offset,
            self.offset + ByteOffset(self.file.source().len() as _),
        )
    }
}

impl<'a> Files<'a> for SourceFile {
    type FileId = ();
    type Name = String;
    type Source = &'a str;

    fn name(&'a self, _id: Self::FileId) -> FilesResult<Self::Name> {
        Ok(self.file.name().clone())
    }

    fn source(&'a self, _id: Self::FileId) -> FilesResult<Self::Source> {
        Ok(self.file.source())
    }

    fn line_index(&'a self, id: Self::FileId, byte_index: usize) -> FilesResult<usize> {
        self.file.line_index(id, byte_index)
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> FilesResult<std::ops::Range<usize>> {
        self.file.line_range(id, line_index)
    }
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceFile")
            .field("file", self.file.source())
            .finish()
    }
}
