use anyhow::{Context, Result};
use console::style;
use image::imageops::overlay;
use image::DynamicImage;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::BufWriter;
use std::path::Path;

use crate::commands::prelude::*;
use crate::constants::*;

struct Asset<'a> {
    layers: BTreeSet<String>,
    base_layer: DynamicImage,
    metadata: Metadata<'a>,
}

#[derive(Serialize, Deserialize)]
struct Metadata<'a> {
    name: String,
    description: String,
    image: String,
    #[serde(borrow = "'a ")]
    attributes: Vec<Trait<'a>>,
}

#[derive(Serialize, Deserialize)]
struct Trait<'a> {
    trait_type: &'a str,
    value: &'a str,
}

pub struct Generate;

impl GenericCommand for Generate {
    fn run(&self, matches: &ArgMatches) -> Result<()> {
        let path = matches.value_of("root").expect("required argument");

        // Get the folders for each layer
        let root_dir = fs::read_dir(path)
            .with_context(|| ArtGenError::MissingDirectory(path.to_string()))
            .unwrap();

        let mut subdirs: Vec<fs::DirEntry> = root_dir.map(|subdir| subdir.unwrap()).collect();

        // Sort the subdirs in alphanumeric order
        // The subfolder names should be prepended with a number corresponding to the desired order of layering
        // (e.g. 01<base_layer>, 02<middle_layer>, 03<top_layer>)
        subdirs.sort_by_key(|dir| dir.path());

        let collection_size = matches
            .value_of("number")
            .expect("required_argument")
            .parse::<u128>()
            .with_context(|| ArtGenError::InvalidCollectionSize)
            .unwrap();

        println!(
            "\n{}{}\n",
            PALETTE_EMOJI,
            style("We're generating some digital art!").yellow().bold(),
        );

        // Create a HashMap to track which assets have been generated
        let mut asset_already_generated = HashMap::new();

        let output_dir = ASSETS_OUTPUT;
        // Create an output directory to store the generated assets
        fs::create_dir_all(output_dir)?;

        let num_generated: u128 = fs::read_dir(output_dir)
            .unwrap()
            .count()
            .try_into()
            .unwrap();

        let metadata_dir = METADATA_OUTPUT;
        // Create a metadata directory to store the generated asset metadata
        fs::create_dir_all(metadata_dir)?;

        // TODO Put this into a dedicated function
        let mut rarity_tracker: Vec<Vec<(String, u128)>> = Vec::new();

        for folder in subdirs {
            let mut layer_rarity: Vec<(String, u128)> = Vec::new();
            for file in fs::read_dir(folder.path()).unwrap() {
                let file = file.unwrap().path();

                // Get the first two characters of the file name (rarity weight)
                let rarity_weight = &file.file_stem().unwrap().to_str().unwrap()[..2];

                layer_rarity.push((
                    file.display().to_string(),
                    rarity_weight.parse::<u128>().unwrap(),
                ));
            }
            rarity_tracker.push(layer_rarity);
        }

        // Create the desired number of assets for the collection
        for i in 0..collection_size {
            let current_image;
            let base_layer;
            let metadata;

            let current_id = i + num_generated;

            loop {
                let asset = gen_asset(&rarity_tracker, current_id)?;

                if !asset_already_generated.contains_key(&asset.layers) {
                    current_image = asset.layers;
                    base_layer = asset.base_layer;
                    metadata = asset.metadata;
                    break;
                }
            }
            asset_already_generated.insert(current_image, true);

            // TODO Add operation in the case that no new assets can be generated

            base_layer.save(format!("{}/{}.png", output_dir, current_id))?;

            let f = fs::File::create(format!("{}/{}", metadata_dir, current_id))
                .expect("Unable to create the metadata file");
            let bw = BufWriter::new(f);
            serde_json::to_writer_pretty(bw, &metadata).expect("Unable to write the metadata file");

            println!("Generated ID {}", current_id);
        }

        Ok(())
    }
}

fn gen_asset(rarity_tracker: &[Vec<(String, u128)>], current_id: u128) -> Result<Asset> {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    let base_dist = WeightedIndex::new(rarity_tracker[0].iter().map(|item| item.1)).unwrap();

    // Select the base layer
    let base_layer_selection = &rarity_tracker[0][base_dist.sample(&mut rng)].0;

    let base_trait_metadata = &Path::new(base_layer_selection)
        .parent()
        .unwrap()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()[2..];

    let base_layer_metadata = &Path::new(base_layer_selection)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()[2..];

    // Open the base layer image in order to be overlayed
    let mut base_layer_image = image::open(&base_layer_selection).unwrap();

    // Create a BTreeSet to store the top layers that get selected
    let mut top_layers = BTreeSet::new();

    let mut metadata_attributes: Vec<Trait> = Vec::new();

    for layer_rarity in &rarity_tracker[1..] {
        let layer_dist = WeightedIndex::new(layer_rarity.iter().map(|item| item.1)).unwrap();

        let file = &layer_rarity[layer_dist.sample(&mut rng)].0;
        top_layers.insert(file);
        let file_stem = &Path::new(file).file_stem().unwrap().to_str().unwrap()[2..];
        let parent_folder = &Path::new(file)
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()[2..];

        let new_trait = Trait {
            trait_type: parent_folder,
            value: file_stem,
        };

        metadata_attributes.push(new_trait);
    }

    let base_trait = Trait {
        trait_type: base_trait_metadata,
        value: base_layer_metadata,
    };

    metadata_attributes.push(base_trait);

    // TODO Abstract metadata fields to a separate config
    let metadata = Metadata {
        name: format!("<my_project> #{}", current_id),
        description: "<my_project> is a cultural revolution.".to_owned(),
        image: format!("ipfs://hash/{}.png", current_id),
        attributes: metadata_attributes,
    };

    // Create a BTreeSet to store all the layers
    let mut all_layers = BTreeSet::new();
    all_layers.insert(base_layer_selection.to_string());

    // Go through the toplayers and overlay the base layer with each toplayer in order
    for layer in top_layers {
        overlay(&mut base_layer_image, &image::open(&layer).unwrap(), 0, 0);
        all_layers.insert(layer.to_string());
    }

    let asset = Asset {
        layers: all_layers,
        base_layer: base_layer_image,
        metadata: metadata,
    };

    Ok(asset)
}
