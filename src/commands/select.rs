use anyhow::Result;
use console::style;
use image::imageops::overlay;
use image::DynamicImage;
use serde::Serialize;
use std::fs;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use crate::commands::prelude::*;
use crate::constants::*;

struct Asset<'a> {
    base_layer: DynamicImage,
    metadata: Metadata<'a>,
}

#[derive(Serialize)]
struct Metadata<'a> {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Trait<'a>>,
}

#[derive(Serialize)]
struct Trait<'a> {
    trait_type: &'a str,
    value: &'a str,
}

struct Layers<'a> {
    base: &'a str,
    top: Vec<&'a str>,
}

pub struct Select;

impl GenericCommand for Select {
    fn run(&self, matches: &ArgMatches) -> Result<()> {
        let layers = matches.values_of("layers").unwrap().collect::<Vec<&str>>();
        let layers = Layers {
            base: layers[0],
            top: layers[1..].to_vec(),
        };

        println!(
            "\n{}{}\n",
            PALETTE_EMOJI,
            style("We're generating some digital art!").yellow().bold(),
        );

        // Create an output directory to store the generated assets
        let output_dir = ASSETS_OUTPUT;
        fs::create_dir_all(output_dir)?;

        // Create a metadata directory to store the generated asset metadata
        let metadata_dir = METADATA_OUTPUT;
        fs::create_dir_all(metadata_dir)?;

        let stdout = std::io::stdout();
        let mut lock = stdout.lock();

        let current_id: u128 = fs::read_dir(output_dir).unwrap().count().try_into()?;

        let asset = gen_asset(layers, current_id)?;
        let base_layer = asset.base_layer;
        let metadata = asset.metadata;
        base_layer.save(format!("{}/{}.png", output_dir, current_id))?;

        let f = fs::File::create(format!("{}/{}", metadata_dir, current_id))?;
        let bw = BufWriter::new(f);
        serde_json::to_writer_pretty(bw, &metadata)?;

        writeln!(lock, "Generated ID {}", current_id)?;

        Ok(())
    }
}

fn gen_asset(layers: Layers, current_id: u128) -> Result<Asset> {
    // Select the base layer
    let base_layer_selection = Path::new(layers.base);

    let base_trait_metadata = &base_layer_selection
        .parent()
        .unwrap()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()[2..];

    let base_layer_metadata = &base_layer_selection.file_stem().unwrap().to_str().unwrap()[2..];

    // Open the base layer image in order to be overlayed
    let mut base_layer = image::open(&base_layer_selection)?;

    let base_trait = Trait {
        trait_type: base_trait_metadata,
        value: base_layer_metadata,
    };

    let mut metadata_attributes: Vec<Trait> = Vec::new();
    metadata_attributes.push(base_trait);

    for l in layers.top.clone() {
        let parent_folder = &Path::new(l)
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()[2..];

        let file_stem = &Path::new(l).file_stem().unwrap().to_str().unwrap()[2..];

        let new_trait = Trait {
            trait_type: parent_folder,
            value: file_stem,
        };

        metadata_attributes.push(new_trait);
    }

    let metadata = Metadata {
        name: format!("<my_project> #{}", current_id),
        description: "<my_project> is a cultural revolution.".to_owned(),
        image: format!("ar://hash/{}.png", current_id),
        attributes: metadata_attributes,
    };

    // Go through the toplayers and overlay the base layer with each toplayer in order
    for l in layers.top {
        overlay(&mut base_layer, &image::open(&l)?, 0, 0);
    }

    Ok(Asset {
        base_layer,
        metadata,
    })
}
