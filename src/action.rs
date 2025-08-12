use regex::{CaptureMatches, Regex};
use std::fmt::{self, Display, Formatter, Write};

#[derive(Debug, Clone, Copy)]
pub enum TextEffect {
    Flicker,
    FadeInOut,
    WriteOut,
}

impl Display for TextEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let repr: &str = From::from(*self);
        write!(f, "{repr}")
    }
}

impl From<TextEffect> for &'static str {
    fn from(text_effect: TextEffect) -> Self {
        match text_effect {
            TextEffect::Flicker => "FLICKER \"1\"",
            TextEffect::FadeInOut => "FADEINOUT \"1\"",
            TextEffect::WriteOut => "WRITEOUT \"1\"",
        }
    }
}

impl From<TextEffect> for String {
    fn from(text_effect: TextEffect) -> Self {
        text_effect.to_string()
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
    pub fade_in_enabled: bool,
    pub fade_out_enabled: bool,
    pub modulate_enabled: bool,
    pub stay_out_enabled: bool,
    pub purge_enabled: bool,
    pub xy: [f64; 2],
    pub rgba1: [u8; 4],
    pub rgba2: [u8; 4],
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

impl Properties {
    pub fn new() -> Self {
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
            fade_in_enabled: false,
            fade_out_enabled: false,
            modulate_enabled: false,
            stay_out_enabled: false,
            purge_enabled: false,
            xy: [0.0, 0.0],
            rgba1: [0, 0, 0, 0],
            rgba2: [0, 0, 0, 0],
            commands: "".to_string(),
        }
    }

    pub fn shift_by_tick(&mut self, adjustment: i64) {
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

        if self.skip_to_time.is_some() && self.skip_to_time.unwrap() < 0.0 {
            self.skip_to_time = None;
        }

        if self.skip_to_tick.is_some() && self.skip_to_tick.unwrap() < 0 {
            self.skip_to_tick = None;
        }
    }

    pub fn shift_by_time(&mut self, adjustment: f64) {
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
                    property.fade_in_enabled = true;
                }
                "FFADE_OUT" => {
                    property.fade_out_enabled = true;
                }
                "FFADE_MODULATE" => {
                    property.modulate_enabled = true;
                }
                "FFADE_STAYOUT" => {
                    property.stay_out_enabled = true;
                }
                "FFADE_PURGE" => {
                    property.purge_enabled = true;
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
            Action::SkipAhead(props) => props.clone(),
            Action::StopPlayback(props) => props.clone(),
            Action::PlayCommands(props) => props.clone(),
            Action::ScreenFadeStart(props) => props.clone(),
            Action::TextMessageStart(props) => props.clone(),
            Action::PlayCDTrackStart(props) => props.clone(),
            Action::PlaySoundStart(props) => props.clone(),
            Action::Pause(props) => props.clone(),
            Action::ChangePlaybackRate(props) => props.clone(),
            Action::ZoomFov(props) => props.clone(),
        }
    }

