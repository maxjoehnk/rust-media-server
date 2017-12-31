use soundcloud;
use provider;
use library::Track;

#[derive(Debug, Clone)]
pub struct SoundcloudTrack {
    pub id: u64,
    pub title: String,
    pub url: String
}

impl From<SoundcloudTrack> for Track {
    fn from(track: SoundcloudTrack) -> Track {
        Track {
            title: track.title,
            artist: None,
            album: None,
            url: track.url,
            provider: provider::Provider::Soundcloud
        }
    }
}

impl From<soundcloud::Track> for SoundcloudTrack {
    fn from(track: soundcloud::Track) -> SoundcloudTrack {
        SoundcloudTrack {
            id: track.id,
            title: track.title,
            url: track.stream_url.unwrap()
        }
    }
}

impl From<soundcloud::Track> for provider::ProviderItem {
    fn from(track: soundcloud::Track) -> provider::ProviderItem {
        provider::ProviderItem::from(Track::from(SoundcloudTrack::from(track)))
    }
}