use super::prelude::{Model, Transform};
use legion::*;
use std::{cell::RefCell, rc::Rc, sync::Mutex};
use sukakpak::{
    anyhow::Result,
    image::{Rgba, RgbaImage},
    nalgebra::Vector2,
    Context,
};
pub struct Grid<T> {
    data: Vec<T>,
    dimensions: Vector2<usize>,
}
impl<T> Grid<T> {
    pub fn from_fn<F: Fn(usize, usize) -> T>(f: F, dimensions: Vector2<usize>) -> Self {
        let mut data = vec![];
        data.reserve(dimensions.x * dimensions.y);
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                data.push(f(x, y));
            }
        }
        Self { dimensions, data }
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        assert!(x < self.dimensions.x);
        assert!(y < self.dimensions.y);
        &self.data[x * self.dimensions.y + y]
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        assert!(x < self.dimensions.x);
        assert!(y < self.dimensions.y);
        &mut self.data[x * self.dimensions.y + y]
    }
}
pub struct InsertableTerrain {}
pub struct Terrain {
    heights: Grid<f32>,

    dimensions: Vector2<usize>,
}
impl Terrain {
    pub fn new_flat(dimensions: Vector2<usize>) -> Self {
        let heights = Grid::from_fn(|_, _| 0.0, dimensions);
        Self {
            heights,
            dimensions,
        }
    }
    pub fn new_cone(
        dimensions: Vector2<usize>,
        center: Vector2<f32>,
        slope: f32,
        center_height: f32,
    ) -> Self {
        let mut heights = vec![];
        heights.reserve(dimensions.x * dimensions.y);
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                let radius = ((x as f32 - center.x).powi(2) + (y as f32 - center.y).powi(2)).sqrt();
                let height = center_height + radius * slope;
                heights.push(height);
            }
        }
        let heights = Grid::from_fn(
            move |x, y| {
                let radius = ((x as f32 - center.x).powi(2) + (y as f32 - center.y).powi(2)).sqrt();
                center_height + radius * slope
            },
            dimensions,
        );
        Self {
            heights,
            dimensions,
        }
    }
    pub fn insert(
        &self,
        world: &mut World,
        resources: &mut Resources,
        context: &Rc<RefCell<Context>>,
    ) -> Result<()> {
        let graph_layer: Box<dyn GraphLayer> = self.into();
        let mut layers = resources
            .get_mut_or_insert::<Vec<Mutex<Box<dyn GraphLayer>>>>((vec![Mutex::new(self.into())]));
        layers.push(Mutex::new(graph_layer));
        let texture = context.borrow_mut().build_texture(&RgbaImage::from_pixel(
            100,
            100,
            Rgba::from([200, 200, 200, 200]),
        ))?;
        let mut vertices = vec![];
        let mut indices = vec![];
        let mut idx = 0;
        for x in 0..self.dimensions.x - 1 {
            for y in 0..self.dimensions.y - 1 {
                //Vertex 0:
                //position
                vertices.push(x as f32);
                vertices.push(self.get(x, y));
                vertices.push(y as f32);

                //texture coord
                vertices.push(0.0);
                vertices.push(0.0);
                //normal
                vertices.push(0.0);
                vertices.push(1.0);
                vertices.push(0.0);

                //Vertex 1

                //position
                vertices.push((x + 1) as f32);
                vertices.push(self.get(x + 1, y));
                vertices.push(y as f32);

                //texture coord
                vertices.push(1.0);
                vertices.push(0.0);
                //normal
                vertices.push(0.0);
                vertices.push(1.0);
                vertices.push(0.0);
                //Vertex2

                //position
                vertices.push((x) as f32);
                vertices.push(self.get(y + 1, x));
                vertices.push((y + 1) as f32);

                //texture coord
                vertices.push(0.0);
                vertices.push(1.0);
                //normal
                vertices.push(0.0);
                vertices.push(1.0);
                vertices.push(0.0);
                //Vertex3

                //position
                vertices.push((x + 1) as f32);
                vertices.push(self.get(y + 1, x + 1));
                vertices.push((y + 1) as f32);

                //texture coord
                vertices.push(1.0);
                vertices.push(1.0);
                //normal
                vertices.push(0.0);
                vertices.push(1.0);
                vertices.push(0.0);
                indices.push(idx);
                indices.push(idx + 2);
                indices.push(idx + 1);

                indices.push(idx + 2);
                indices.push(idx + 3);
                indices.push(idx + 1);
                idx += 4;
            }
        }
        let mesh = sukakpak::MeshAsset {
            indices,
            vertices: vertices.iter().map(|f| f.to_ne_bytes()).flatten().collect(),
            vertex_layout: sukakpak::VertexLayout {
                components: vec![
                    sukakpak::VertexComponent::Vec3F32,
                    sukakpak::VertexComponent::Vec2F32,
                    sukakpak::VertexComponent::Vec3F32,
                ],
            },
        };

        let model = Model::new(context.borrow_mut().build_mesh(mesh, texture));
        world.push((InsertableTerrain {}, Transform::default(), model, texture));
        Ok(())
    }
    fn get(&self, x: usize, y: usize) -> f32 {
        *self.heights.get(x, y)
    }
}
struct TerrainWeight(f32);
pub struct TerrainGraphLayer {
    grid: Grid<TerrainWeight>,
}
impl From<&Terrain> for TerrainGraphLayer {
    fn from(t: &Terrain) -> Self {
        let mut data = vec![];
        data.reserve(t.heights.dimensions.x * t.heights.dimensions.y);
        for x in 0..t.heights.dimensions.x {
            for y in 0..t.heights.dimensions.y {
                data.push(TerrainWeight(*t.heights.get(x, y)));
            }
        }
        Self {
            grid: Grid {
                data,
                dimensions: t.heights.dimensions,
            },
        }
    }
}
impl From<&Terrain> for Box<dyn GraphLayer> {
    fn from(t: &Terrain) -> Self {
        let layer: TerrainGraphLayer = t.into();
        Box::new(layer)
    }
}
impl GraphLayer for TerrainGraphLayer {
    fn get_children(&self, point: &GraphNode) -> Vec<(GraphWeight, GraphWeight)> {
        todo!()
    }
    fn get_distance(&self, start_point: &GraphNode, end_point: &GraphNode) -> GraphWeight {
        todo!()
    }
}

