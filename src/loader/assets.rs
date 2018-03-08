use sdl2::ttf::{Font, Sdl2TtfContext};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path.clone(),
            size: details.size,
        }
    }
}

pub struct ResourceManager<'l, K, R, L> where K: Hash + Eq, L: 'l + ResourceLoader<'l, R> {
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {

    type Args = FontDetails;

    fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
        self.load_font(&details.path, details.size)
    }
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L> where K: Hash + Eq, L: ResourceLoader<'l, R> {

    pub fn new(loader: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String> where L: ResourceLoader<'l, R, Args = D>, D: Eq + Hash + ?Sized, K: Borrow<D> + for<'a> From<&'a D> {
        self.cache
            .get(details)
            .cloned()
            .map_or_else(|| {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            }, Ok)
    }
}

pub type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;
