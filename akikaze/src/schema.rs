use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BannerData {
    patch: String,
    data: Vec<Phase>,
}

#[derive(Debug, Deserialize)]
pub struct Phase {
    pub phase: u8,
    pub banners: BannerContainer,
}

#[derive(Debug, Deserialize)]
pub struct BannerContainer {
    pub standardVersion: u8,
    pub events: Banner,
    pub weapons: WeaponBanner,
}

#[derive(Debug, Deserialize)]
pub struct Banner {
    pub featured: CharBanner,
    pub rateup: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CharBanner {
    pub bannerName: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct WeaponBanner {
    pub bannerName: String,
    pub fatepointsystem: bool,
    pub featured: Vec<Weapon>,
    pub rateup: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Weapon {
    pub name: String,
}