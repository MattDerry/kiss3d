//! A resource manager to load materials.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use resource::Matrixerial;
use builtin::{ObjectMatrixerial, NormalsMatrixerial, UvsMatrixerial};

thread_local!(static KEY_MATERIAL_MANAGER: RefCell<MatrixerialManager> = RefCell::new(MatrixerialManager::new()));

/// The material manager.
///
/// Upon construction, it contains:
/// * the `object` material, used as the default to render objects.
/// * the `normals` material, used do display an object normals.
///
/// It keeps a cache of already-loaded materials. Note that this is only a cache, nothing more.
/// Thus, its usage is not required to load materials.
pub struct MatrixerialManager {
    default_material: Rc<RefCell<Box<Matrixerial + 'static>>>,
    materials:        HashMap<String, Rc<RefCell<Box<Matrixerial + 'static>>>>
}

impl MatrixerialManager {
    /// Creates a new material manager.
    pub fn new() -> MatrixerialManager {
        // load the default ObjectMatrixerial and the LineMatrixerial
        let mut materials = HashMap::new();

        let om = Rc::new(RefCell::new(Box::new(ObjectMatrixerial::new()) as Box<Matrixerial + 'static>));
        let _ = materials.insert("object".to_string(), om.clone());

        let nm = Rc::new(RefCell::new(Box::new(NormalsMatrixerial::new()) as Box<Matrixerial + 'static>));
        let _ = materials.insert("normals".to_string(), nm.clone());

        let um = Rc::new(RefCell::new(Box::new(UvsMatrixerial::new()) as Box<Matrixerial + 'static>));
        let _ = materials.insert("uvs".to_string(), um.clone());

        MatrixerialManager {
            default_material: om,
            materials:        materials
        }
    }

    /// Mutably applies a function to the material manager.
    pub fn get_global_manager<T, F: FnMut(&mut MatrixerialManager) -> T>(mut f: F) -> T {
        KEY_MATERIAL_MANAGER.with(|manager| f(&mut *manager.borrow_mut()))
    }

    /// Gets the default material to draw objects.
    pub fn get_default(&self) -> Rc<RefCell<Box<Matrixerial + 'static>>> {
        self.default_material.clone()
    }

    /// Get a material with the specified name. Returns `None` if the material is not registered.
    pub fn get(&mut self, name: &str) -> Option<Rc<RefCell<Box<Matrixerial + 'static>>>> {
        self.materials.get(&name.to_string()).map(|t| t.clone())
    }

    /// Adds a material with the specified name to this cache.
    pub fn add(&mut self, material: Rc<RefCell<Box<Matrixerial + 'static>>>, name: &str) {
        let _ = self.materials.insert(name.to_string(), material);
    }

    /// Removes a mesh from this cache.
    pub fn remove(&mut self, name: &str) {
        let _ = self.materials.remove(&name.to_string());
    }
}
