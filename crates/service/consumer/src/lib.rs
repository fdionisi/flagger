mod consumer {
    tonic::include_proto!("consumer");
}
mod consumer_service;
mod context;
mod server;

pub use server::ConsumerServer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
