fn main() {
    let handle1 = std::thread::spawn(thread1);
    let handle2 = std::thread::spawn(thread2);
    handle1.join().unwrap();
    handle2.join().unwrap();
}

windows::include_bindings!();

fn inner_wmi() {
    let com_con = wmi::COMLibrary::without_security().unwrap();
    let wmi_con = wmi::WMIConnection::new(com_con.into()).unwrap();
    let results = wmi_con
        .exec_query_native_wrapper("SELECT OSArchitecture FROM Win32_OperatingSystem").unwrap()
        .collect::<Vec<_>>();
}

fn inner_wrt() {
    let before = std::time::Instant::now();
    let pm = Windows::Management::Deployment::PackageManager::new().ok().unwrap();
    let iter = pm.FindPackages().ok().unwrap();
    for package in iter {
        let path = package.InstalledLocation().and_then(|v| v.Path());
        //println!("{:?}", path);
    }
    let after = std::time::Instant::now();
    println!("Time: {:?}", after - before);
}

fn thread1() {
    inner_wmi();
    inner_wrt();
}

fn thread2() {
    inner_wmi();
}
