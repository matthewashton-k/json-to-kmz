use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use serde_json::Value;
use zip::DateTime;
use zip::write::FileOptions;
use zip::CompressionMethod::Stored;
use std::time::SystemTime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("locations.json"); // location data from westernmininghistory
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // where to write the output
    let output_file = File::create("locations.kmz")?;

    // kmz files are just zip files in disguise :)
    let mut zip = zip::ZipWriter::new(output_file);
    let options = FileOptions::default()
        .compression_method(Stored)
        .last_modified_time(DateTime::default());
    zip.start_file("doc.kml", options)?;

    write!(zip, r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
<Document>"#)?; // write header

    let mut i = 0;
    for line in reader.lines() { // each line should have a separate json object
        let line = line?;
        let v: Value = serde_json::from_str(&line)?; // parse line

        // get name and location
        let name = v["name"].as_str().unwrap().replace("&", "and"); // for some reason kmz doesnt like the & symbol which is used sometimes in the original .json file
        let lat = v["lat"].as_f64().unwrap();
        let long = v["long"].as_f64().unwrap();

        // the list of mines is VERY LONG so this is just here because I wanted a representative sample of mines so I could look at the distribution of mines accross the west
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
