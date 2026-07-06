use napi::bindgen_prelude::*;
use napi_derive::napi;

use float_pigment_css::parser::Warning;
use float_pigment_css::{StyleSheetImportIndex, StyleSheetResource};

// ---------- types ----------

#[napi(string_enum)]
pub enum OutputType {
    #[napi(value = "bincode")]
    Bincode,
    #[napi(value = "json")]
    Json,
    #[napi(value = "none")]
    None,
}

#[napi(object)]
pub struct SourceCodePosition {
    pub line: u32,
    pub column: u32,
}

#[napi(object)]
pub struct CompileWarning {
    pub start: SourceCodePosition,
    pub end: SourceCodePosition,
    pub message: String,
}

#[napi(object)]
pub struct FileResult {
    pub content: Option<Buffer>,
    pub warnings: Vec<CompileWarning>,
}

#[napi(object)]
pub struct CompileResult {
    pub files: Vec<FileEntry>,
    pub import_index: Option<Buffer>,
}

#[napi(object)]
pub struct FileEntry {
    pub path: String,
    pub file: FileResult,
}

#[napi(object)]
pub struct CompileArgument {
    /// Source files to compile
    pub src: Vec<SourceEntry>,
    /// Output encoding format
    pub output_type: OutputType,
    /// Prefix for tag name selectors, default "wx-"
    pub tag_name_prefix: Option<String>,
}

#[napi(object)]
pub struct SourceEntry {
    pub path: String,
    pub content: Buffer,
}

#[napi(object)]
pub struct CompileSingleArgument {
    /// CSS file name
    pub file_name: String,
    /// CSS file content
    pub file_content: Buffer,
    /// Output encoding format
    pub output_type: OutputType,
    /// Prefix for tag name selectors, default "wx-"
    pub tag_name_prefix: Option<String>,
}

// ---------- helpers ----------

fn convert_warnings(warnings: &[Warning]) -> Vec<CompileWarning> {
    warnings
        .iter()
        .map(|w| CompileWarning {
            start: SourceCodePosition {
                line: w.start_line,
                column: w.start_col,
            },
            end: SourceCodePosition {
                line: w.end_line,
                column: w.end_col,
            },
            message: w.message.to_string(),
        })
        .collect()
}

fn serialize_sheet(
    resource: &StyleSheetResource,
    path: &str,
    output_type: &OutputType,
) -> Option<Buffer> {
    // Serializes the *unlinked* single style sheet for `path` (i.e. `@import`
    // is not expanded here; linking happens on the consumer side). `None` is
    // returned when `output_type` is `None`, or when `path` is not present in
    // the resource — for the compile flows below the path is always the one we
    // just added, so `None` here effectively means "no output requested".
    match output_type {
        OutputType::Bincode => resource.serialize_bincode(path).map(Buffer::from),
        OutputType::Json => resource
            .serialize_json(path)
            .map(|s| Buffer::from(s.into_bytes())),
        OutputType::None => None,
    }
}

fn serialize_import_index(index: &StyleSheetImportIndex, output_type: &OutputType) -> Option<Buffer> {
    match output_type {
        OutputType::Bincode => Some(Buffer::from(index.serialize_bincode())),
        OutputType::Json => Some(Buffer::from(index.serialize_json().into_bytes())),
        OutputType::None => None,
    }
}

// ---------- core compile logic ----------

fn do_compile(args: CompileArgument) -> Result<CompileResult> {
    let tag_prefix = args.tag_name_prefix.as_deref().unwrap_or("wx-").to_owned();
    let output_type = args.output_type;
    let mut resource = StyleSheetResource::new();
    let mut file_entries = Vec::with_capacity(args.src.len());

    for entry in args.src {
        let source = std::str::from_utf8(entry.content.as_ref()).map_err(|e| {
            Error::new(
                Status::InvalidArg,
                format!("Invalid UTF-8 in source for '{}': {e}", entry.path),
            )
        })?;

        let warnings = resource.add_source(&entry.path, source);
        resource.add_tag_name_prefix(&entry.path, &tag_prefix);

        // Serialize each sheet as an independent, unlinked unit. `import_index`
        // (built after the loop, once all sources are present) carries the
        // cross-file `@import` graph; linking is performed by the consumer.
        let content = serialize_sheet(&resource, &entry.path, &output_type);

        file_entries.push(FileEntry {
            path: entry.path,
            file: FileResult {
                content,
                warnings: convert_warnings(&warnings),
            },
        });
    }

    let import_index = if !matches!(output_type, OutputType::None) {
        let idx = resource.generate_import_indexes();
        serialize_import_index(&idx, &output_type)
    } else {
        None
    };

    Ok(CompileResult {
        files: file_entries,
        import_index,
    })
}

fn do_compile_single(args: CompileSingleArgument) -> Result<FileResult> {
    let tag_prefix = args.tag_name_prefix.as_deref().unwrap_or("wx-");
    let mut resource = StyleSheetResource::new();

    let source = std::str::from_utf8(args.file_content.as_ref()).map_err(|e| {
        Error::new(
            Status::InvalidArg,
            format!("Invalid UTF-8 in fileContent: {e}"),
        )
    })?;

    let warnings = resource.add_source(&args.file_name, source);
    resource.add_tag_name_prefix(&args.file_name, tag_prefix);

    let content = serialize_sheet(&resource, &args.file_name, &args.output_type);

    Ok(FileResult {
        content,
        warnings: convert_warnings(&warnings),
    })
}

// ---------- sync exports ----------

#[napi]
pub fn compile_sync(args: CompileArgument) -> Result<CompileResult> {
    do_compile(args)
}

#[napi]
pub fn compile_single_sync(args: CompileSingleArgument) -> Result<FileResult> {
    do_compile_single(args)
}

// ---------- async exports ----------

pub struct CompileTask {
    args: Option<CompileArgument>,
}

// `#[napi]` on the `impl Task` block registers the CompileTask -> CompileResult
// type mapping so the generated d.ts is `Promise<CompileResult>` rather than
// `Promise<unknown>`. It does NOT export CompileTask as a JS symbol (that would
// require `#[napi]` on the struct itself).
#[napi]
impl Task for CompileTask {
    type Output = CompileResult;
    type JsValue = CompileResult;

    fn compute(&mut self) -> Result<Self::Output> {
        let args = self.args.take().ok_or_else(|| {
            Error::new(Status::GenericFailure, "compile task args already consumed")
        })?;
        do_compile(args)
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi]
pub fn compile(args: CompileArgument) -> AsyncTask<CompileTask> {
    AsyncTask::new(CompileTask { args: Some(args) })
}

pub struct CompileSingleTask {
    args: Option<CompileSingleArgument>,
}

#[napi]
impl Task for CompileSingleTask {
    type Output = FileResult;
    type JsValue = FileResult;

    fn compute(&mut self) -> Result<Self::Output> {
        let args = self.args.take().ok_or_else(|| {
            Error::new(
                Status::GenericFailure,
                "compile_single task args already consumed",
            )
        })?;
        do_compile_single(args)
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi]
pub fn compile_single(args: CompileSingleArgument) -> AsyncTask<CompileSingleTask> {
    AsyncTask::new(CompileSingleTask { args: Some(args) })
}
