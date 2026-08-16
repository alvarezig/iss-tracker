use crate::client::IssPosition;

pub struct Observer {
    pub latitude: f64,
    pub longitude: f64,
}

impl Observer {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    pub fn distance_to(&self, iss: &IssPosition) -> f64 {
        const EARTH_RADIUS: f64 = 6371.0;

        let lat1 = self.latitude.to_radians();
        let lon1 = self.longitude.to_radians();
        let lat2 = iss.latitude.to_radians();
        let lon2 = iss.longitude.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let theta = 2.0 * a.sqrt().asin();

        let r1 = EARTH_RADIUS;
        let r2 = EARTH_RADIUS + iss.altitude;

        (r1.powi(2) + r2.powi(2) - 2.0 * r1 * r2 * theta.cos()).sqrt()
    }
}