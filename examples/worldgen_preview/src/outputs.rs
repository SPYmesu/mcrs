use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use anyhow::{Context, bail};
use glam::IVec3;
use mcrs_minecraft_anvil::{Chunk, Heightmaps, PaletteNames, RegionFile, Section, write_chunk};
use mcrs_minecraft_chunk::{ColumnHeights, VoxelId};
use mcrs_minecraft_core::RegionPos;
use mcrs_minecraft_noise::sample_grid::SampleGrid;
use mcrs_minecraft_worldgen_density::program::Workspace;
use mcrs_minecraft_worldgen_density::router::{
    CONTINENTS, DEPTH, EROSION, RIDGES, TEMPERATURE, VEGETATION,
};
use mcrs_minecraft_worldgen_generator::staging::FilledSnapshot;
use mcrs_voxel::ColumnPos;
use serde::Serialize;

use crate::game_data::GameData;

#[derive(Serialize)]
struct Signals {
    temperature: Vec<f64>,
    vegetation: Vec<f64>,
    continentalness: Vec<f64>,
    erosion: Vec<f64>,
    depth: Vec<f64>,
    weirdness: Vec<f64>,
}

#[derive(Serialize)]
struct ClimateFile {
    x0: i32,
    z0: i32,
    step: i32,
    nx: i32,
    nz: i32,
    signals: Signals,
}

pub struct ClimateGrid {
    pub x0: i32,
    pub z0: i32,
    pub step: i32,
    pub nx: i32,
    pub nz: i32,
}

struct Names {
    blocks: PaletteNames<VoxelId>,
    biomes: PaletteNames<u8>,
    block_ids: HashMap<u16, VoxelId>,
    biome_ids: HashMap<u8, u8>,
}

impl Names {
    fn block(&mut self, data: &GameData, state: VoxelId) -> anyhow::Result<()> {
        if self.block_ids.contains_key(&state.0) {
            return Ok(());
        }
        let definitions = &data.blocks.0;
        let (name, properties) = definitions
            .properties_of(state.0)
            .with_context(|| format!("block state {} is unknown", state.0))?;
        let is_default = definitions
            .block(name.as_str())
            .is_some_and(|block| block.default_state_id.number() == state.0);
        let id = self
            .blocks
            .intern(
                name.as_str(),
                properties
                    .iter()
                    .filter(|_| !is_default)
                    .map(|(key, value)| (key.as_str(), value.as_str())),
            )
            .with_context(|| format!("block state {} has no palette entry", state.0))?;
        self.block_ids.insert(state.0, id);
        Ok(())
    }

    fn biome(&mut self, data: &GameData, biome: u8) -> anyhow::Result<()> {
        if self.biome_ids.contains_key(&biome) {
            return Ok(());
        }
        let name = data
            .biome_names
            .get(usize::from(biome))
            .with_context(|| format!("biome {biome} is unknown"))?;
        let id = self
            .biomes
            .intern(name.as_str(), [])
            .with_context(|| format!("biome {name} has no palette entry"))?;
        self.biome_ids.insert(biome, id);
        Ok(())
    }
}

fn chunk_of(
    data: &GameData,
    names: &mut Names,
    pos: ColumnPos,
    snapshot: &FilledSnapshot,
    rows: &std::ops::Range<i32>,
) -> anyhow::Result<Chunk> {
    let bottom = snapshot
        .y_sections
        .first()
        .copied()
        .context("a column has no sections")?;
    let mut sections = Vec::new();
    for y in rows.clone() {
        let (blocks, biomes) = y
            .checked_sub(bottom)
            .and_then(|slot| usize::try_from(slot).ok())
            .and_then(|slot| snapshot.sections.get(slot))
            .and_then(Option::as_ref)
            .context("a section of the dimension is not generated")?;
        let mut failure = None;
        blocks.for_each_distinct(|state| {
            failure = failure.take().or_else(|| names.block(data, state).err());
        });
        biomes.for_each_distinct(|biome| {
            failure = failure.take().or_else(|| names.biome(data, biome).err());
        });
        if let Some(error) = failure {
            return Err(error);
        }
        let block = |state: VoxelId| names.block_ids.get(&state.0).copied().unwrap_or(VoxelId(0));
        let biome = |value: u8| names.biome_ids.get(&value).copied().unwrap_or_default();
        sections.push(Section {
            y: i8::try_from(y).context("a section lies outside the anvil range")?,
            block_states: Some(blocks.0.map(block)),
            biomes: Some(biomes.0.map(biome)),
            noise_biomes: None,
            block_light: None,
            sky_light: None,
        });
    }
    let maps = snapshot
        .maps
        .as_ref()
        .context("a column has no heightmaps")?;
    let longs = |heights: &ColumnHeights| {
        Some(
            heights
                .raw_longs()
                .iter()
                .map(|&long| long.cast_signed())
                .collect::<Vec<_>>(),
        )
    };
    let block_entities = snapshot
        .block_entities
        .iter()
        .map(mcrs_minecraft_nbt::to_nbt_compound)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Chunk {
        pos,
        status: mcrs_minecraft_anvil::ChunkStatus::Full,
        is_light_on: false,
        inhabited_time: 0,
        last_update: 0,
        heightmaps: Heightmaps {
            world_surface: longs(&maps.surface.0),
            ocean_floor: longs(&maps.solid.0),
            motion_blocking: longs(&maps.motion.0),
            motion_blocking_no_leaves: longs(&maps.no_leaves.0),
            ..Heightmaps::default()
        },
        block_entities,
        retrogen: None,
        post_processing: Vec::new(),
        upgrade_data: Default::default(),
        blending_data: None,
        sections,
        fluid_ticks: Vec::new(),
        block_ticks: Vec::new(),
        structures: Default::default(),
        decode_report: Default::default(),
    })
}

