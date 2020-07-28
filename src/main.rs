mod godaddy;
use godaddy::*;

fn main() {
    let authority =
        Authority::new("", "secret", api_url::PRODUCTION);

    authority.update_subdoomain("test", RecordType::A)


}
