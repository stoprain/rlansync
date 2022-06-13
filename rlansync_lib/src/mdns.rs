
    // println!("in_msg {:?}", in_msg);
    // let content = match in_msg.details {
    //     MessageField(test) => println!("FileInfoRequest {:?}", test),
    //     MessageField(FileDataRequest) => println!("FileDataRequest"),
    // };
    // assert_eq!(in_msg.from, 12345);
    // println!("content {:?}", content);

fn setup_mdns() {

    // pub extern "C" fn notify(from: *const c_char, callback: SwiftObject) {
        // thread::spawn(move || {
        //     thread::sleep(Duration::from_secs(3));
        //     callback.succeeded()
        // });
    
    //     let mut ip = String::new();
    //     for iface in get_if_addrs::get_if_addrs().unwrap() {
    //         if iface.name == "en0" {
    //             match iface.addr {
    //                 V4(v) => {
    //                     ip = v.ip.to_string().to_owned()
    //                 },
    //                 V6(v) => {},
    //             }
    //         }
    //         // println!("{:#?}", iface);
    //     }
    
    //     // Create a daemon
    //     let mdns = ServiceDaemon::new().expect("Failed to create daemon");
    
    
    // /*
    // dns-sd -B _services._dns-sd._udp
    // dns-sd -B _rlan-sync._udp
    // dns-sd -L "rains-macb" _rlan-sync._udp
    // dns-sd -L "rains-ipho" _rlan-sync._udp
    
    // sudo killall -HUP mDNSResponder;
    // */
    
    //     // Create a service info.
    //     let service_type = "_rlan._tcp.local.";
    
    //     //receiver
    //     // let mdns1 = ServiceDaemon::new().expect("Failed to create daemon");
    //     let receiver = mdns.browse(service_type).expect("Failed to browse");
    //     // Receive the browse events in sync or async. Here is
    //     // an example of using a thread. Users can call `receiver.recv_async().await`
    //     // if running in async environment.
    //     std::thread::spawn(move || {
    //         while let Ok(event) = receiver.recv() {
    //             match event {
    //                 ServiceEvent::ServiceResolved(info) => {
    //                     println!("Resolved a new service: {}, {:?}, {}", info.get_fullname(), info.get_addresses(), info.get_port());
    //                 }
    //                 other_event => {
    //                     println!("Received other event: {:?}", &other_event);
    //                 }
    //             }
    //         }
    //     });
    
    //     //publish
    
    
    //     let ss = gethostname().into_string().unwrap().to_lowercase();
    //     let instance_name = ss.substring(0, 10);
    //     // let instance_name = "my_instance";
    
    //     let s = ip.clone() + ".local.";
    //     let host_ipv4 = ip.as_str();
    //     let host_name = s.as_str();
    //     let port = 8888;
    //     let mut properties = HashMap::new();
    //     properties.insert("property_1".to_string(), "test".to_string());
    //     properties.insert("property_2".to_string(), "1234".to_string());
    
    //     println!("from {:?}", from);
    //     println!("instance_name {:?}", instance_name);
    //     println!("host_name {:?}", host_name);
    
    //     let my_service = ServiceInfo::new(
    //         service_type,
    //         &instance_name,
    //         host_name,
    //         host_ipv4,
    //         port,
    //         Some(properties),
    //     ).unwrap();
    
    //     // Register with the daemon, which publishes the service.
    //     mdns.register(my_service).expect("Failed to register our service");
    
        // setup_mdns();
    }