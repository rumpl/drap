use drap::bundle::store;

fn main() {
    let bundles = store::list_bundles();
    for bundle in &bundles {
        println!("{}", bundle.name)
    }
}
