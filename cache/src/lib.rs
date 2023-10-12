mod cache_trait;
pub use cache_trait::Cache;

mod btreemap_cache;
mod my_cache1;
mod my_cache2;
mod my_cache3;
mod my_cache4;
mod deny_left_recursion_cache;

pub use deny_left_recursion_cache::DenyLeftRecursionCache;
pub use btreemap_cache::BTreeCache;
pub use my_cache1::MyCache1;
pub use my_cache2::MyCache2;
pub use my_cache3::MyCache3;
pub use my_cache4::MyCache4;


// Still need to try a flatpacked lru cache that doesn't use pointer indexing and instead pushes to a stack and then just reads the last N items
// May be faster due to better cache coherency, may be slower due to needing to scan don't know until I try.