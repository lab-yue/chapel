pub mod chapel;

pub mod request;
pub mod response;

#[cfg(test)]
mod tests {

    #[test]
    fn test_listen() {
        let app = super::chapel::Chapel{};
        app.listen(4000)
    }
}
