use regex::{CaptureMatches, Regex};
use std::fmt::{self, Display, Formatter, Write};

#[derive(Debug, Clone)]
pub enum Fade {
    FadeIn,
    FadeOut,
    Modulate,
    StayOut,
    Purge,
}

impl Display for Fade {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<Fade> for String {
    fn from(fade: Fade) -> Self {
        match fade {
            Fade::FadeIn => "FFADE_IN \"1\"".to_string(),
            Fade::FadeOut => "FFADE_OUT \"1\"".to_string(),
            Fade::Modulate => "FFADE_MODULATE \"1\"".to_string(),
            Fade::StayOut => "FFADE_STAYOUT \"1\"".to_string(),
            Fade::Purge => "FFADE_PURGE \"1\"".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextEffect {
    Flicker,
    FadeInOut,
    WriteOut,
}

impl Display for TextEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<TextEffect> for String {
    fn from(text_effect: TextEffect) -> Self {
        match text_effect {
            TextEffect::Flicker => "FLICKER \"1\"".to_string(),
            TextEffect::FadeInOut => "FADEINOUT \"1\"".to_string(),
            TextEffect::WriteOut => "WRITEOUT \"1\"".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Properties {
    pub name: String,
    pub start_tick: Option<i64>,
    pub start_time: Option<f64>,
    pub stop_tick: Option<i64>,
    pub stop_time: Option<f64>,
    pub skip_to_tick: Option<i64>,
    pub skip_to_time: Option<f64>,
    pub track: i64,
    pub spline: bool,
    pub stayout: bool,
    pub final_fov: f64,
    pub playback_rate: f64,
    pub zoom_in_time: f64,
    pub zoom_out_time: f64,
    pub hold_time: f64,
    pub duration: f64,
    pub fade_in: f64,
    pub fade_out: f64,
    pub fx_time: f64,
    pub sound: String,
    pub message: String,
    pub font: String,
    pub commands: String,
    pub effect: TextEffect,
    pub fade: Fade,
    pub xy: [f64; 2],
    pub rgba1: [u8; 4],
    pub rgba2: [u8; 4],
}

impl Properties {
    fn new() -> Self {
        Properties {
            name: "Unnamed".to_string(),
            start_tick: None,
            start_time: None,
            stop_tick: None,
            stop_time: None,
            skip_to_tick: None,
            skip_to_time: None,
            track: 0,
            spline: false,
            stayout: false,
            final_fov: 0.0,
            playback_rate: 0.0,
            zoom_in_time: 0.0,
            zoom_out_time: 0.0,
            hold_time: 0.0,
            duration: 0.0,
            sound: "".to_string(),
            message: "".to_string(),
            font: "".to_string(),
            fade_in: 0.0,
            fade_out: 0.0,
            fx_time: 0.0,
            effect: TextEffect::FadeInOut,
            fade: Fade::FadeIn,
            xy: [0.0, 0.0],
            rgba1: [0, 0, 0, 0],
            rgba2: [0, 0, 0, 0],
            commands: "".to_string(),
        }
    }

    fn shift_by_tick(&mut self, adjustment: i64) {
        if adjustment == 0 {
            return;
        }

        if self.start_tick.is_some() {
            self.start_tick = Some(self.start_tick.unwrap() + adjustment);

            if self.start_tick.unwrap() < 0 {
                self.start_tick = None;
            }
        } else if adjustment > 0 {
            self.start_tick = Some(adjustment)
        }

        if self.start_time.is_some() {
            self.start_time = Some(self.start_time.unwrap() + (adjustment as f64 / 66.0));

            if self.start_time.unwrap() < 0.0 {
                self.start_time = None;
            }
        }

        if self.skip_to_tick.is_some() {
            self.skip_to_tick = Some(self.skip_to_tick.unwrap() + adjustment);
        }

        if self.skip_to_time.is_some() {
            self.skip_to_time = Some(self.skip_to_time.unwrap() + (adjustment as f64 / 66.0));
        }

        if self.skip_to_time.unwrap() < 0.0 {
            self.skip_to_time = None;
        }

        if self.skip_to_tick.unwrap() < 0 {
            self.skip_to_tick = None;
        }
    }

    fn shift_by_time(&mut self, adjustment: f64) {
        if adjustment == 0.0 {
            return;
        }

        if self.start_tick.is_some() {
            self.start_tick = Some(self.start_tick.unwrap() + (adjustment * 66.0) as i64);

            if self.start_tick.unwrap() < 0 {
                self.start_tick = None;
            }
        }

        if self.start_time.is_some() {
            self.start_time = Some(self.start_time.unwrap() + adjustment);

            if self.start_time.unwrap() < 0.0 {
                self.start_time = None;
            }
        } else if adjustment > 0.0 {
            self.start_time = Some(adjustment)
        }

        if self.skip_to_tick.is_some() {
            self.skip_to_tick = Some(self.start_tick.unwrap() + (adjustment * 66.0) as i64);
        }

        if self.skip_to_time.is_some() {
            self.skip_to_time = Some(self.skip_to_time.unwrap() + adjustment);
        }

        if self.skip_to_time.unwrap() < 0.0 {
            self.skip_to_time = None;
        }

        if self.skip_to_tick.unwrap() < 0 {
            self.skip_to_tick = None;
        }
    }
}

impl From<CaptureMatches<'_, '_>> for Properties {
    fn from(factory_properties: CaptureMatches) -> Self {
        let mut property = Properties::new();

        for prop in factory_properties {
            //* readability is for nerds
            match prop[1].trim() {
                "name" => {
                    property.name = prop[2].to_string();
                }
                "starttime" => {
                    property.start_time = Some(prop[2].parse::<f64>().unwrap());
                }
                "starttick" => {
                    property.start_tick = Some(prop[2].parse::<i64>().unwrap());
                }
                "skiptotick" => {
                    property.skip_to_tick = Some(prop[2].parse::<i64>().unwrap());
                }
                "skiptotime" => {
                    property.skip_to_time = Some(prop[2].parse::<f64>().unwrap());
                }
                "stoptick" => {
                    property.stop_tick = Some(prop[2].parse::<i64>().unwrap());
                }
                "stoptime" => {
                    property.stop_time = Some(prop[2].parse::<f64>().unwrap());
                }
                "track" => {
                    property.track = prop[2].parse::<i64>().unwrap();
                }
                "spline" => {
                    property.spline = &prop[2] == "1";
                }
                "stayout" => {
                    property.stayout = &prop[2] == "1";
                }
                "finalfov" => {
                    property.final_fov = prop[2].parse::<f64>().unwrap();
                }
                "playbackrate" => {
                    property.playback_rate = prop[2].parse::<f64>().unwrap();
                }
                "fovrateout" => {
                    property.fade_out = prop[2].parse::<f64>().unwrap();
                }
                "fovratein" => {
                    property.fade_in = prop[2].parse::<f64>().unwrap();
                }
                "fovhold" => {
                    property.hold_time = prop[2].parse::<f64>().unwrap();
                }
                "zoomintime" => {
                    property.zoom_in_time = prop[2].parse::<f64>().unwrap();
                }
                "zoomouttime" => {
                    property.zoom_out_time = prop[2].parse::<f64>().unwrap();
                }
                "duration" => {
                    property.duration = prop[2].parse::<f64>().unwrap();
                }
                "pausetime" => {
                    property.duration = prop[2].parse::<f64>().unwrap();
                }
                "holdtime" => {
                    property.hold_time = prop[2].parse::<f64>().unwrap();
                }
                "fadein" => {
                    property.fade_in = prop[2].parse::<f64>().unwrap();
                }
                "fadeout" => {
                    property.fade_out = prop[2].parse::<f64>().unwrap();
                }
                "fxtime" => {
                    property.fx_time = prop[2].parse::<f64>().unwrap();
                }
                "sound" => {
                    property.sound = prop[2].to_string();
                }
                "message" => {
                    property.message = prop[2].to_string();
                }
                "font" => {
                    property.font = prop[2].to_string();
                }
                "commands" => {
                    property.commands = prop[2].to_string();
                }
                "x" => {
                    property.xy[0] = prop[2].parse::<f64>().unwrap();
                }
                "y" => {
                    property.xy[1] = prop[2].parse::<f64>().unwrap();
                }
                "r" => {
                    property.rgba1[0] = prop[2].parse::<u8>().unwrap();
                }
                "r1" => {
                    property.rgba1[0] = prop[2].parse::<u8>().unwrap();
                }
                "g" => {
                    property.rgba1[1] = prop[2].parse::<u8>().unwrap();
                }
                "g1" => {
                    property.rgba1[1] = prop[2].parse::<u8>().unwrap();
                }
                "b" => {
                    property.rgba1[2] = prop[2].parse::<u8>().unwrap();
                }
                "b1" => {
                    property.rgba1[2] = prop[2].parse::<u8>().unwrap();
                }
                "a" => {
                    property.rgba1[3] = prop[2].parse::<u8>().unwrap();
                }
                "a1" => {
                    property.rgba1[3] = prop[2].parse::<u8>().unwrap();
                }
                "r2" => {
                    property.rgba2[0] = prop[2].parse::<u8>().unwrap();
                }
                "g2" => {
                    property.rgba2[1] = prop[2].parse::<u8>().unwrap();
                }
                "b2" => {
                    property.rgba2[2] = prop[2].parse::<u8>().unwrap();
                }
                "a2" => {
                    property.rgba2[3] = prop[2].parse::<u8>().unwrap();
                }
                "FFADE_IN" => {
                    property.fade = Fade::FadeIn;
                }
                "FFADE_OUT" => {
                    property.fade = Fade::FadeOut;
                }
                "FFADE_MODULATE" => {
                    property.fade = Fade::Modulate;
                }
                "FFADE_STAYOUT" => {
                    property.fade = Fade::StayOut;
                }
                "FFADE_PURGE" => {
                    property.fade = Fade::Purge;
                }
                "FLICKER" => {
                    property.effect = TextEffect::Flicker;
                }
                "FADEINOUT" => {
                    property.effect = TextEffect::FadeInOut;
                }
                "WRITEOUT" => {
                    property.effect = TextEffect::WriteOut;
                }
                _ => {}
            }
        }

        property
    }
}

#[derive(Debug, Clone)]
pub enum ActionType {
    SkipAhead,
    StopPlayback,
    PlayCommands,
    ScreenFadeStart,
    TextMessageStart,
    PlayCDTrackStart,
    PlaySoundStart,
    Pause,
    ChangePlaybackRate,
    ZoomFov,
}

#[derive(Debug, Clone)]
pub enum Action {
    SkipAhead(Properties),
    StopPlayback(Properties),
    PlayCommands(Properties),
    ScreenFadeStart(Properties),
    TextMessageStart(Properties),
    PlayCDTrackStart(Properties),
    PlaySoundStart(Properties),
    Pause(Properties),
    ChangePlaybackRate(Properties),
    ZoomFov(Properties),
}

impl Action {
    pub fn new(factory: ActionType) -> Self {
        match factory {
            ActionType::SkipAhead => Action::SkipAhead(Properties::new()),
            ActionType::StopPlayback => Action::StopPlayback(Properties::new()),
            ActionType::PlayCommands => Action::PlayCommands(Properties::new()),
            ActionType::ScreenFadeStart => Action::ScreenFadeStart(Properties::new()),
            ActionType::TextMessageStart => Action::TextMessageStart(Properties::new()),
            ActionType::PlayCDTrackStart => Action::PlayCDTrackStart(Properties::new()),
            ActionType::PlaySoundStart => Action::PlaySoundStart(Properties::new()),
            ActionType::Pause => Action::Pause(Properties::new()),
            ActionType::ChangePlaybackRate => Action::ChangePlaybackRate(Properties::new()),
            ActionType::ZoomFov => Action::ZoomFov(Properties::new()),
            _ => {
                todo!();
            }
        }
    }

    pub fn props(&self) -> Properties {
        match self {
            Action::SkipAhead(props) => {
                return props.clone();
            }
            Action::StopPlayback(props) => {
                return props.clone();
            }
            Action::PlayCommands(props) => {
                return props.clone();
            }
            Action::ScreenFadeStart(props) => {
                return props.clone();
            }
            Action::TextMessageStart(props) => {
                return props.clone();
            }
            Action::PlayCDTrackStart(props) => {
                return props.clone();
            }
            Action::PlaySoundStart(props) => {
                return props.clone();
            }
            Action::Pause(props) => {
                return props.clone();
            }
            Action::ChangePlaybackRate(props) => {
                return props.clone();
            }
            Action::ZoomFov(props) => {
                return props.clone();
            }
        }
    }

    pub fn set_props(&mut self, new_props: Properties) -> Self {
        return match self {
            Action::SkipAhead(_) => Action::SkipAhead(new_props),
            Action::StopPlayback(_) => Action::StopPlayback(new_props),
            Action::PlayCommands(_) => Action::PlayCommands(new_props),
            Action::ScreenFadeStart(_) => Action::ScreenFadeStart(new_props),
            Action::TextMessageStart(_) => Action::TextMessageStart(new_props),
            Action::PlayCDTrackStart(_) => Action::PlayCDTrackStart(new_props),
            Action::PlaySoundStart(_) => Action::PlaySoundStart(new_props),
            Action::Pause(_) => Action::Pause(new_props),
            Action::ChangePlaybackRate(_) => Action::ChangePlaybackRate(new_props),
            Action::ZoomFov(_) => Action::ZoomFov(new_props),
        };
    }

    pub fn props_mut(&mut self) -> &mut Properties {
        match self {
            Action::SkipAhead(props) => &mut *props,
            Action::StopPlayback(props) => &mut *props,
            Action::PlayCommands(props) => &mut *props,
            Action::ScreenFadeStart(props) => &mut *props,
            Action::TextMessageStart(props) => &mut *props,
            Action::PlayCDTrackStart(props) => &mut *props,
            Action::PlaySoundStart(props) => &mut *props,
            Action::Pause(props) => &mut *props,
            Action::ChangePlaybackRate(props) => &mut *props,
            Action::ZoomFov(props) => &mut *props,
        }
    }
}

impl From<String> for Action {
    fn from(factory_text: String) -> Self {
        let re = Regex::new("(.*) \"(.*)\"").unwrap();

        let mut events = re.captures_iter(&factory_text);

        let factory = events.next().unwrap();

        return match &factory[2] {
            "SkipAhead" => Action::SkipAhead(Properties::from(events)),
            "StopPlayback" => Action::StopPlayback(Properties::from(events)),
            "PlayCommands" => Action::PlayCommands(Properties::from(events)),
            "ScreenFadeStart" => Action::ScreenFadeStart(Properties::from(events)),
            "TextMessageStart" => Action::TextMessageStart(Properties::from(events)),
            "PlayCDTrackStart" => Action::PlayCDTrackStart(Properties::from(events)),
            "PlaySoundStart" => Action::PlaySoundStart(Properties::from(events)),
            "Pause" => Action::Pause(Properties::from(events)),
            "ChangePlaybackRate" => Action::ChangePlaybackRate(Properties::from(events)),
            "Zoom FOV" => Action::ZoomFov(Properties::from(events)),
            _ => {
                println!("{:?}", &factory[2]);
                todo!();
            }
        };
    }
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        match action {
            Action::SkipAhead(props) => {
                let mut action_str = "\t\tfactory \"SkipAhead\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                if props.skip_to_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tskiptotick \"{}\"\r\n",
                        props.skip_to_tick.unwrap()
                    );
                }

                if props.skip_to_time.is_some() {
                    action_str = format!(
                        "{}\t\tskiptotime \"{:.3}\"\r\n",
                        action_str,
                        props.skip_to_time.unwrap()
                    );
                }

                action_str
            }
            Action::StopPlayback(props) => {
                let mut action_str = "\t\tfactory \"StopPlayback\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                action_str
            }
            Action::PlayCommands(props) => {
                let mut action_str = "\t\tfactory \"PlayCommands\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                write!(
                    &mut action_str,
                    "\t\tcommands \"{:.6}\"\r\n",
                    props.commands
                );

                action_str
            }
            Action::ScreenFadeStart(props) => {
                let mut action_str = "\t\tfactory \"ScreenFadeStart\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                write!(
                    &mut action_str,
                    "\t\tduration \"{:.3}\"\r\n",
                    props.duration
                );
                write!(
                    &mut action_str,
                    "\t\tholdtime \"{:.3}\"\r\n",
                    props.hold_time
                );
                write!(&mut action_str, "\t\t{}\r\n", String::from(props.fade));
                write!(&mut action_str, "\t\tr \"{}\"\r\n", props.rgba1[0]);
                write!(&mut action_str, "\t\tg \"{}\"\r\n", props.rgba1[1]);
                write!(&mut action_str, "\t\tb \"{}\"\r\n", props.rgba1[2]);
                write!(&mut action_str, "\t\ta \"{}\"\r\n", props.rgba1[3]);

                action_str
            }
            Action::TextMessageStart(props) => {
                let mut action_str = "\t\tfactory \"TextMessageStart\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                write!(&mut action_str, "\t\tmessage \"{}\"\r\n", props.message);
                write!(&mut action_str, "\t\tfont \"{}\"\r\n", props.font);
                write!(&mut action_str, "\t\tfadein \"{:.3}\"\r\n", props.fade_in);
                write!(&mut action_str, "\t\tfadeout \"{:.3}\"\r\n", props.fade_out);
                write!(
                    &mut action_str,
                    "\t\tholdtime \"{:.3}\"\r\n",
                    props.hold_time
                );
                write!(&mut action_str, "\t\tfxtime \"{:.3}\"\r\n", props.fx_time);
                write!(&mut action_str, "\t\t{}\r\n", String::from(props.effect));

                write!(&mut action_str, "\t\tx \"{}\"\r\n", props.xy[0]);
                write!(&mut action_str, "\t\ty \"{}\"\r\n", props.xy[1]);

                write!(&mut action_str, "\t\tr1 \"{}\"\r\n", props.rgba1[0]);
                write!(&mut action_str, "\t\tg1 \"{}\"\r\n", props.rgba1[1]);
                write!(&mut action_str, "\t\tb1 \"{}\"\r\n", props.rgba1[2]);
                write!(&mut action_str, "\t\ta1 \"{}\"\r\n", props.rgba1[3]);

                write!(&mut action_str, "\t\tr2 \"{}\"\r\n", props.rgba2[0]);
                write!(&mut action_str, "\t\tg2 \"{}\"\r\n", props.rgba2[1]);
                write!(&mut action_str, "\t\tb2 \"{}\"\r\n", props.rgba2[2]);
                write!(&mut action_str, "\t\ta2 \"{}\"\r\n", props.rgba2[3]);

                action_str
            }
            Action::PlayCDTrackStart(props) => {
                let mut action_str = "\t\tfactory \"PlayCDTrackStart\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                write!(&mut action_str, "\t\ttrack \"{}\"\r\n", props.track);

                action_str
            }
            Action::PlaySoundStart(props) => {
                let mut action_str = "\t\tfactory \"PlaySoundStart\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                write!(&mut action_str, "\t\tsound \"{}\"\r\n", props.sound);

                action_str
            }
            Action::Pause(props) => {
                let mut action_str = "\t\tfactory \"Pause\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                if props.stop_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstoptick \"{}\"\r\n",
                        props.stop_tick.unwrap()
                    );
                }