pub struct GraphNode(pub Vector2<usize>);
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum GraphWeight {
    Some(i32),
    Infinity,
}
impl GraphWeight {
    pub fn is_finite(&self) -> bool {
        match self {
            GraphWeight::Some(_) => true,
            GraphWeight::Infinity => false,
        }
    }
}
impl std::fmt::Display for GraphWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Infinity => write!(f, "Infinity"),
            Self::Some(v) => write!(f, "Some({})", v),
        }
    }
}
impl std::ops::Add for GraphWeight {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Self::Some(num) => match other {
                Self::Some(other_num) => Self::Some(num + other_num),
                Self::Infinity => Self::Infinity,
            },
            Self::Infinity => Self::Infinity,
        }
    }
}
impl std::cmp::PartialOrd for GraphWeight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for GraphWeight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Infinity => match other {
                Self::Infinity => std::cmp::Ordering::Equal,
                Self::Some(_) => std::cmp::Ordering::Greater,
            },
            Self::Some(s) => match other {
                Self::Infinity => std::cmp::Ordering::Less,
                Self::Some(o) => s.cmp(o),
            },
        }
    }
}
impl std::iter::Sum for GraphWeight {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(GraphWeight::Some(0), |acc, x| acc + x)
    }
}
pub trait GraphLayer: Send {
    /// Gets nodes connected to a node on a graph
    fn get_children(&self, point: &GraphNode) -> Vec<(GraphWeight, GraphWeight)>;
    /// gets weight connecting two points. If points are not connecte infinity is
    /// returned
    fn get_distance(&self, start_point: &GraphNode, end_point: &GraphNode) -> GraphWeight;
}
