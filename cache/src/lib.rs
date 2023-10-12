mod cache_trait;
pub use cache_trait::Cache;

mod btreemap_cache;
mod my_cache1;
mod my_cache2;
mod my_cache3;
mod my_cache4;

pub use btreemap_cache::BTreeCache;
pub use my_cache1::MyCache1;
pub use my_cache2::MyCache2;
pub use my_cache3::MyCache3;
pub use my_cache4::MyCache4;
