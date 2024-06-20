use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_server::{TraceService, TraceServiceServer},
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::Constraint,
    widgets::{Row, Table},
    Frame, Terminal,
};
use tonic::{Request, Response, Status};

use crate::{buffer::RingBuffer, cli::Cli};

mod buffer;
mod cli;

pub(crate) mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug)]
pub(crate) struct TuiState {
    view_state: ViewState,
    buffer: Arc<RwLock<RingBuffer<ExportTraceServiceRequest>>>,
}

impl TuiState {
    pub(crate) fn new(buffer: Arc<RwLock<RingBuffer<ExportTraceServiceRequest>>>) -> Self {
        Self {
            view_state: ViewState::default(),
            buffer,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) enum ViewState {
    #[default]
    List,
    Detail,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let buffer = RwLock::new(RingBuffer::<ExportTraceServiceRequest>::new(100));
    let buffer = Arc::new(buffer);

    let service = InMemoryTraceService::new(buffer.clone());
    let service = TraceServiceServer::new(service);

    let otel_collector = tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(service)
            .serve(cli.address),
    );

    let mut stdout = std::io::stdout();

    enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
    let mut tui_state = TuiState::new(buffer);

    let _ = run_tui(&mut terminal, &mut tui_state);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();

    otel_collector.abort()
}

fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    tui_state: &mut TuiState,
) -> std::io::Result<bool> {
    loop {
        terminal.draw(|frame| draw_frame(frame, tui_state)).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Esc => match tui_state.view_state {
                    ViewState::List => todo!(),
                    ViewState::Detail => tui_state.view_state = ViewState::List,
                },
                _ => {}
            }
        }
    }
}

fn draw_frame(frame: &mut Frame, tui_state: &TuiState) {
    // TODO (@Techassi): Retrieve all traces
    let widget = match tui_state.view_state {
        ViewState::List => {
            Table::new([Row::new([""])], [Constraint::Percentage(25); 4]).header(Row::new([
                "Trace ID",
                "Trace Name",
                "Start Time",
                "Duration",
            ]))
        }
        ViewState::Detail => todo!(),
    };

    frame.render_widget(widget, frame.size())
}

struct InMemoryTraceService {
    buffer: Arc<RwLock<RingBuffer<ExportTraceServiceRequest>>>,
}

#[async_trait]
impl TraceService for InMemoryTraceService {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        dbg!(&request);

        let message = request.into_inner();
        self.buffer.write().unwrap().push(message);

        Ok(Response::new(ExportTraceServiceResponse {
            partial_success: None,
        }))
    }
}

impl InMemoryTraceService {
    pub fn new(buffer: Arc<RwLock<RingBuffer<ExportTraceServiceRequest>>>) -> Self {
        Self { buffer }
    }
}
