fn main() {
    windows::build! {
        Windows::Management::Deployment::PackageManager,
        Windows::Storage::StorageFolder,
    }
}
