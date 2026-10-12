//! Maps point-based VectorCraft cameras to physical-pixel shared views.
use crate::VectorcraftApp;
use artcraft_canvas::Camera2d;

/// A host-composited display frame in document coordinates. This is only a
/// presentation cache: the native vector document remains the editable source.
pub struct Presentation {
    pub native_document: u64,
    pub texture: egui::TextureHandle,
    pub bounds: vectorcraft_geom::Rect,
}

pub(crate) fn paint(app: &VectorcraftApp, painter: &egui::Painter, xf: &crate::canvas::Xf) -> bool {
    let Some(frame) = app.shared_presentation.as_ref().filter(|f| app.session.active().is_some_and(|s| s.uid == f.native_document)) else {
        return false;
    };
    let mut mesh = egui::Mesh::with_texture(frame.texture.id());
    for (pos, uv) in xf.quad(frame.bounds).into_iter().zip([egui::pos2(0.0, 0.0), egui::pos2(1.0, 0.0), egui::pos2(1.0, 1.0), egui::pos2(0.0, 1.0)]) {
        mesh.vertices.push(egui::epaint::Vertex { pos, uv, color: egui::Color32::WHITE });
    }
    mesh.add_triangle(0, 1, 2);
    mesh.add_triangle(0, 2, 3);
    painter.add(egui::Shape::mesh(mesh));
    true
}

pub(crate) fn enter(app: &mut VectorcraftApp, ppp: f32) {
    if app.shared_canvas.is_none() || !ppp.is_finite() || ppp <= 0.0 {
        return;
    }
    app.sync_views();
    let Some(id) = app.session.active().map(|d| d.uid) else { return };
    let Some(camera) = app.shared_canvas.as_mut().and_then(|b| b.initial_camera(id)) else { return };
    if let Some(view) = app.view_mut() {
        view.center = vectorcraft_geom::Point::new(camera.center[0], camera.center[1]);
        view.zoom = camera.zoom / f64::from(ppp);
        view.rotation = camera.rotation_degrees;
        view.fitted = true;
    }
}

pub(crate) fn leave(app: &VectorcraftApp, ppp: f32) {
    let Some(binding) = &app.shared_canvas else { return };
    let Some(id) = app.session.active().map(|d| d.uid) else { return };
    if let Some(view) = app.view() {
        binding.publish(id, Camera2d { center: [view.center.x, view.center.y], zoom: view.zoom * f64::from(ppp), rotation_degrees: view.rotation });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn physical_zoom_survives_dpi_and_foreign_tabs_are_not_modified() {
        let mut session = vectorcraft_engine::Session::new();
        session.add_document(vectorcraft_doc::Document::new(100.0, 80.0), None);
        let id = session.active().unwrap().uid;
        let mut app = VectorcraftApp::new(session, Default::default());
        let shared = artcraft_canvas::SharedView::new(1, 1);
        let camera = Camera2d { center: [30.0, 25.0], zoom: 2.0, rotation_degrees: 17.0 };
        assert!(shared.bind(999).publish(999, camera));
        app.shared_canvas = Some(shared.bind(id));
        enter(&mut app, 2.0);
        assert_eq!(app.view().unwrap().zoom, 1.0);
        leave(&app, 2.0);
        assert_eq!(shared.camera(), Some(camera));
        app.session.add_document(vectorcraft_doc::Document::new(10.0, 10.0), None);
        app.sync_views();
        let before = *app.view().unwrap();
        enter(&mut app, 2.0);
        assert_eq!(app.view().unwrap().center, before.center);
        leave(&app, 2.0);
        assert_eq!(shared.camera(), Some(camera));
    }
}