pub fn write_world(
    data: &GameData,
    columns: &BTreeMap<ColumnPos, FilledSnapshot>,
    world: &Path,
) -> anyhow::Result<()> {
    let directory = world.join("region");
    fs::create_dir_all(&directory).with_context(|| directory.display().to_string())?;
    let bottom = data.min_y >> 4;
    let rows = bottom..bottom + (data.height >> 4).cast_signed();
    let mut regions: BTreeMap<(i32, i32), Vec<(ColumnPos, &FilledSnapshot)>> = BTreeMap::new();
    for (pos, snapshot) in columns {
        let at = RegionPos::from(*pos);
        regions
            .entry((at.x, at.z))
            .or_default()
            .push((*pos, snapshot));
    }
    for ((x, z), members) in regions {
        let path = directory.join(format!("r.{x}.{z}.mca"));
        let mut names = Names {
            blocks: PaletteNames::new(),
            biomes: PaletteNames::new(),
            block_ids: HashMap::new(),
            biome_ids: HashMap::new(),
        };
        let mut records = Vec::with_capacity(members.len());
        for (pos, snapshot) in members {
            let chunk = chunk_of(data, &mut names, pos, snapshot, &rows)
                .with_context(|| format!("column {},{}", pos.x, pos.z))?;
            let bytes = write_chunk(&chunk, rows.start, &names.blocks, &names.biomes)
                .with_context(|| format!("column {},{}", pos.x, pos.z))?;
            records.push((pos, bytes));
        }
        let mut region = if path.is_file() {
            RegionFile::open(&path)?
        } else {
            RegionFile::create(RegionPos { x, z }, &path)
        };
        region.write(&records, &path)?;
    }
    Ok(())
}

pub fn write_climate(data: &GameData, grid: &ClimateGrid, file: &Path) -> anyhow::Result<()> {
    let Some(router) = data.fill.router.as_ref() else {
        bail!("the dimension has no noise router, so it has no climate");
    };
    let indices = [TEMPERATURE, VEGETATION, CONTINENTS, EROSION, DEPTH, RIDGES];
    let volume = SampleGrid::new(
        IVec3::new(grid.nx, 1, grid.nz),
        IVec3::new(grid.x0, router.sea_level, grid.z0),
        IVec3::splat(grid.step),
    );
    let points = volume.len();
    let mut values = vec![0.0f32; indices.len() * points];
    router.fill_roots(&mut Workspace::new(), &volume, &indices, &mut values);
    let mut rows = values.chunks(points).map(|row| {
        row.iter()
            .map(|value| (f64::from(*value) * 1e4).round() / 1e4)
            .collect::<Vec<f64>>()
    });
    let mut next = || rows.next().unwrap_or_default();
    let document = ClimateFile {
        x0: grid.x0,
        z0: grid.z0,
        step: grid.step,
        nx: grid.nx,
        nz: grid.nz,
        signals: Signals {
            temperature: next(),
            vegetation: next(),
            continentalness: next(),
            erosion: next(),
            depth: next(),
            weirdness: next(),
        },
    };
    let text = serde_json::to_string(&document)?;
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).with_context(|| parent.display().to_string())?;
    }
    fs::write(file, text).with_context(|| file.display().to_string())
}
