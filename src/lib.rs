pub mod karabo_hash;
pub mod binary_writers;
pub mod binary_readers;
pub mod web_socket;

#[cfg(test)]
mod tests;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
