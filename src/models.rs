use crate::diagnostic::Diagnostic;
use crate::{FileName, Label, LabelStyle, Location, ReportingFiles, ReportingSpan, Severity};

crate struct Message<'doc> {
    message: &'doc Option<String>,
}

impl<'doc> Message<'doc> {
    crate fn new(message: &'doc Option<String>) -> Message<'doc> {
        Message { message }
    }

    crate fn message(&self) -> &Option<String> {
        &self.message
    }
}

#[derive(Copy, Clone, Debug)]
crate struct Header<'doc> {
    severity: Severity,
    code: Option<&'doc str>,
    message: &'doc str,
}

impl<'doc> Header<'doc> {
    crate fn new(diagnostic: &'doc Diagnostic<impl ReportingSpan>) -> Header<'doc> {
        Header {
            severity: diagnostic.severity,
            code: diagnostic.code.as_ref().map(|c| &c[..]),
            message: &diagnostic.message,
        }
    }

    crate fn severity(&self) -> &'static str {
        match self.severity {
            Severity::Bug => "bug",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Help => "help",
            Severity::Note => "note",
        }
    }

    crate fn code(&self) -> &Option<&'doc str> {
        &self.code
    }

    crate fn message(&self) -> String {
        self.message.to_string()
    }
}

crate fn severity(diagnostic: &Diagnostic<impl ReportingSpan>) -> &'static str {
    match diagnostic.severity {
        Severity::Bug => "bug",
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Help => "help",
        Severity::Note => "note",
    }
}

#[derive(Copy, Clone, Debug)]
crate struct SourceLine<'doc, Span: ReportingSpan> {
    files: &'doc ReportingFiles<InnerSpan = Span>,
    label: &'doc Label<Span>,
    config: &'doc dyn crate::Config,
}

impl<Span: ReportingSpan> SourceLine<'doc, Span> {
    crate fn new(
        files: &'doc ReportingFiles<InnerSpan = Span>,
        label: &'doc Label<Span>,
        config: &'doc dyn crate::Config,
    ) -> SourceLine<'doc, Span> {
        SourceLine {
            files,
            label,
            config,
        }
    }

    crate fn location(&self) -> Location {
        self.files
            .location(self.label.span.start())
            .expect("location")
    }

    crate fn filename(&self) -> String {
        match &self.label.span.file_name() {
            FileName::Virtual(name) => format!("<{}>", name.to_str().unwrap()),
            FileName::Real(name) => self.config.filename(name),
            FileName::Verbatim(name) => format!("{}", name),
        }
    }

    crate fn line_span(&self) -> Span {
        self.files
            .line_span(self.location().line)
            .expect("line_span")
    }

    crate fn line_number(&self) -> usize {
        self.location().line + 1
    }

    crate fn line_number_len(&self) -> usize {
        self.line_number().to_string().len()
    }

    // crate fn before_line_len(&self) -> usize {
    //     // TODO: Improve
    //     self.before_marked().len() + self.line_number().to_string().len()
    // }

    crate fn before_marked(&self) -> &'doc str {
        self.files
            .source(&self.line_span().with_end(self.label.span.start()))
            .expect("line_prefix")
    }

    crate fn after_marked(&self) -> &'doc str {
        self.files
            .source(&self.line_span().with_start(self.label.span.end()))
            .expect("line_suffix")
            .trim_right_matches(|ch| ch == '\r' || ch == '\n')
    }

    crate fn marked(&self) -> &'doc str {
        self.files.source(&self.label.span).expect("line_marked")
    }
}

#[derive(Copy, Clone)]
pub struct LabelledLine<'doc, Span: ReportingSpan> {
    source_line: SourceLine<'doc, Span>,
    label: &'doc Label<Span>,
}

impl<Span: ReportingSpan> LabelledLine<'doc, Span> {
    crate fn new(
        source_line: SourceLine<'doc, Span>,
        label: &'doc Label<Span>,
    ) -> LabelledLine<'doc, Span> {
        LabelledLine { source_line, label }
    }

    crate fn mark(&self) -> &'static str {
        match self.label.style {
            LabelStyle::Primary => "^",
            LabelStyle::Secondary => "-",
        }
    }

    crate fn style(&self) -> &'static str {
        match self.label.style {
            LabelStyle::Primary => "primary",
            LabelStyle::Secondary => "secondary",
        }
    }

    crate fn message(&self) -> &Option<String> {
        self.label.message()
    }

    crate fn source_line(&self) -> &SourceLine<'doc, Span> {
        &self.source_line
    }
}
