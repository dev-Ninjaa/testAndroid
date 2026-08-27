use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

/// Discord application ID - replace with your own from https://discord.com/developers/applications
const APP_ID: &str = "1540597943151763486";

const IDLE_TICK: Duration = Duration::from_secs(15);
const CONNECT_RETRY_MIN: Duration = Duration::from_secs(1);
const CONNECT_RETRY_MAX: Duration = Duration::from_secs(15);
const SEND_FLOOR: Duration = Duration::from_millis(1500);

enum Msg {
    Track { title: String, artist: String, video_id: String },
    Duration(f64),
    Position { pos: f64, at: Instant },
    Playing(bool),
}

pub struct DiscordHandle {
    tx: Sender<Msg>,
}

impl DiscordHandle {
    pub fn set_track(&self, title: &str, artist: &str, video_id: &str) {
        let _ = self.tx.send(Msg::Track {
            title: title.to_string(),
            artist: artist.to_string(),
            video_id: video_id.to_string(),
        });
    }

    pub fn set_duration(&self, secs: f64) {
        let _ = self.tx.send(Msg::Duration(secs));
    }

    pub fn set_position(&self, pos: f64) {
        let _ = self.tx.send(Msg::Position { pos, at: Instant::now() });
    }

    pub fn set_playing(&self, playing: bool) {
        let _ = self.tx.send(Msg::Playing(playing));
    }
}

struct Presence {
    client: Option<DiscordIpcClient>,
    track_title: Option<String>,
    track_artist: Option<String>,
    video_id: Option<String>,
    duration: f64,
    playing: bool,
    pos: f64,
    pos_at: Instant,
    last_send: Option<Instant>,
    connect_backoff: Duration,
    last_connect_try: Option<Instant>,
}

impl Presence {
    fn new() -> Self {
        Presence {
            client: None,
            track_title: None,
            track_artist: None,
            video_id: None,
            duration: 0.0,
            playing: false,
            pos: 0.0,
            pos_at: Instant::now(),
            last_send: None,
            connect_backoff: CONNECT_RETRY_MIN,
            last_connect_try: None,
        }
    }

    fn apply(&mut self, msg: Msg) {
        match msg {
            Msg::Track { title, artist, video_id } => {
                self.track_title = Some(title);
                self.track_artist = Some(artist);
                self.video_id = Some(video_id);
                self.pos = 0.0;
                self.pos_at = Instant::now();
            }
            Msg::Duration(secs) => self.duration = secs,
            Msg::Position { pos, at } => {
                self.pos = pos;
                self.pos_at = at;
            }
            Msg::Playing(on) => {
                if self.playing != on {
                    self.playing = on;
                }
            }
        }
    }

    fn sync(&mut self) {
        self.ensure_connected();
        
        if self.playing && self.track_title.is_some() {
            if self.can_send() {
                self.push_card();
            }
        } else if !self.playing {
            self.clear_card();
        }
    }

    fn can_send(&self) -> bool {
        match self.last_send {
            Some(instant) => instant.elapsed() >= SEND_FLOOR,
            None => true,
        }
    }

    fn ensure_connected(&mut self) -> bool {
        if self.client.is_some() {
            return true;
        }

        if let Some(last_try) = self.last_connect_try {
            if last_try.elapsed() < self.connect_backoff {
                return false;
            }
        }

        self.last_connect_try = Some(Instant::now());
        let mut client = DiscordIpcClient::new(APP_ID);
        
        match client.connect() {
            Ok(()) => {
                println!("Discord Rich Presence connected");
                self.client = Some(client);
                self.connect_backoff = CONNECT_RETRY_MIN;
                true
            }
            Err(e) => {
                println!("Discord connection failed: {}", e);
                self.connect_backoff = (self.connect_backoff * 2).min(CONNECT_RETRY_MAX);
                false
            }
        }
    }

    fn push_card(&mut self) {
        let Some(mut client) = self.client.take() else {
            return;
        };

        let title = self.track_title.as_ref().unwrap();
        let artist = self.track_artist.as_ref().unwrap();
        
        let start_ms = now_ms() - (self.pos * 1000.0) as i64;
        let end_ms = if self.duration > 0.0 {
            Some(start_ms + (self.duration * 1000.0) as i64)
        } else {
            None
        };

        let mut ts = activity::Timestamps::new().start(start_ms);
        if let Some(end) = end_ms {
            ts = ts.end(end);
        }

        let mut act = activity::Activity::new()
            .activity_type(activity::ActivityType::Listening)
            .details(title.as_str())
            .state(artist.as_str())
            .timestamps(ts);

        let buttons = vec![
            activity::Button::new("Get Maple Music", "https://github.com/dev-Ninjaa/maple.music")
        ];
        act = act.buttons(buttons);

        self.last_send = Some(Instant::now());
        
        if client.set_activity(act).is_ok() {
            self.client = Some(client);
        }
    }

    fn clear_card(&mut self) {
        if let Some(mut client) = self.client.take() {
            self.last_send = Some(Instant::now());
            let _ = client.clear_activity();
            self.client = Some(client);
        }
    }
}

pub fn spawn(enabled: bool) -> Option<DiscordHandle> {
    if !enabled || APP_ID.is_empty() {
        return None;
    }

    let (tx, rx) = channel::<Msg>();
    
    match std::thread::Builder::new()
        .name("discord-rpc".into())
        .spawn(move || run(rx))
    {
        Ok(_) => Some(DiscordHandle { tx }),
        Err(e) => {
            eprintln!("Discord RPC thread spawn failed: {}", e);
            None
        }
    }
}

fn run(rx: Receiver<Msg>) {
    let mut presence = Presence::new();
    
    loop {
        match rx.recv_timeout(IDLE_TICK) {
            Ok(msg) => {
                presence.apply(msg);
                loop {
                    match rx.try_recv() {
                        Ok(msg) => presence.apply(msg),
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => return,
                    }
                }
                presence.sync();
            }
            Err(_) => {
                presence.sync();
            }
        }
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
