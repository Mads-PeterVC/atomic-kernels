use ak_core::{Structure, Trajectory};
use ak_vis::viewer::{
    BallAndStickStyle, BondList, BondScope, Face, FaceList, HeadlessRenderConfig, RenderStyle,
    ViewerConfig, export_image_with_session,
};

fn main() {
    let output = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "headless-scene.png".to_string());

    let structure = Structure::new(
        vec![
            [0.0, 0.0, 0.0],
            [1.1, 1.1, 1.1],
            [1.1, -1.1, -1.1],
            [-1.1, 1.1, -1.1],
            [-1.1, -1.1, 1.1],
        ],
        vec![22, 8, 8, 8, 8],
        [[12.0, 0.0, 0.0], [0.0, 12.0, 0.0], [0.0, 0.0, 12.0]],
        [false, false, false],
    );
    let trajectory = Trajectory::new(vec![structure]);
    let config = ViewerConfig::default();
    let export = HeadlessRenderConfig::new(output.clone(), 960, 720);

    export_image_with_session(trajectory, config, export, |session| {
        session
            .set_bonds(BondList::new([(0, 1), (0, 2), (0, 3), (0, 4)]), Some(0))
            .expect("set bonds");
        session
            .set_faces(
                FaceList::new([
                    Face::new([1, 2, 3], [0.13, 0.52, 0.78, 0.34]).unwrap(),
                    Face::new([1, 4, 2], [0.18, 0.65, 0.66, 0.30]).unwrap(),
                    Face::new([1, 3, 4], [0.83, 0.56, 0.16, 0.28]).unwrap(),
                    Face::new([2, 4, 3], [0.73, 0.31, 0.22, 0.30]).unwrap(),
                ]),
                Some(0),
            )
            .expect("set faces");
        session
            .set_render_style(
                RenderStyle::BallAndStick(BallAndStickStyle {
                    atom_scale: 0.55,
                    bond_radius: 0.06,
                    bond_color: [0.55, 0.55, 0.55, 1.0],
                    bond_scope: BondScope::TouchSelection,
                }),
                vec![true, true, true, true, true],
                Some(0),
                false,
            )
            .expect("set render style");
        session.frame_all().expect("frame all");
        session
            .set_camera_view(None, None, Some(0.55), Some(0.35))
            .expect("set camera");
    })
    .expect("headless render");

    println!("saved {output}");
}
