use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use serde_json::Value;
use zip::DateTime;
use zip::write::FileOptions;
use zip::CompressionMethod::Stored;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("locations.json");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let output_file = File::create("locations.kmz")?;
    let mut zip = zip::ZipWriter::new(output_file);
    let options = FileOptions::default()
        .compression_method(Stored)
        .last_modified_time(DateTime::default());
    zip.start_file("doc.kml", options)?;

    write!(zip, r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
<Document>"#)?;

    let mut i = 0;
    for line in reader.lines() {
        let line = line?;
        let v: Value = serde_json::from_str(&line)?;

        let name = v["name"].as_str().unwrap().replace("&", "and");
        let lat = v["lat"].as_f64().unwrap();
        let long = v["long"].as_f64().unwrap();

        if i % 1000 == 0 {
            write!(zip, r#"<NetworkLink>
<name>Region {}</name>
<Region>
<LatLonAltBox>
<north>{}</north>
<south>{}</south>
<east>{}</east>
<west>{}</west>
</LatLonAltBox>
<Lod>
<minLodPixels>128</minLodPixels>
<maxLodPixels>1024</maxLodPixels>
</Lod>
</Region>
<Link>
<href>region{}.kml</href>
<viewRefreshMode>onRegion</viewRefreshMode>
</Link>
</NetworkLink>"#, i / 1000, lat + 0.1, lat - 0.1, long + 0.1, long - 0.1, i / 1000)?;
        }

        i += 1;
    }

    write!(zip, r#"</Document>
</kml>"#)?;

    Ok(())
}
