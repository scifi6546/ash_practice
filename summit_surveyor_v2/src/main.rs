#![allow(clippy::nonstandard_macro_braces)]
mod camera;
mod gui;
mod model;
mod skiier;
mod terrain;
use camera::{Camera, Transform};
use gui::EventCollector;
use legion::*;
use model::{RenderingCtx, ScreenPlane};
use std::{cell::RefCell, f32, rc::Rc, time::Duration};
use sukakpak::{
    image::{Rgba, RgbaImage},
    nalgebra::{Vector2, Vector3},
    Context, Event, Sukakpak,
};
use terrain::{InsertableTerrain, Terrain};
struct Game {
    world: World,
    resources: Resources,
    game_render_surface: ScreenPlane,
}
pub mod prelude {
    pub use super::camera::{Camera, Transform};
    pub use super::model::{Model, RenderingCtx};
    pub use super::terrain::{GraphLayer, Terrain};
}

impl sukakpak::Renderable for Game {
    fn init(context: Rc<RefCell<Context>>) -> Self {
        context
            .borrow_mut()
            .load_shader("./shaders/test", "world")
            .expect("failed to load");
        context
            .borrow_mut()
            .load_shader("./shaders/gui_shader", "gui_shader")
            .expect("failed to load gui shader");
        let mut resources = Resources::default();
        let mut world = World::default();
        context
            .borrow_mut()
            .bind_shader(&sukakpak::BoundFramebuffer::ScreenFramebuffer, "gui_shader")
            .expect("failed to bind");
        Terrain::new_cone(Vector2::new(100, 100), Vector2::new(50.0, 50.0), -1.0, 50.0)
            .insert(&mut world, &mut resources, &context)
            .expect("failed to build terrain");
        model::insert_cube(
            Transform::default()
                .set_scale(Vector3::new(0.2, 0.2, 0.2))
                .translate(Vector3::new(0.0, 0.0, 1.0)),
            &mut world,
            context.clone(),
        )
        .expect("failed to insert");
        let default_tex = context
            .borrow_mut()
            .build_texture(&RgbaImage::from_pixel(
                100,
                100,
                Rgba::from([100, 100, 100, 255]),
            ))
            .expect("failed to build default texture");
        let hover_tex = context
            .borrow_mut()
            .build_texture(&RgbaImage::from_pixel(
                100,
                100,
                Rgba::from([0, 80, 80, 255]),
            ))
            .expect("failed to build default texture");

        let click_tex = context
            .borrow_mut()
            .build_texture(&RgbaImage::from_pixel(
                100,
                100,
                Rgba::from([0, 80, 80, 255]),
            ))
            .expect("failed to build default texture");

        gui::GuiComponent::insert(
            Box::new(
                gui::VerticalContainer::new(
                    vec![
                        Box::new(
                            gui::GuiSquare::new(
                                Transform::default().set_scale(Vector3::new(0.2, 0.1, 1.0)),
                                default_tex,
                                hover_tex,
                                click_tex,
                                context.clone(),
                            )
                            .expect("failed to build square"),
                        ),
                        Box::new(
                            gui::GuiSquare::new(
                                Transform::default().set_scale(Vector3::new(0.1, 0.1, 1.0)),
                                default_tex,
                                hover_tex,
                                click_tex,
                                context.clone(),
                            )
                            .expect("failed to build square"),
                        ),
                        Box::new(
                            gui::VerticalContainer::new(
                                vec![
                                    Box::new(
                                        gui::GuiSquare::new(
                                            Transform::default()
                                                .set_scale(Vector3::new(0.2, 0.1, 1.0)),
                                            default_tex,
                                            hover_tex,
                                            click_tex,
                                            context.clone(),
                                        )
                                        .expect("failed to build square"),
                                    ),
                                    Box::new(
                                        gui::GuiSquare::new(
                                            Transform::default()
                                                .set_scale(Vector3::new(0.1, 0.1, 1.0)),
                                            default_tex,
                                            hover_tex,
                                            click_tex,
                                            context.clone(),
                                        )
                                        .expect("failed to build square"),
                                    ),
                                ],
                                gui::VerticalContainerStyle {
                                    alignment: gui::ContainerAlignment::Center,
                                    padding: 0.01,
                                },
                                Vector3::new(0.0, 0.0, -0.6),
                                context.clone(),
                            )
                            .expect("failed to create vertical container"),
                        ),
                        Box::new(gui::TextLabel::new(
                "hello world, Here is a loooong paragraph, do you like reading really really really long paragraphs? You know the ones that go on an on forever so long you wonder why the person is still writing. I do so here is one of those loooooong ones."
                    .to_string(),
                            0.003,
                            Transform::default().set_scale(Vector3::new(0.5, 1.0, 1.0)),
                            context.clone(),
                        )),
                    ],
                    gui::VerticalContainerStyle {
                        alignment: gui::ContainerAlignment::Center,
                        padding: 0.01,
                    },
                    Vector3::new(0.0, 0.0, 0.5),
                    context.clone(),
                )
                .expect("failed to build vertical container"),
            ),
            &mut world,
        )
        .expect("failed to insert?");

        println!("*******************\nBuilding Raw Text\n***************");
        gui::GuiComponent::insert(
            Box::new(gui::TextLabel::new(
                "hello world, Here is a loooong paragraph, do you like reading really really really long paragraphs? You know the ones that go on an on forever so long you wonder why the person is still writing. I do so here is one of those loooooong ones."
                    .to_string(),
                0.006,
                Transform::default()
                    .set_scale(Vector3::new(2.0, 1.0, 1.0))
                    .translate(Vector3::new(0.0, 0.0, 0.0)),
                context.clone(),
            )),
            &mut world,
        )
        .expect("failed to insert");

        resources.insert(RenderingCtx::new(&context));
        resources.insert(
            Camera::default()
                .set_translation(Vector3::new(0.0, 2.0, 0.0))
                .set_yaw(f32::consts::PI / 2.0),
        );
        resources.insert(EventCollector::default());
        let game_render_surface = model::build_screen_plane(context, Vector2::new(1000, 1000), 0.0)
            .expect("faled to create render surface");
        Self {
            world,
            resources,
            game_render_surface,
        }
    }
    fn render_frame(
        &mut self,
        events: &[Event],
        context: Rc<RefCell<Context>>,
        delta_time: Duration,
    ) {
        self.process_events(delta_time, events);

        context
            .borrow_mut()
            .bind_framebuffer(&sukakpak::BoundFramebuffer::UserFramebuffer(
                self.game_render_surface.framebuffer,
            ))
            .expect("failed to bind");
        let mut game_renderng_schedule = Schedule::builder()
            .add_system(skiier::skiier_system())
            .add_system(gui::event::send_events_system())
            .add_system(gui::react_events_system())
            .add_system(model::render_model_system())
            .add_system(terrain_camera_system())
            .build();
        game_renderng_schedule.execute(&mut self.world, &mut self.resources);
        context
            .borrow_mut()
            .bind_framebuffer(&sukakpak::BoundFramebuffer::ScreenFramebuffer)
            .expect("failed to bind");
        context
            .borrow_mut()
            .draw_mesh(
                Transform::default()
                    .set_translation(Vector3::new(0.0, 0.0, 0.9))
                    .to_bytes(),
                &self.game_render_surface.mesh,
            )
            .expect("failed to draw screen surface");
        let mut gui_rendering_schedule = Schedule::builder()
            .add_system(gui::render_gui_component_system())
            .build();
        gui_rendering_schedule.execute(&mut self.world, &mut self.resources);
        self.resources.get_mut::<EventCollector>().unwrap().clear();
    }
}
impl Game {
    pub fn process_events(&mut self, delta_time: Duration, events: &[Event]) {
        self.resources
            .get_mut::<EventCollector>()
            .expect("failed to get event collector")
            .process_events(delta_time, events);
    }
}
fn main() {
    Sukakpak::new::<Game>(sukakpak::CreateInfo {
        default_size: Vector2::new(1000, 1000),
        name: "Summit Surveyor".to_string(),
    });
}
#[system(for_each)]
pub fn terrain_camera(
    terrain: &InsertableTerrain,
    #[resource] events: &mut EventCollector,
    #[resource] camera: &mut Camera,
) {
    if events.keycodes_down.contains(&30) {
        *camera = camera.clone().translate(Vector3::new(-0.01, 0.0, 0.0))
    }

    if events.keycodes_down.contains(&32) {
        *camera = camera.clone().translate(Vector3::new(0.01, 0.0, 0.0))
    }

    if events.keycodes_down.contains(&31) {
        *camera = camera.clone().translate(Vector3::new(0.0, 0.0, -0.01))
    }
    if events.keycodes_down.contains(&17) {
        *camera = camera.clone().translate(Vector3::new(0.0, 0.0, 0.01))
    }
    if events.left_mouse_down {
        *camera.yaw() += events.mouse_delta_pos.x * events.delta_time.as_secs_f32() * 1000.0;
        *camera.pitch() += events.mouse_delta_pos.y * events.delta_time.as_secs_f32() * 1000.0;
    }
}
