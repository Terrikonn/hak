#![no_std]

extern crate alloc;

use alloc::{
    fmt,
    format,
    string::String,
    sync::Arc,
    vec::Vec,
};
use core::sync::atomic::{
    AtomicUsize,
    Ordering,
};

use hashbrown::HashMap;
use lazy_static::lazy_static;
use owo_colors::{
    colors::xterm::*,
    OwoColorize,
};
use spin::{
    Mutex,
    MutexGuard,
};
use tracing::{
    field::{
        Field,
        Visit,
    },
    Id,
    Level,
    Subscriber,
};

#[derive(Clone)]
pub struct CurrentSpan {
    current: &'static Arc<Mutex<Vec<Id>>>,
}

impl CurrentSpan {
    pub fn new() -> Self {
        lazy_static! {
            static ref CURRENT: Arc<Mutex<Vec<Id>>> = Arc::new(Mutex::new(Vec::new()));
        }
        Self {
            current: &CURRENT,
        }
    }

    pub fn id(&self) -> Option<Id> {
        (*self.current.lock()).last().cloned()
    }

    pub fn enter(&self, span: Id) {
        self.current.lock().push(span)
    }

    pub fn exit(&self) {
        self.current.lock().pop();
    }
}

pub struct KernelSubscriber<O: core::fmt::Write> {
    current: CurrentSpan,
    indent_amount: usize,
    output: Mutex<O>,
    stack: Mutex<Vec<Id>>,
    spans: Mutex<HashMap<Id, Span>>,
    ids: AtomicUsize,
}

struct Span {
    parent: Option<Id>,
    kvs: Vec<(&'static str, String)>,
}

struct Event<'a, O: fmt::Write + 'a> {
    output: MutexGuard<'a, O>,
    comma: bool,
}

struct ColorLevel<'a>(&'a Level);

impl<'a> fmt::Display for ColorLevel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self.0 {
            Level::TRACE => format!("{}", "TRACE".fg::<Purple>()),
            Level::DEBUG => format!("{}", "DEBUG".fg::<Blue>()),
            Level::INFO => format!("{}", "INFO ".fg::<Green>()),
            Level::WARN => format!("{}", "WARN ".fg::<Yellow>()),
            Level::ERROR => format!("{}", "ERROR".fg::<Red>()),
        })
    }
}

impl Span {
    fn new(parent: Option<Id>, attrs: &tracing::span::Attributes<'_>) -> Self {
        let mut span = Self {
            parent,
            kvs: Vec::new(),
        };
        attrs.record(&mut span);
        span
    }
}

impl Visit for Span {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.kvs.push((field.name(), format!("{:?}", value)))
    }
}

impl<'a, O: fmt::Write> Visit for Event<'a, O> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        write!(
            &mut self.output,
            "{comma} ",
            comma = if self.comma {
                ","
            } else {
                ""
            },
        )
        .unwrap();
        let name = field.name();
        if name == "message" {
            write!(&mut self.output, "{}", format!("{:?}", value).bold()).unwrap();
            self.comma = true;
        } else {
            write!(&mut self.output, "{}: {:?}", name.bold(), value).unwrap();
            self.comma = true;
        }
    }
}

impl<O: fmt::Write> KernelSubscriber<O> {
    // uart::uart::Uart::new(0x1000_0000)
    pub fn new(writer: O, indent_amount: usize) -> Self {
        Self {
            current: CurrentSpan::new(),
            indent_amount,
            output: Mutex::new(writer),
            stack: Mutex::new(Vec::new()),
            spans: Mutex::new(HashMap::new()),
            ids: AtomicUsize::new(1),
        }
    }

    fn print_kvs<'a, I, K, V>(&self, writer: &mut O, kvs: I, leading: &str) -> fmt::Result
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str> + 'a,
        V: fmt::Display + 'a,
    {
        let mut kvs = kvs.into_iter();
        if let Some((k, v)) = kvs.next() {
            write!(writer, "{}{}: {}", leading, k.as_ref().bold(), v)?;
        }
        for (k, v) in kvs {
            write!(writer, ", {}: {}", k.as_ref().bold(), v)?;
        }

        Ok(())
    }

    fn print_indent(&self, writer: &mut O, indent: usize) -> fmt::Result {
        for _ in 0..(indent * self.indent_amount) {
            write!(writer, " ")?;
        }
        Ok(())
    }
}

impl<O: fmt::Write + 'static> Subscriber for KernelSubscriber<O> {
    fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, span: &tracing::span::Attributes<'_>) -> tracing::Id {
        let next = self.ids.fetch_add(1, Ordering::SeqCst) as u64;
        let id = tracing::Id::from_u64(next);
        let span = Span::new(self.current.id(), span);
        self.spans.lock().insert(id.clone(), span);
        id
    }

    fn record(&self, span: &tracing::Id, values: &tracing::span::Record<'_>) {
        let mut spans = self.spans.lock();
        if let Some(span) = spans.get_mut(span) {
            values.record(span);
        }
    }

    fn record_follows_from(&self, _span: &tracing::Id, _follows: &tracing::Id) {
        unimplemented!()
    }

    fn enter(&self, span_id: &tracing::Id) {
        self.current.enter(span_id.clone());
        let mut output = self.output.lock();
        let mut stack = self.stack.lock();
        let spans = self.spans.lock();
        let data = spans.get(span_id);
        let parent = data.and_then(|span| span.parent.as_ref());
        if !stack.iter().any(|id| id == span_id) {
            let indent = if let Some(idx) = stack.iter().position(|id| parent.map(|p| id == p).unwrap_or(false)) {
                let idx = idx + 1;
                stack.truncate(idx);
                idx
            } else {
                stack.clear();
                0
            };
            self.print_indent(&mut output, indent).unwrap();
            stack.push(span_id.clone());
            if let Some(data) = data {
                self.print_kvs(&mut output, data.kvs.iter().map(|(k, v)| (k, v)), "").unwrap()
            }
            writeln!(&mut output).unwrap();
        }
    }

    fn event(&self, event: &tracing::Event<'_>) {
        let mut output = self.output.lock();
        let indent = self.stack.lock().len();
        self.print_indent(&mut output, indent).unwrap();
        write!(
            &mut output,
            "{level} {target}",
            level = ColorLevel(event.metadata().level()),
            target = &event.metadata().target(),
        )
        .unwrap();
        let mut visitor = Event {
            output,
            comma: false,
        };
        event.record(&mut visitor);
        writeln!(&mut visitor.output).unwrap();
    }

    #[inline]
    fn exit(&self, _span: &tracing::Id) {
        // TODO; unify stack with current span.
        self.current.exit();
    }

    fn try_close(&self, _id: tracing::Id) -> bool {
        // TODO: GC unneeded spans.
        false
    }
}
