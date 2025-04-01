pub mod config;
pub mod error;
pub mod esa;
pub mod handlers;
#[cfg(not(test))]
pub mod server;
pub mod vertex_ai;
pub mod webhook;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
