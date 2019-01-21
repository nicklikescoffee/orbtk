use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, HashSet},
    rc::Rc,
};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{
    application::Tree,
    backend::Backend,
    enums::Visibility,
    properties::{Bounds, Point},
    render_object::RenderObject,
    theme::Selector,
    widget::Context,
};

/// The `RenderSystem` iterates over all visual widgets and used its render objects to draw them on the screen.
pub struct RenderSystem {
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub backend: Rc<RefCell<dyn Backend>>,
    pub update: Rc<Cell<bool>>,
    pub debug_flag: Rc<Cell<bool>>,
}

impl System<Tree> for RenderSystem {
    fn run(&self, tree: &Tree, ecm: &mut EntityComponentManager) {
        if !self.update.get() || tree.parent.is_empty() {
            return;
        }

        let mut backend = self.backend.borrow_mut();
        let render_context = backend.render_context();

        let mut hidden_parents: HashSet<Entity> = HashSet::new();

        let mut offsets = BTreeMap::new();
        offsets.insert(tree.root, (0, 0));

        // render window background
        render_context
            .renderer
            .render(render_context.theme.color("background", &"window".into()));

        for node in tree.into_iter() {
            let mut global_position = Point::default();

            if let Some(offset) = offsets.get(&tree.parent[&node]) {
                global_position = Point::new(offset.0, offset.1);
            }

            // Hide all children of a hidden parent
            if hidden_parents.contains(&tree.parent[&node]) {
                hidden_parents.insert(node);
                continue;
            }

            // render debug border for each widget
            if self.debug_flag.get() {
                if let Ok(bounds) = ecm.borrow_component::<Bounds>(node) {
                    if let Ok(parent_bounds) = ecm.borrow_component::<Bounds>(tree.parent[&node]) {
                        let selector = Selector::from("debugborder");

                        render_context.renderer.render_rectangle(
                            bounds,
                            parent_bounds,
                            &global_position,
                            render_context.theme.uint("border-radius", &selector),
                            render_context.theme.color("background", &selector),
                            render_context.theme.uint("border-width", &selector),
                            render_context.theme.color("border-color", &selector),
                            render_context.theme.float("opcaity", &selector),
                        );
                    }
                }
            }

            // hide hidden widget
            if let Ok(visibility) = ecm.borrow_component::<Visibility>(node) {
                if *visibility != Visibility::Visible {
                    hidden_parents.insert(node);
                    continue;
                }
            }

            if let Some(render_object) = self.render_objects.borrow().get(&node) {
                render_object.render(
                    render_context.renderer,
                    &mut Context::new(
                        node,
                        ecm,
                        tree,
                        &render_context.event_queue,
                        &render_context.theme,
                        None,
                    ),
                    &global_position,
                );
            }

            let mut global_pos = (0, 0);

            if let Ok(bounds) = ecm.borrow_component::<Bounds>(node) {
                global_pos = (global_position.x + bounds.x, global_position.y + bounds.y);
                offsets.insert(node, global_pos);
            }

            if let Ok(g_pos) = ecm.borrow_mut_component::<Point>(node) {
                g_pos.x = global_pos.0;
                g_pos.y = global_pos.1;
            }
        }
    }
}
