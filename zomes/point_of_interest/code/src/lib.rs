#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;
extern crate holochain_collections;
extern crate geojson;
extern crate geohash;

use geojson::{Feature, FeatureCollection};
use geohash::{encode, Coordinate};

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, dna::entry_types::Sharing,
    error::HolochainError, json::JsonString
};
use holochain_collections::bucket_set::{
    self,
    BucketSetStorable,
    BucketIterable,
};

// Use newtype idiom for geojson::Feature to implement DefaultJson
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PointOfInterest(Feature);

// Implement traits for holochain-collections
// Generate geohash bucket id for given point
// Use 1 character geohash which gives 32 buckets
impl BucketSetStorable for PointOfInterest {
    fn derive_bucket_id(&self) -> String {
        // get point data out of PointOfInterest
        let value = self.to_owned().0.geometry.unwrap().value;
        match value {
            // If we were given a point geometry
            geojson::Value::Point(value) => {
                // encode lat/lng to geohash
                let c = Coordinate { x: value[0], y: value[1] };
                // use first char of geohash as bucket index (32 buckets)
                let hash = encode(c, 1).expect("Invalid coordinate");
                hash
            },
            // Anything else we can't handle
            _ => unreachable!(),
        }
    }
}

// All bucket ids which is the 32 character geohash alphabet
impl BucketIterable for PointOfInterest {
    fn buckets() -> Box<Iterator<Item = String>> {
        // use base32 geohash alphabet
        let alphabet = "0123456789bcdefghjkmnpqrstuvwxyz"
        .chars().map(|c| {
            c.to_string()
        });
        Box::new(alphabet)
    }
}

// Use newtype idiom for geojson::FeatureCollection to implement DefaultJson
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PointsOfInterest(FeatureCollection);

// save point to bucket based on geographic location
pub fn handle_add_point(entry: PointOfInterest) -> ZomeApiResult<Address> {
    bucket_set::store("point_of_interest".into(), entry)
}

// return a wrapped geojson::FeatureCollection of all points to client
pub fn handle_get_all_points() -> ZomeApiResult<PointsOfInterest> {

    let points = bucket_set::retrieve_all::<PointOfInterest>("point_of_interest".into()).unwrap();
    let mut features: Vec<Feature> = Vec::new();

    for p in points {
        let point = hdk::utils::get_as_type::<PointOfInterest>(p.to_owned())?;
        let feature = point.0;
        features.push(feature);
    }
    
    let collection = FeatureCollection {
        bbox: None,
        features: features,
        foreign_members: None,
    };
    
    let poi = PointsOfInterest(collection);
    Ok(poi)
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "point_of_interest",
        description: "point of interest information",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<PointOfInterest>| {
            Ok(())
        }
    )
}

define_zome! {
    entries: [
       definition(),
       bucket_set::bucket_entry_def_for("point_of_interest".into())
    ]

    genesis: || { Ok(()) }

    functions: [
        add_point: {
            inputs: |entry: PointOfInterest|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_point
        }
        get_all_points: {
            inputs: | |,
            outputs: |result: ZomeApiResult<PointsOfInterest>|,
            handler: handle_get_all_points
        }
    ]

    traits: {
        hc_public [get_all_points, add_point]
    }
}