                if props.stop_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstoptime \"{}\"\r\n",
                        props.stop_time.unwrap()
                    );
                }

                write!(
                    &mut action_str,
                    "\t\tpausetime \"{:.6}\"\r\n",
                    props.duration
                );

                action_str
            }
            Action::ChangePlaybackRate(props) => {
                let mut action_str = "\t\tfactory \"ChangePlaybackRate\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                if props.stop_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstoptick \"{}\"\r\n",
                        props.stop_tick.unwrap()
                    );
                }

                if props.stop_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstoptime \"{}\"\r\n",
                        props.stop_time.unwrap()
                    );
                }

                write!(
                    &mut action_str,
                    "\t\tplaybackrate \"{:.6}\"\r\n",
                    props.final_fov
                );

                action_str
            }
            Action::ZoomFov(props) => {
                let mut action_str = "\t\tfactory \"Zoom FOV\"\r\n".to_string();

                write!(&mut action_str, "\t\tname \"{}\"\r\n", props.name);

                if props.start_tick.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttick \"{}\"\r\n",
                        props.start_tick.unwrap()
                    );
                }

                if props.start_time.is_some() {
                    write!(
                        &mut action_str,
                        "\t\tstarttime \"{}\"\r\n",
                        props.start_time.unwrap()
                    );
                }

                write!(&mut action_str, "\t\tspline \"{}\"\r\n", props.spline);
                write!(&mut action_str, "\t\tstayout \"{}\"\r\n", props.stayout);
                write!(
                    &mut action_str,
                    "\t\tfinalfov \"{:.6}\"\r\n",
                    props.final_fov
                );
                write!(
                    &mut action_str,
                    "\t\tfovrateout \"{:.6}\"\r\n",
                    props.fade_out
                );
                write!(
                    &mut action_str,
                    "\t\tfovratein \"{:.6}\"\r\n",
                    props.fade_in
                );
                write!(
                    &mut action_str,
                    "\t\tfovhold \"{:.6}\"\r\n",
                    props.hold_time
                );

                action_str
            }
        }
    }
}
