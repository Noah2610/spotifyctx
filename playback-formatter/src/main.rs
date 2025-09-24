use clap::Parser;
use cmus_status_line::{
    cmus_status::{
        data::CmusData,
        output::{Format, StatusOutput},
    },
    error::Error as CmusError,
};
use std::{
    error::Error,
    fmt,
    io::{self, stdin, Read},
};

#[derive(Parser)]
struct Args {
    /// If the player is currently playing or paused
    #[clap(short, long)]
    pub status: Status,

    /// Current progress given in --unit
    #[clap(short, long)]
    pub progress: u32,

    /// Total duration given in --unit
    #[clap(short, long)]
    pub duration: u32,

    /// Optional title of the current track
    #[clap(short, long)]
    pub title: Option<String>,

    /// Optional artist of the current track
    #[clap(short, long)]
    pub artist: Option<String>,

    /// Optional album of the current track
    #[clap(short = 'A', long)]
    pub album: Option<String>,

    /// Time unit for --progress and --duration
    #[clap(short, long, default_value_t = Unit::default())]
    pub unit: Unit,

    /// Output format.
    /// Use special value "-" to read from stdin
    #[clap(short, long, value_parser = parse_arg_format)]
    pub format: ArgFormat,
}

fn parse_arg_format(
    s: &str,
) -> Result<ArgFormat, Box<dyn std::error::Error + Send + Sync + 'static>> {
    ArgFormat::try_from(s).map_err(|e| e.0)
}

#[derive(Clone)]
struct ArgFormat(pub Format);

impl TryFrom<&str> for ArgFormat {
    type Error = MyError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "-" => {
                let mut input = String::new();
                stdin().read_to_string(&mut input)?;
                Format::try_from(format!("%{{{}}}", input))
            }
            s => Format::try_from(format!("%{{{}}}", s)),
        }
        .map(|f| ArgFormat(f))
        .map_err(Into::into)
    }
}

#[derive(Clone, Default, clap::ValueEnum)]
enum Unit {
    /// Milliseconds
    #[default]
    Ms,
    /// Seconds
    S,
}

impl Unit {
    pub fn to_s(&self, n: u32) -> u32 {
        match self {
            Self::Ms => n / 1000,
            Self::S => n,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Ms => write!(f, "ms"),
            Unit::S => write!(f, "s"),
        }
    }
}

#[derive(Clone, clap::ValueEnum)]
enum Status {
    /// Playback is playing
    Playing,
    /// Playback is paused
    Paused,
    /// Playback is Stopped
    Stopped,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Playing => write!(f, "playing"),
            Status::Paused => write!(f, "paused"),
            Status::Stopped => write!(f, "stopped"),
        }
    }
}

#[derive(Debug)]
struct MyError(pub Box<dyn Error + Send + Sync + 'static>);

impl From<CmusError> for MyError {
    fn from(err: CmusError) -> Self {
        MyError(Box::new(err))
    }
}

impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError(Box::new(err))
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Error] {}", self.0)
    }
}

impl Error for MyError {}

fn main() -> Result<(), MyError> {
    let args = Args::parse();

    let cmus_data: CmusData = format!(
        r#"
            {title_opt}
            status {status}
            position {progress}
            duration {duration}
            {artist_opt}
            {album_opt}
        "#,
        status = args.status,
        progress = args.unit.to_s(args.progress),
        duration = args.unit.to_s(args.duration),
        title_opt = args
            .title
            .filter(|t| t.len() > 0)
            .map(|t| format!("file {}", t))
            .unwrap_or_default(),
        artist_opt = args
            .artist
            .filter(|t| t.len() > 0)
            .map(|a| format!("tag artist {}", a))
            .unwrap_or_default(),
        album_opt = args
            .album
            .filter(|t| t.len() > 0)
            .map(|a| format!("tag album {}", a))
            .unwrap_or_default(),
    )
    .try_into()?;

    let output = StatusOutput::builder()
        .data(cmus_data)
        .format(args.format.0)
        .build()?;

    println!("{}", output);

    Ok(())
}