    pub fn set_props(&mut self, new_props: Properties) -> Self {
        match self {
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
        }
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

        match &factory[2] {
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
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::SkipAhead(props) => {
                write!(f, "\t\tfactory \"SkipAhead\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n",)?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.skip_to_tick.as_ref() {
                    write!(f, "\t\tskiptotick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.skip_to_time.as_ref() {
                    write!(f, "\t\tskiptotime \"{prop:.3}\"\r\n")?;
                }
            }
            Action::StopPlayback(props) => {
                write!(f, "\t\tfactory \"StopPlayback\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }
            }
            Action::PlayCommands(props) => {
                write!(f, "\t\tfactory \"PlayCommands\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n");
                }

                write!(f, "\t\tcommands \"{}\"\r\n", props.commands)?;
            }
            Action::ScreenFadeStart(props) => {
                write!(f, "\t\tfactory \"ScreenFadeStart\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                write!(f, "\t\tduration \"{:.3}\"\r\n", props.duration)?;

                write!(f, "\t\tholdtime \"{:.3}\"\r\n", props.hold_time)?;

                if props.fade_in_enabled {
                    write!(f, "\t\tFFADE_IN \"1\"\r\n")?;
                }

                if props.fade_out_enabled {
                    write!(f, "\t\tFFADE_OUT \"1\"\r\n")?;
                }

                if props.modulate_enabled {
                    write!(f, "\t\tFFADE_MODULATE \"1\"\r\n")?;
                }

                if props.stay_out_enabled {
                    write!(f, "\t\tFFADE_STAYOUT \"1\"\r\n")?;
                }

                if props.purge_enabled {
                    write!(f, "\t\tFFADE_PURGE \"1\"\r\n")?;
                }

                write!(f, "\t\tr \"{}\"\r\n", props.rgba1[0])?;
                write!(f, "\t\tg \"{}\"\r\n", props.rgba1[1])?;
                write!(f, "\t\tb \"{}\"\r\n", props.rgba1[2])?;
                write!(f, "\t\ta \"{}\"\r\n", props.rgba1[3])?;
            }
            Action::TextMessageStart(props) => {
                write!(f, "\t\tfactory \"TextMessageStart\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                write!(f, "\t\tmessage \"{}\"\r\n", props.message)?;
                write!(f, "\t\tfont \"{}\"\r\n", props.font)?;
                write!(f, "\t\tfadein \"{:.3}\"\r\n", props.fade_in)?;
                write!(f, "\t\tfadeout \"{:.3}\"\r\n", props.fade_out)?;
                write!(f, "\t\tholdtime \"{:.3}\"\r\n", props.hold_time)?;
                write!(f, "\t\tfxtime \"{:.3}\"\r\n", props.fx_time)?;
                write!(f, "\t\t{}\r\n", props.effect)?;

                write!(f, "\t\tx \"{}\"\r\n", props.xy[0])?;
                write!(f, "\t\ty \"{}\"\r\n", props.xy[1])?;

                write!(f, "\t\tr1 \"{}\"\r\n", props.rgba1[0])?;
                write!(f, "\t\tg1 \"{}\"\r\n", props.rgba1[1])?;
                write!(f, "\t\tb1 \"{}\"\r\n", props.rgba1[2])?;
                write!(f, "\t\ta1 \"{}\"\r\n", props.rgba1[3])?;

                write!(f, "\t\tr2 \"{}\"\r\n", props.rgba2[0])?;
                write!(f, "\t\tg2 \"{}\"\r\n", props.rgba2[1])?;
                write!(f, "\t\tb2 \"{}\"\r\n", props.rgba2[2])?;
                write!(f, "\t\ta2 \"{}\"\r\n", props.rgba2[3])?;
            }
            Action::PlayCDTrackStart(props) => {
                write!(f, "\t\tfactory \"PlayCDTrackStart\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                write!(f, "\t\ttrack \"{}\"\r\n", props.track)?;
            }
            Action::PlaySoundStart(props) => {
                write!(f, "\t\tfactory \"PlaySoundStart\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                write!(f, "\t\tsound \"{}\"\r\n", props.sound)?;
            }
            Action::Pause(props) => {
                write!(f, "\t\tfactory \"Pause\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.stop_tick.as_ref() {
                    write!(f, "\t\tstoptick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.stop_time.as_ref() {
                    write!(f, "\t\tstoptime \"{prop}\"\r\n")?;
                }

                write!(f, "\t\tpausetime \"{:.6}\"\r\n", props.duration)?;
            }
            Action::ChangePlaybackRate(props) => {
                write!(f, "\t\tfactory \"ChangePlaybackRate\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.stop_tick.as_ref() {
                    write!(f, "\t\tstoptick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.stop_time.as_ref() {
                    write!(f, "\t\tstoptime \"{prop}\"\r\n")?;
                }

                write!(f, "\t\tplaybackrate \"{:.6}\"\r\n", props.final_fov)?;
            }
            Action::ZoomFov(props) => {
                write!(f, "\t\tfactory \"Zoom FOV\"\r\n")?;

                write!(f, "\t\tname \"{}\"\r\n", props.name)?;

                if let Some(prop) = props.start_tick.as_ref() {
                    write!(f, "\t\tstarttick \"{prop}\"\r\n")?;
                }

                if let Some(prop) = props.start_time.as_ref() {
                    write!(f, "\t\tstarttime \"{prop}\"\r\n")?;
                }

                write!(f, "\t\tspline \"{}\"\r\n", props.spline)?;
                write!(f, "\t\tstayout \"{}\"\r\n", props.stayout)?;
                write!(f, "\t\tfinalfov \"{:.6}\"\r\n", props.final_fov)?;
                write!(f, "\t\tfovrateout \"{:.6}\"\r\n", props.fade_out)?;
                write!(f, "\t\tfovratein \"{:.6}\"\r\n", props.fade_in)?;
                write!(f, "\t\tfovhold \"{:.6}\"\r\n", props.hold_time)?;
            }
        };

        Ok(())
    }
}

impl From<Action> for String {
    fn from(action: Action) -> Self {
        action.to_string()
    }
}
